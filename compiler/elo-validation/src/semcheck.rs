use elo_ir::{ast::{self, TypedField}, cir::{Intrinsic, ExpressionIdentity}};
use elo_error::semerror::*;
use elo_ir::cir;
use elo_lexer::span::Span;
use std::collections::HashMap;

pub struct Namespace {
    pub name: Option<String>,
    pub constants: HashMap<String, (Span, cir::Typing)>,
    pub structs: HashMap<String, (Span, cir::Struct)>,
    pub enums: HashMap<String, (Span, cir::Enum)>,
    pub functions: HashMap<String, (Span, cir::FunctionHead)>,
    pub locals: Vec<Scope>,
}

pub enum Inference {
    Equal,
    Cast,
    Invalid,
}

#[derive(Debug)]
pub struct Variable {
    pub mutable: bool,
    pub typing: cir::Typing,
}

pub type Scope = HashMap<String, Variable>;

type TypedExpression = (cir::Expression, cir::Typing);

pub struct SemanticChecker {
    namespace: Namespace,
    current_function: String,
    pub errors: Vec<SemanticError>,
}

impl SemanticChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            current_function: String::new(),
            namespace: Namespace {
                name: None,
                structs: HashMap::new(),
                enums: HashMap::new(),
                constants: HashMap::new(),
                functions: HashMap::new(),
                locals: Vec::new(),
            },
        }
    }

    fn check_type(&mut self, typ: &ast::Type) -> Result<cir::Typing, SemanticError> {
        match &typ.typing {
            // TODO: Add generics
            ast::Typing::Named { name, .. } => {
                if let Some(t) = cir::Primitive::from_str(name) {
                    return Ok(cir::Typing::Primitive(t));
                } else if let Some((_, e)) = self.namespace.enums.get(name) {
                    return Ok(cir::Typing::Enum(e.clone()));
                } else if let Some((_, e)) = self.namespace.structs.get(name) {
                    return Ok(cir::Typing::Struct(e.clone()));
                }
                return Err(SemanticError {
                    span: typ.span,
                    case: SemanticErrorCase::UnresolvedName {
                        name: name.clone(),
                    },
                });
            }
            ast::Typing::Pointer { typ, mutable } => {
                let inner_typing = self.check_type(typ)?;
                return Ok(cir::Typing::Pointer {
                    mutable: *mutable,
                    typ: Box::new(inner_typing),
                });
            }
            ast::Typing::Tuple { types } => {
                let mut checked_types = Vec::new();
                for t in types {
                    checked_types.push(self.check_type(t)?);
                }
                return Ok(cir::Typing::Tuple { types: checked_types })
            }
            ast::Typing::Function { args, ret } => {
                let mut checked_types = Vec::new();
                for t in args {
                    checked_types.push(self.check_type(t)?);
                }
                let mut checked_ret = cir::Typing::Void;
                if let Some(t) = ret {
                    checked_ret = self.check_type(t)?;
                }
                return Ok(cir::Typing::Function {
                    ret: Box::new(checked_ret),
                    arguments: checked_types,
                    variadic: false,
                    extrn: false
                });
            }
            ast::Typing::Array { typ, amount } => {
                return Ok(cir::Typing::Array {
                    typ: Box::new(self.check_type(typ)?),
                    amount: *amount
                });
            }
        }
    }

    fn typecheck_binop(
        &mut self,
        lhs: TypedExpression,
        rhs: TypedExpression,
        binop: &ast::BinaryOperation,
        span: Span,
    ) -> Result<TypedExpression, SemanticError> {
        let ir_binop = cir::BinaryOperation::from_ast(&binop);
        let rhs_inferred = self.make_inference(rhs.0, &rhs.1, &lhs.1);

        if rhs_inferred.is_none() {
            return Err(SemanticError {
                span: span,
                case: SemanticErrorCase::TypeMismatch {
                    got: format!("{}", rhs.1),
                    expected: format!("{}", lhs.1),
                },
            });
        }

        let typing = match ir_binop {
            cir::BinaryOperation::Add
            | cir::BinaryOperation::Sub
            | cir::BinaryOperation::Mul
            | cir::BinaryOperation::Div
            | cir::BinaryOperation::Mod
            | cir::BinaryOperation::BAnd
            | cir::BinaryOperation::BOr
            | cir::BinaryOperation::BXor
            | cir::BinaryOperation::LShift
            | cir::BinaryOperation::RShift => lhs.1,
            cir::BinaryOperation::Eq
            | cir::BinaryOperation::Ne
            | cir::BinaryOperation::Lt
            | cir::BinaryOperation::Le
            | cir::BinaryOperation::Gt
            | cir::BinaryOperation::Ge
            | cir::BinaryOperation::And
            | cir::BinaryOperation::Or => cir::Typing::Primitive(cir::Primitive::Bool),
            cir::BinaryOperation::Assign
            | cir::BinaryOperation::AssignAdd
            | cir::BinaryOperation::AssignSub
            | cir::BinaryOperation::AssignMul
            | cir::BinaryOperation::AssignDiv
            | cir::BinaryOperation::AssignMod
            | cir::BinaryOperation::AssignBAnd
            | cir::BinaryOperation::AssignBOr
            | cir::BinaryOperation::AssignBXor => {
                match lhs.0.identity {
                    ExpressionIdentity::Locatable(false) => {
                        return Err(SemanticError {
                            span: span,
                            case: SemanticErrorCase::AssignImmutable {
                                expression: format!("{}", lhs.0),
                            },
                        });
                    }
                    ExpressionIdentity::Immediate => {
                        return Err(SemanticError {
                            span: span,
                            case: SemanticErrorCase::InvalidExpression {
                                what: format!("{}", lhs.0),
                                should: "valid left-hand-side operand".to_string(),
                            },
                        });
                    }
                    _ => {} // Ok! For assignment, the lhs must be a mutable locatable value
                }
                cir::Typing::Void
            }
        };

        let expr = cir::Expression {
            span,
            data: cir::ExpressionData::BinaryOperation {
                operator: ir_binop,
                left    : Box::new(lhs.0),
                right   : Box::new(rhs_inferred.unwrap())
            },
            identity: ExpressionIdentity::Immediate
        };

        Ok((expr, typing))
    }

    fn typecheck_intrinsic_call(
        &mut self,
        intrinsic: cir::Intrinsic,
        arguments: &Vec<ast::Expression>,
        call_span: Span,
    ) -> Result<TypedExpression, SemanticError> {
        let signature = match intrinsic {
            cir::Intrinsic::Print => {
                (cir::Typing::Void,
                 vec![cir::Typing::Primitive(cir::Primitive::Str)])
            }
        };

        let passed_length = arguments.len();
        let expected_len = signature.1.len();
        if passed_length < expected_len {
            return Err(SemanticError {
                span: call_span,
                case: SemanticErrorCase::UnmatchedArguments {
                    function: format!("{intrinsic}"),
                    got: passed_length,
                    expected: expected_len,
                    too_much: false,
                },
            });
        } else if passed_length > expected_len {
            return Err(SemanticError {
                span: call_span,
                case: SemanticErrorCase::UnmatchedArguments {
                    function: format!("{intrinsic}"),
                    got: passed_length,
                    expected: expected_len,
                    too_much: true,
                },
            });
        }

        let mut checked_arguments = Vec::new();
        for (expression, expected_type) in arguments.iter().zip(signature.1.clone()) {
            let (checked, got_type) = self.typecheck_expr(expression, false)?;
            if let Some(checked) = self.make_inference(checked, &got_type, &expected_type) {
                checked_arguments.push(checked);
            } else {
                return Err(SemanticError {
                    span: expression.span,
                    case: SemanticErrorCase::TypeMismatch {
                        got: format!("{}", got_type),
                        expected: format!("{}", expected_type),
                    },
                });
            }
        }

        return Ok((
            cir::Expression {
                span: call_span,
                data: cir::ExpressionData::IntrinsicCall {
                    intrinsic,
                    arguments: checked_arguments,
                },
                identity: ExpressionIdentity::Immediate,
            },
            signature.0,
        ));
    }

    fn typecheck_function_call(
        &mut self,
        expr: cir::Expression,
        ret: cir::Typing,
        arguments: &Vec<cir::Typing>,
        variadic: bool,
        caller_arguments: &Vec<ast::Expression>,
        call_span: Span,
    ) -> Result<TypedExpression, SemanticError> {
        let return_type = ret;
        let passed_length = caller_arguments.len();
        let expected_len = arguments.len();
        if passed_length < expected_len {
            return Err(SemanticError {
                span: call_span,
                case: SemanticErrorCase::UnmatchedArguments {
                    function: format!("{expr}"),
                    got: passed_length,
                    expected: expected_len,
                    too_much: false,
                },
            });
        } else if (passed_length > expected_len) && !variadic {
            return Err(SemanticError {
                span: call_span,
                case: SemanticErrorCase::UnmatchedArguments {
                    function: format!("{expr}"),
                    got: passed_length,
                    expected: expected_len,
                    too_much: true,
                },
            });
        }
        let mut checked_arguments = Vec::new();
        let iter = caller_arguments.into_iter().zip(arguments.clone());
        for (expression, expected_type) in iter {
            let (checked, got_type) = self.typecheck_expr(expression, false)?;
            if let Some(checked) = self.make_inference(checked, &got_type, &expected_type) {
                checked_arguments.push(checked);
            } else {
                return Err(SemanticError {
                    span: expression.span,
                    case: SemanticErrorCase::TypeMismatch {
                        got: format!("{}", got_type),
                        expected: format!("{}", expected_type),
                    },
                });
            }
        }

        // get the remaining extra arguments if the fn is variadic
        for extra in caller_arguments.iter().skip(expected_len) {
            // remaining if the function is variadic
            let (extra, _) = self.typecheck_expr(extra, false)?;
            checked_arguments.push(extra);
        }

        return Ok((
            cir::Expression {
                span: call_span,
                data: cir::ExpressionData::FunctionCall {
                    function: Box::new(expr),
                    arguments: checked_arguments,
                },
                identity: ExpressionIdentity::Immediate,
            },
            return_type,
        ));
    }

    fn auto_dereference(&self, expression: TypedExpression) -> TypedExpression {
        let (mut expr, mut typ) = expression;
        let span = expr.span;
        while let cir::Typing::Pointer { typ: inner, mutable } = typ {
            expr = cir::Expression {
                span,
                data: cir::ExpressionData::UnaryOperation {
                    operator: cir::UnaryOperation::Deref,
                    operand: Box::new(expr)
                },
                identity: ExpressionIdentity::Locatable(mutable),
            };
            typ = *inner;
        }
        (expr, typ)
    }

    // Make the changes in the expression so the inference is possible
    fn make_inference(&self, expression: cir::Expression, from: &cir::Typing, into: &cir::Typing) -> Option<cir::Expression> {
        let inf = self.typecheck_inference(from, into);
        let id = expression.identity;

        match inf {
            Inference::Invalid => None,
            Inference::Cast => Some(cir::Expression {
                span: expression.span,
                data: cir::ExpressionData::Cast {
                    expr: Box::new(expression),
                    typ: into.clone(),
                },
                identity: id,
            }),
            Inference::Equal => Some(expression),
        }
    }

    // Implicit type cast checking
    fn typecheck_inference(&self, from: &cir::Typing, into: &cir::Typing) -> Inference {
        if from == into {
            return Inference::Equal;
        }

        let x = from.get_size();
        let y = into.get_size();

        let mut cast = false;
        if from.is_unsigned() {
            cast = ((into.is_signed() || into.is_float()) && y >= 2*x) || (into.is_unsigned() && y >= x);
        }
        if from.is_signed() {
            cast = (into.is_float() || into.is_signed()) && y >= x;
        }
        if let (
            &cir::Typing::Primitive(cir::Primitive::Char),
            &cir::Typing::Primitive(cir::Primitive::U32)
        ) = (&from, &into) {
            cast = true;
        }
        if let (
            &cir::Typing::Primitive(cir::Primitive::U8),
            &cir::Typing::Primitive(cir::Primitive::Char)
        ) = (&from, &into) {
            cast = true;
        }
        if let (
            &cir::Typing::Pointer { mutable: true, .. },
            &cir::Typing::Pointer { mutable: false, .. },
        ) = (&from, &into) {
            cast = true;
        }

        if cast {
            return Inference::Cast;
        }

        return Inference::Invalid;
    }

    // Explicit type cast rules
    fn typecheck_cast(&mut self, origin: &cir::Typing, into: &cir::Typing, span: Span) -> Result<(), SemanticError> {
        let mut ok = false;

        if let Inference::Cast | Inference::Equal = self.typecheck_inference(&origin, &into) {
            return Ok(());
        }

        if origin.is_integer() {
            ok = into.is_bool() || into.is_integer() || into.is_float();
        } else if origin.is_float() {
            ok = into.is_bool() || into.is_integer() || into.is_float();
        } else if origin.is_bool() {
            ok = into.is_integer();
        } else if let (
            &cir::Typing::Primitive(cir::Primitive::Char),
            &cir::Typing::Primitive(cir::Primitive::U8)
        ) = (&origin, &into) {
            ok = true;
        }

        if !ok {
            return Err(SemanticError {
                span: span,
                case: SemanticErrorCase::InvalidCast {
                    from: format!("{}", origin),
                    into: format!("{}", into),
                }
            });
        }
        return Ok(())
    }

    fn typecheck_expr(&mut self, expr: &ast::Expression, function_call: bool) -> Result<TypedExpression, SemanticError> {
        match &expr.data {
            ast::ExpressionData::Cast { expr: inner, typ } => {
                let typ = self.check_type(typ)?;
                let (inner, origin) = self.typecheck_expr(inner, function_call)?;
                let id = inner.identity;
                self.typecheck_cast(&origin, &typ, expr.span)?;
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Cast {
                            expr: Box::new(inner),
                            typ: typ.clone(),
                        },
                        identity: id,
                    },
                    typ,
                ))
            }
            ast::ExpressionData::BinaryOperation {
                operator,
                left,
                right,
            } => {
                let lhs = self.typecheck_expr(left, function_call)?;
                let rhs = self.typecheck_expr(right, function_call)?;
                Ok(self.typecheck_binop(lhs, rhs, operator, expr.span)?)
            }
            ast::ExpressionData::UnaryOperation { operator, operand } => {
                let operator = cir::UnaryOperation::from_ast(operator);
                let (operand, operand_type) = self.typecheck_expr(&operand, function_call)?;
                let operand_id = operand.identity;
                let operation_type;
                let id;
                match operator {
                    cir::UnaryOperation::Addr => {
                        if let ExpressionIdentity::Locatable(mutable) = operand_id {
                            operation_type = cir::Typing::Pointer {
                                mutable,
                                typ: Box::new(operand_type),
                            };
                            id = ExpressionIdentity::Immediate;
                        } else {
                            return Err(SemanticError {
                                span: expr.span,
                                case: SemanticErrorCase::InvalidExpression {
                                    what: format!("{}", operand),
                                    should: "valid value to reference".to_string(),
                                },
                            });
                        }
                    }
                    cir::UnaryOperation::Neg => {
                        if operand_type.is_integer() || operand_type.is_float() {
                            operation_type = if let Some(signed) = operand_type.get_signed() {
                                signed
                            } else {
                                operand_type
                            };
                            id = ExpressionIdentity::Immediate;
                        } else {
                            return Err(SemanticError {
                                span: expr.span,
                                case: SemanticErrorCase::TypeMismatch {
                                    got: format!("{}", operand_type),
                                    expected: "integer or floating-point".to_string(),
                                },
                            });
                        }
                    }
                    cir::UnaryOperation::Not
                    | cir::UnaryOperation::BNot => {
                        operation_type = operand_type;
                        id = ExpressionIdentity::Immediate;
                    }
                    cir::UnaryOperation::Deref => match operand_id {
                        ExpressionIdentity::Immediate => {
                            return Err(SemanticError {
                                span: expr.span,
                                case: SemanticErrorCase::InvalidExpression {
                                    what: format!("{}", operand),
                                    should: "valid value to dereference".to_string(),
                                },
                            });
                        }
                        _ => {
                            if let cir::Typing::Pointer { typ, mutable } = operand_type {
                                operation_type = *typ;
                                id = ExpressionIdentity::Locatable(mutable);
                            } else {
                                return Err(SemanticError {
                                    span: expr.span,
                                    case: SemanticErrorCase::TypeMismatch {
                                        got: format!("{}", operand_type),
                                        expected: "pointer".to_string(),
                                    },
                                });
                            }
                        }
                    },
                };
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::UnaryOperation {
                            operator,
                            operand: Box::new(operand),
                        },
                        identity: id,
                    },
                    operation_type,
                ))
            }
            ast::ExpressionData::CharacterLiteral { value } => {
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Integer {
                            value: *value as i128,
                        },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Primitive(cir::Primitive::Char),
                ));
            }
            ast::ExpressionData::StrLiteral { value } => {
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::StringLiteral {
                            value: value.clone(),
                        },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Primitive(cir::Primitive::Str),
                ));
            }
            ast::ExpressionData::Tuple { exprs } => {
                let mut validated_exprs = Vec::new();
                let mut types = Vec::new();
                for expr in exprs {
                    let (e, t) = self.typecheck_expr(expr, function_call)?;
                    validated_exprs.push(e);
                    types.push(t);
                };
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Tuple {
                            exprs: validated_exprs,
                            types: types.clone(),
                        },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Tuple { types },
                ));
            }
            ast::ExpressionData::Array { exprs, amount } => {
                let mut checked_exprs = Vec::new();
                let mut r#type: Option<cir::Typing> = None;
                for i in exprs {
                    let (mut expr, expr_typing) = self.typecheck_expr(i, function_call)?;
                    let span = i.span;
                    if let Some(ref expected) = r#type {
                        if let Some(inferred) = self.make_inference(expr, &expr_typing, &expected) {
                            expr = inferred;
                        } else {
                            return Err(SemanticError {
                                span: span,
                                case: SemanticErrorCase::TypeMismatch {
                                    got: format!("{}", expr_typing),
                                    expected: format!("{}", expected),
                                },
                            });
                        }
                    } else {
                        r#type = Some(expr_typing);
                    }
                    checked_exprs.push(expr);
                }
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::ArrayLiteral {
                            exprs: checked_exprs,
                            typ: r#type.clone().unwrap(),
                        },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Array {
                        typ: Box::new(r#type.unwrap()),
                        amount: *amount,
                    },
                ));
            }
            ast::ExpressionData::TupleAccess { origin, field } => {
                let (tuple, typ) = self.typecheck_expr(origin, function_call)?;
                let id = tuple.identity;
                if let cir::Typing::Tuple { ref types } = typ {
                    if *field >= types.len() {
                        return Err(SemanticError {
                            span: expr.span,
                            case: SemanticErrorCase::InvalidTupleMember {
                                member: *field,
                                tuple: format!("{}", &typ),
                                member_count: types.len(),
                            }
                        });
                    }
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::TupleAccess {
                                origin: Box::new(tuple),
                                field: *field,
                            },
                            identity: id,
                        },
                        types.get(*field).unwrap().clone(),
                    ));
                } else {
                    return Err(SemanticError {
                        span: tuple.span,
                        case: SemanticErrorCase::NonTupleMemberAccess {
                            thing: format!("{}", tuple),
                            typ: format!("{}", typ)
                        }
                    })
                }
            }
            ast::ExpressionData::Subscript { origin, inner } => {
                let (origin, origin_type) = self.typecheck_expr(origin, function_call)?;
                let origin_id = origin.identity;
                let (inner, inner_type) = self.typecheck_expr(inner, function_call)?;
                let inner_span = inner.span;
                if let cir::Typing::Array { typ, .. } = origin_type {
                    if let Some(inner) = self.make_inference(inner, &inner_type, &cir::Typing::Primitive(cir::Primitive::UInt)) {
                        return Ok((
                            cir::Expression {
                                span: expr.span,
                                data: cir::ExpressionData::ArraySubscript {
                                    origin: Box::new(origin),
                                    index: Box::new(inner)
                                },
                                identity: origin_id,
                            },
                            *typ,
                        ))
                    } else {
                        return Err(SemanticError {
                            span: inner_span,
                            case: SemanticErrorCase::TypeMismatch {
                                got: format!("{}", inner_type),
                                expected: format!("unsigned integer")
                            }
                        })
                    }
                } else {
                    return Err(SemanticError {
                        span: origin.span,
                        case: SemanticErrorCase::IndexNonIndexable {
                            thing: format!("{origin}"),
                            got: format!("{origin_type}"),
                        }
                    })
                }
            }
            ast::ExpressionData::FieldAccess { origin, field } => {
                if let ast::ExpressionData::Identifier { name } = &origin.data {
                    if let Some((_, e)) = self.namespace.enums.get(name) {
                        let mut found = false;
                        for i in e.variants.iter() {
                            if i == field {
                                found = true;
                            }
                        }
                        if !found {
                            return Err(SemanticError {
                                span: origin.span,
                                case: SemanticErrorCase::UnknownEnumVariant {
                                    enumeration: e.name.clone(),
                                    variant: field.clone(),
                                }
                            })
                        }
                        return Ok((
                            cir::Expression {
                                span: expr.span,
                                data: cir::ExpressionData::EnumVariant {
                                    origin: e.name.clone(),
                                    variant: field.clone()
                                },
                                identity: ExpressionIdentity::Immediate,
                            },
                            cir::Typing::Enum(e.clone()),
                        ));
                    }
                }

                let meta = self.typecheck_expr(origin, function_call)?;
                let (expression, typing) = self.auto_dereference(meta);
                let id = expression.identity;
                match typing {
                    cir::Typing::Struct(st) => {
                        // the case when you are getting a field from struct instance
                        // search for field
                        let mut typ = None; // return type of the whole expression
                        for (f, t) in st.fields {
                            if field == &f {
                                typ = Some(t);
                            }
                        }
                        if let None = typ {
                            return Err(SemanticError {
                                span: expr.span,
                                case: SemanticErrorCase::UnresolvedField {
                                    name: format!("{field}"),
                                    from: format!("struct {}", st.name),
                                },
                            });
                        }
                        return Ok((
                            cir::Expression {
                                span: expr.span,
                                data: cir::ExpressionData::FieldAccess {
                                    origin: Box::new(expression),
                                    field: field.clone(),
                                },
                                identity: id,
                            },
                            typ.unwrap(),
                        ));
                    }
                    _ => {
                        return Err(SemanticError {
                            span: origin.span,
                            case: SemanticErrorCase::NonAggregateFieldAccess {
                                typ: format!("{}", typing),
                                field: field.clone(),
                            },
                        });
                    }
                }
            }
            ast::ExpressionData::FunctionCall {
                function,
                arguments: caller_arguments,
            } => {
                let (function, function_type) = self.typecheck_expr(function, true)?;
                let span = function.span;
                if let cir::Typing::Function { ret, arguments, variadic, extrn: _ } = function_type {
                    return self.typecheck_function_call(
                        function,
                        *ret,
                        &arguments,
                        variadic,
                        caller_arguments,
                        span,
                    );
                } else if let cir::Typing::Intrinsic(intrinsic) = function_type {
                    return self.typecheck_intrinsic_call(intrinsic, caller_arguments, span);
                } else {
                    return Err(SemanticError {
                        span: expr.span,
                        case: SemanticErrorCase::CallNonFunction {
                            typ: format!("{function_type}"),
                        },
                    });
                }
            }
            ast::ExpressionData::StructInit { name, fields } => {
                let span = expr.span;
                let (_, strukt) = self
                    .namespace
                    .structs
                    .get(name)
                    .ok_or_else(|| SemanticError {
                        span: span,
                        case: SemanticErrorCase::UnresolvedName {
                            name: format!("{}", &name),
                        },
                    })?
                    .clone();
                let mut checked_fields = Vec::new();
                for field in fields {
                    let expected_typing =
                        strukt.fields.get(&field.name).ok_or_else(|| SemanticError {
                            span: span,
                            case: SemanticErrorCase::UnresolvedField {
                                name: format!("{}", &field.name),
                                from: format!("struct {}", &strukt.name),
                            },
                        })?;
                    let field_value_span = field.value.span;
                    let (expr, typing) = self.typecheck_expr(&field.value, function_call)?;
                    if let Some(expr) = self.make_inference(expr, &typing, expected_typing) {
                        checked_fields.push((field.name.clone(), expr));
                    } else {
                        return Err(SemanticError {
                            span: field_value_span,
                            case: SemanticErrorCase::TypeMismatch {
                                got: format!("{}", typing),
                                expected: format!("{}", expected_typing),
                            },
                        });
                    }
                }
                let thing = cir::ExpressionData::StructInit {
                    origin: strukt.clone(),
                    fields: checked_fields,
                };
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: thing,
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Struct(strukt),
                ))
            }
            ast::ExpressionData::IntegerLiteral { value } => {
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Integer {
                            value: *value,
                        },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Primitive(cir::Primitive::UInt),
                ))
            }
            ast::ExpressionData::FloatLiteral { value } => {
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Float { value: *value },
                        identity: ExpressionIdentity::Immediate,
                    },
                    cir::Typing::Primitive(cir::Primitive::Float),
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => Ok((
                cir::Expression {
                    span: expr.span,
                    data: cir::ExpressionData::Bool { value: *value },
                    identity: ExpressionIdentity::Immediate,
                },
                cir::Typing::Primitive(cir::Primitive::Bool),
            )),
            ast::ExpressionData::Identifier { name } => {
                if let Some((_, t)) = self.namespace.constants.get(name) {
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::Identifier { name: name.clone() },
                            identity: ExpressionIdentity::Immediate,
                        },
                        t.clone()
                    ));
                } else if let Some((_, f)) = self.namespace.functions.get(name) {
                    let args = f.arguments.iter().map(|(_, typ)| typ.clone()).collect::<Vec<cir::Typing>>();
                    if f.extrn && !function_call {
                        return Err(SemanticError {
                            span: expr.span,
                            case: SemanticErrorCase::UseExternFnAsExpr { name: f.name.to_string() }
                        });
                    }
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::Identifier { name: name.clone() },
                            identity: ExpressionIdentity::Function(f.extrn),
                        },
                        cir::Typing::Function {
                            ret: Box::new(f.ret.clone()),
                            arguments: args,
                            variadic: f.variadic,
                            extrn: f.extrn,
                        }
                    ));
                } else if let Some(i) = Intrinsic::from_str(name) {
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::Identifier { name: name.clone() },
                            identity: ExpressionIdentity::Function(false),
                        },
                        cir::Typing::Intrinsic(i)
                    ));
                } else {
                    // Iterate the local namespace in reverse (from the most recent scope to the oldest)
                    // to find the variable.
                    // This is because the most recent scope should take precedence.
                    // If the variable is not found, return an error.
                    for i in self.namespace.locals.iter().rev() {
                        if let Some(var) = i.get(name) {
                            return Ok((
                                cir::Expression {
                                    span: expr.span,
                                    data: cir::ExpressionData::Identifier { name: name.clone() },
                                    identity: ExpressionIdentity::Locatable(var.mutable),
                                },
                                var.typing.clone()
                            ));
                        }
                    }
                    return Err(SemanticError {
                        span: expr.span,
                        case: SemanticErrorCase::UnresolvedName { name: name.clone() },
                    });
                }
            }
        }
    }

    fn typecheck_block(
        &mut self,
        block: Vec<ast::Node>,
        expects_return: Option<&cir::Typing>
    ) -> Result<Vec<cir::Statement>, SemanticError> {
        let mut blk = Vec::new();
        // Create a new scope for the function
        self.namespace.locals.push(HashMap::new());
        for a in Box::new(block).into_iter() {
            match self.typecheck_node(a, expects_return) {
                Ok(x) => blk.push(x),
                Err(e) => {
                    self.namespace.locals.pop();
                    return Err(e);
                }
            }
        }
        self.namespace.locals.pop();
        Ok(blk)
    }

    fn typecheck_function_block(
        &mut self,
        block: Vec<ast::Node>,
        return_type: &cir::Typing,
        function_name: &str,
        function_arguments: HashMap<String, Variable>,
    ) -> Result<Vec<cir::Statement>, SemanticError> {
        self.current_function = function_name.to_string();
        let mut blk = Vec::new();

        // Create a new scope for the function
        let mut scope = HashMap::new();
        scope.extend(function_arguments);
        self.namespace.locals.push(scope);

        for a in Box::new(block).into_iter() {
            match self.typecheck_node(a, Some(return_type)) {
                Ok(x) => blk.push(x),
                Err(e) => {
                    self.namespace.locals.pop();
                    return Err(e);
                }
            }
        }

        self.namespace.locals.pop();
        Ok(blk)
    }

    fn controlcheck_inner_function_block(&mut self, span: Span, block: &Vec<cir::Statement>, is_top_level: bool, function_name: &str, return_type: &cir::Typing) -> Result<(bool, Span), SemanticError> {
        let mut last_span = span;
        for i in block {
            last_span = i.span;
            match &i.kind {
                cir::StatementKind::ReturnStatement { .. } => {
                    return Ok((true, i.span));
                }
                cir::StatementKind::IfStatement { block_true, block_false, .. } => {
                    let (a, s1) = self.controlcheck_inner_function_block(last_span, block_true, false, function_name, return_type)?;
                    let (b, s2) = self.controlcheck_inner_function_block(last_span, block_false, false, function_name, return_type)?;
                    if a && b {
                        return Ok((true, s2));
                    }
                    last_span = if a { s2 } else { s1 };
                }
                // NOTE: In this case there's no reason to check for while loop,
                //       since it always reaches the end and continues execution after its condition becomes false
                _ => {}
            }
        }
        if is_top_level && return_type != &cir::Typing::Void {
            return Err(SemanticError { span: last_span, case: SemanticErrorCase::NoReturn {
                function: function_name.to_string(),
                returns: format!("{}", return_type)
            }});
        }
        Ok((false, last_span))
    }

    fn controlcheck_function_block(
        &mut self,
        span: Span,
        block: &Vec<cir::Statement>,
        function_name: &str,
        return_type: &cir::Typing,
    ) -> Result<(), SemanticError> {
        self.controlcheck_inner_function_block(span, block, true, function_name, return_type)?;
        Ok(())
    }

    // If there is a name, return the span of the definition
    fn check_name_availability(&self, name: &str) -> Option<Span> {
        if let Some((span, _)) = self.namespace.functions.get(name) {
            return Some(*span)
        } else if let Some((span, _)) = self.namespace.enums.get(name) {
            return Some(*span)
        } else if let Some((span, _)) = self.namespace.structs.get(name) {
            return Some(*span)
        } else if let Some((span, _)) = self.namespace.constants.get(name) {
            return Some(*span)
        }
        None
    }

    fn typecheck_node(&mut self, node: ast::Node, expects_return: Option<&cir::Typing>) -> Result<cir::Statement, SemanticError> {
        match node.stmt {
            ast::Statement::LetStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;

                for i in self.namespace.locals.iter().rev() {
                    if i.get(name).is_some() {
                        return Err(SemanticError {
                            span: node.span,
                            case: SemanticErrorCase::VariableRedefinition { name: name.clone() }
                        });
                    }
                }

                let (expr, typ) = self.typecheck_expr(assignment, false)?;

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().insert(
                    name.clone(),
                    Variable {
                        mutable: false,
                        typing: typ.clone(),
                    },
                );
                Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::Variable {
                            assignment: expr,
                            binding: name.clone(),
                            typing: typ,
                        }
                    }
                )
            }
            ast::Statement::VarStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.typecheck_expr(assignment, false)?;

                for i in self.namespace.locals.iter().rev() {
                    if i.get(name).is_some() {
                        return Err(SemanticError {
                            span: node.span,
                            case: SemanticErrorCase::VariableRedefinition { name: name.clone() }
                        });
                    }
                }

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().insert(
                    name.clone(),
                    Variable {
                        mutable: true,
                        typing: typ.clone(),
                    },
                );
                Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::Variable {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    }
                })
            }
            ast::Statement::ConstStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.typecheck_expr(assignment, false)?;
                let annotated = self.check_type(&stmt.typing)?;
                self.namespace.constants.insert(name.clone(), (node.span, typ.clone()));
                if let Some(expr) = self.make_inference(expr, &typ, &annotated) {
                    return Ok(cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::Constant {
                            value: expr,
                            binding: name.clone(),
                            typing: typ,
                        }
                    })
                } else {
                    return Err(SemanticError {
                        span: stmt.typing.span,
                        case: SemanticErrorCase::TypeMismatch {
                            got: format!("{}", typ),
                            expected: format!("{}", annotated),
                        },
                    });
                }
            }
            ast::Statement::ReturnStatement(stmt) => {
                if expects_return.is_none() && stmt.expr.is_some() {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::MisplacedReturn })
                }
                if let Some(expr) = &stmt.expr {
                    let (expr, typ) = self.typecheck_expr(expr, false)?;
                    let got_return = &typ;
                    let expected_return = expects_return.unwrap();
                    if let Some(expr) = self.make_inference(expr, got_return, expected_return) {
                        return Ok(cir::Statement {
                            span: node.span,
                            kind: cir::StatementKind::ReturnStatement {
                                value: Some(expr),
                                typing: typ,
                            }
                        });
                    } else {
                        if expected_return == &cir::Typing::Void {
                            return Err(SemanticError { span: node.span, case: SemanticErrorCase::ReturnValueOnVoidFunction {
                                function: self.current_function.clone(),
                            }})
                        }
                        return Err(SemanticError { span: node.span, case: SemanticErrorCase::MismatchedReturnType {
                            function: self.current_function.clone(),
                            got: format!("{}", typ),
                            expected: format!("{}", expected_return),
                        }});
                    }
                }
                Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::ReturnStatement {
                        value: None,
                        typing: cir::Typing::Void,
                    }
                })
            }
            ast::Statement::FnStatement(stmt) => {
                if let Some(s) = self.check_name_availability(&stmt.name) {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::NameRedefinition {
                        name: stmt.name.clone(),
                        defined: s
                    }})
                }

                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push((a.name.clone(), self.check_type(&a.typing)?));
                }

                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.check_type(ret_type)?,
                    None => cir::Typing::Void,
                };

                // Add the arguments to the scope
                let mut arguments = HashMap::new();
                for arg in validated_args.iter() {
                    let (name, typing) = arg;
                    let mut mutable = false;
                    if let cir::Typing::Pointer { mutable: true, .. } = typing {
                        mutable = true;
                    }
                    arguments.insert(
                        name.clone(),
                        Variable {
                            mutable,
                            typing: typing.clone(),
                        },
                    );
                }

                let head = cir::FunctionHead {
                    name: stmt.name.clone(),
                    ret: validated_ret_type.clone(),
                    arguments: validated_args,
                    variadic: false, // In this case, variadic is ALWAYS false
                                     // Because Elo is not meant to support variadic functions at all.
                    extrn: false     // The same for extrn which is meant to flag if this function should be mangled
                };

                // Insert the function into the namespace
                self.namespace.functions.insert(stmt.name.clone(), (node.span, head.clone()));

                let validated_block = self.typecheck_function_block(stmt.block.content, &validated_ret_type, &stmt.name, arguments)?;
                self.controlcheck_function_block(node.span, &validated_block, &stmt.name, &validated_ret_type)?;

                let validated = cir::Function {
                    head: head.clone(),
                    block: validated_block,
                };

                return Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::FnStatement(validated)
                });
            }
            ast::Statement::ExternFnStatement(stmt) => {
                if let Some(s) = self.check_name_availability(&stmt.name) {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::NameRedefinition {
                        name: stmt.name.clone(),
                        defined: s
                    }})
                }

                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push((a.name.clone(), self.check_type(&a.typing)?));
                }
                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.check_type(ret_type)?,
                    None => cir::Typing::Void,
                };
                let head = cir::FunctionHead {
                    name: stmt.name.clone(),
                    ret: validated_ret_type,
                    arguments: validated_args,
                    variadic: stmt.variadic,
                    extrn: true,
                };
                self.namespace.functions.insert(stmt.name.clone(), (node.span, head.clone()));
                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::ExternFnStatement(head)
                    }
                );
            }
            ast::Statement::StructStatement(stmt) => {
                if let Some(s) = self.check_name_availability(&stmt.name) {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::NameRedefinition {
                        name: stmt.name.clone(),
                        defined: s
                    }})
                }

                let mut fields = HashMap::new();
                for TypedField { name, typing } in &stmt.fields {
                    let checked_type = self.check_type(typing)?;
                    fields.insert(name.clone(), checked_type);
                }
                let e = cir::Struct {
                    name: stmt.name,
                    fields,
                };
                self.namespace.structs.insert(e.name.clone(), (node.span, e.clone()));
                return Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::StructStatement(e)
                });
            }
            ast::Statement::EnumStatement(stmt) => {
                if let Some(s) = self.check_name_availability(&stmt.name) {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::NameRedefinition {
                        name: stmt.name.clone(),
                        defined: s
                    }})
                }

                let e = cir::Enum {
                    name: stmt.name,
                    variants: stmt.variants,
                };
                self.namespace.enums.insert(e.name.clone(), (node.span, e.clone()));
                return Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::EnumStatement(e)
                });
            }
            ast::Statement::IfStatement(stmt) => {
                let (condition, typing) = self.typecheck_expr(&stmt.condition, false)?;
                if typing != cir::Typing::Primitive(cir::Primitive::Bool) {
                    return Err(SemanticError {
                        span: stmt.condition.span,
                        case: SemanticErrorCase::TypeMismatch {
                            got: format!("{}", typing),
                            expected: format!("{}", cir::Typing::Primitive(cir::Primitive::Bool)),
                        },
                    });
                }

                self.namespace.locals.push(HashMap::new());
                let block_true_content = self.typecheck_block(stmt.block_true.content, expects_return)?;
                self.namespace.locals.pop(); // Pop the true block scope

                self.namespace.locals.push(HashMap::new());
                let mut block_false_content = vec![];
                if let Some(block_false) = stmt.block_false {
                    block_false_content = self.typecheck_block(block_false.content, expects_return)?;
                }
                self.namespace.locals.pop(); // Pop the false block scope

                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::IfStatement {
                            condition,
                            block_true: block_true_content,
                            block_false: block_false_content,
                        }
                    }
                );
            }
            ast::Statement::WhileStatement(stmt) => {
                let (condition, typing) = self.typecheck_expr(&stmt.condition, false)?;
                if typing != cir::Typing::Primitive(cir::Primitive::Bool) {
                    return Err(SemanticError {
                        span: stmt.condition.span,
                        case: SemanticErrorCase::TypeMismatch {
                            got: format!("{}", typing),
                            expected: format!("{}", cir::Typing::Primitive(cir::Primitive::Bool)),
                        },
                    });
                }
                self.namespace.locals.push(HashMap::new());
                let block = self.typecheck_block(stmt.block.content, expects_return)?;
                self.namespace.locals.pop();
                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::WhileStatement { condition, block }
                    }
                );
            }
            ast::Statement::ExpressionStatement(stmt) => {
                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::ExpressionStatement(self.typecheck_expr(&stmt, false)?.0)
                    }
                );
            }
        }
    }

    pub fn typecheck_main_function(&mut self) -> Result<(), SemanticError> {
        let main = cir::FunctionHead {
            name: String::from("main"),
            ret: cir::Typing::Void,
            arguments: Vec::new(),
            variadic: false,
            extrn: false
        };

        if let Some((span, f)) = self.namespace.functions.get("main") {
            if &main == f {
                return Ok(());
            }
            return Err(SemanticError {
                span: *span,
                case: SemanticErrorCase::InvalidMainSignature {
                    should_be: format!("{}", main)
                }
            });
        }
        return Ok(())
    }

    // Type-check, control-flow check and transform the AST into the IR of Elo code
    pub fn go(&mut self, input: Vec<ast::Node>) -> cir::Program {
        // This is why i'm making a language
        let mut stmts = Vec::new();
        for node in Box::new(input).into_iter() {
            match self.typecheck_node(node, None) {
                Ok(s) => {
                    stmts.push(s)
                },
                Err(e) => self.errors.push(e),
            }
        }
        match self.typecheck_main_function() {
            Ok(()) => {},
            Err(e) => self.errors.push(e),
        }
        cir::Program { nodes: stmts }
    }
}
