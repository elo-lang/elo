use elo_ir::ast::{self, Expression, TypedField};
use elo_error::semerror::*;
use elo_ir::cir;
use elo_lexer::span::Span;
use std::collections::HashMap;

pub struct Namespace {
    pub name: Option<String>,
    pub constants: HashMap<String, cir::Typing>,
    pub structs: HashMap<String, cir::Struct>,
    pub enums: HashMap<String, cir::Enum>,
    pub functions: HashMap<String, cir::FunctionHead>,
    pub locals: Vec<Scope>,
}

#[derive(Debug)]
pub struct Variable {
    pub mutable: bool,
    pub typing: cir::Typing,
}

pub type Scope = HashMap<String, Variable>;

#[derive(Debug, PartialEq, Eq)]
// This enum is like an extended version of the concept of Lvalues and Rvalues in C/C++ terms
// read more
enum ExpressionIdentity {
    Locatable(bool), // bool: mutable
    Immediate,
}

impl std::fmt::Display for ExpressionIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionIdentity::Locatable(mutable) => write!(f, "{} locatable", if *mutable { "mutable" } else { "immutable" }),
            ExpressionIdentity::Immediate => write!(f, "immediate"),
        }
    }
}

type ExpressionMetadata = (cir::Expression, cir::Typing, ExpressionIdentity);

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
                } else if let Some(e) = self.namespace.enums.get(name) {
                    return Ok(cir::Typing::Enum(e.clone()));
                } else if let Some(e) = self.namespace.structs.get(name) {
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
            _ => todo!("implement other types"),
        }
    }

    fn typecheck_binop(
        &mut self,
        lhs: &ExpressionMetadata,
        rhs: &ExpressionMetadata,
        binop: &ast::BinaryOperation,
        span: elo_lexer::span::Span,
    ) -> Result<(cir::BinaryOperation, cir::Typing, ExpressionIdentity), SemanticError> {
        let ir_binop = cir::BinaryOperation::from_ast(&binop);
        if rhs.1 != lhs.1 {
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
            | cir::BinaryOperation::RShift => lhs.1.clone(),
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
                match lhs.2 {
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
        Ok((ir_binop, typing, ExpressionIdentity::Immediate))
    }

    fn typecheck_function_call(
        &mut self,
        name: &str,
        arguments: &Vec<Expression>,
        span: Span,
    ) -> Result<ExpressionMetadata, SemanticError> {
        let function = self.namespace.functions.get(name).ok_or(SemanticError {
            span: span,
            case: SemanticErrorCase::UnresolvedName {
                name: name.to_string(),
            },
        })?;
        let extrn = function.extrn;

        let return_type = function.ret.clone();
        let passed_length = arguments.len();
        let expected_len = function.arguments.len();
        if passed_length < expected_len {
            return Err(SemanticError {
                span: span,
                case: SemanticErrorCase::UnmatchedArguments {
                    name: name.to_string(),
                    got: passed_length,
                    expected: expected_len,
                    too_much: false,
                },
            });
        } else if (passed_length > expected_len) && !function.variadic {
            return Err(SemanticError {
                span: span,
                case: SemanticErrorCase::UnmatchedArguments {
                    name: name.to_string(),
                    got: passed_length,
                    expected: expected_len,
                    too_much: true,
                },
            });
        }
        let mut checked_arguments = Vec::new();
        let iter = arguments.iter().zip(function.arguments.clone());
        for (expression, (_, expected_type)) in iter {
            let (checked, got_type, _) = self.typecheck_expr(expression)?;
            if got_type != expected_type {
                return Err(SemanticError {
                    span: expression.span,
                    case: SemanticErrorCase::TypeMismatch {
                        got: format!("{}", got_type),
                        expected: format!("{}", expected_type),
                    },
                });
            }
            checked_arguments.push(checked);
        }


        // get the remaining extra arguments if the fn is variadic
        for extra in arguments.iter().skip(expected_len) {
            // remaining if the function is variadic
            let (extra, _, _) = self.typecheck_expr(extra)?;
            checked_arguments.push(extra);
        }

        return Ok((
            cir::Expression {
                span,
                data: cir::ExpressionData::FunctionCall {
                    function: name.to_string(),
                    arguments: checked_arguments,
                    extrn
                }
            },
            return_type,
            ExpressionIdentity::Immediate,
        ));
    }

    fn typecheck_expr(&mut self, expr: &ast::Expression) -> Result<ExpressionMetadata, SemanticError> {
        match &expr.data {
            ast::ExpressionData::BinaryOperation {
                operator,
                left,
                right,
            } => {
                let lhs = self.typecheck_expr(left)?;
                let rhs = self.typecheck_expr(right)?;
                let (operator, typing, op_id) =
                    self.typecheck_binop(&lhs, &rhs, operator, expr.span)?;
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::BinaryOperation {
                            operator,
                            left: Box::new(lhs.0),
                            right: Box::new(rhs.0),
                        }
                    },
                    typing,
                    op_id,
                ))
            }
            ast::ExpressionData::UnaryOperation { operator, operand } => {
                let operator = cir::UnaryOperation::from_ast(operator);
                let (operand, operand_type, operand_id) = self.typecheck_expr(&operand)?;
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
                    cir::UnaryOperation::Neg
                    | cir::UnaryOperation::Not
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
                    },
                    operation_type,
                    id,
                ))
            }
            ast::ExpressionData::CharacterLiteral { value } => {
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::StringLiteral {
                            value: String::from(*value),
                        }
                    },
                    cir::Typing::Primitive(cir::Primitive::Char),
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::StrLiteral { value } => {
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::StringLiteral {
                            value: value.clone(),
                        }
                    },
                    // TODO: Change this to `str` type.
                    cir::Typing::Pointer {
                        typ: Box::new(cir::Typing::Primitive(cir::Primitive::U8)),
                        mutable: false,
                    },
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::Tuple { exprs } => {
                let mut validated_exprs = Vec::new();
                let mut types = Vec::new();
                for expr in exprs {
                    let (e, t, _i) = self.typecheck_expr(expr)?;
                    validated_exprs.push(e);
                    types.push(t);
                };
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Tuple {
                            exprs: validated_exprs,
                            types: types.clone(),
                        }
                    },
                    cir::Typing::Tuple { types },
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::Array { exprs, amount } => {
                let mut checked_exprs = Vec::new();
                let mut r#type: Option<cir::Typing> = None;
                for i in exprs {
                    let (expr, expr_typing, _) = self.typecheck_expr(i)?;
                    let span = i.span;
                    checked_exprs.push(expr);
                    if let Some(ref expected) = r#type {
                        if expected != &expr_typing {
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
                }
                return Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::ArrayLiteral {
                            exprs: checked_exprs,
                            typ: r#type.clone().unwrap(),
                        }
                    },
                    // TODO: Change this to `str` type.
                    cir::Typing::Array {
                        typ: Box::new(r#type.unwrap()),
                        amount: *amount,
                    },
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::TupleAccess { origin, field } => {
                let (tuple, typ, id) = self.typecheck_expr(origin)?;
                if let cir::Typing::Tuple { ref types } = typ {
                    if *field >= types.len() {
                        return Err(SemanticError {
                            span: expr.span,
                            case: SemanticErrorCase::InvalidTupleIndex {
                                tried_to: *field,
                                tuple: format!("{}", &typ),
                                items_count: types.len(),
                            }
                        });
                    }
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::TupleAccess {
                                origin: Box::new(tuple),
                                field: *field,
                            }
                        },
                        types.get(*field).unwrap().clone(),
                        id,
                    ));
                } else {
                    return Err(SemanticError {
                        span: expr.span,
                        case: SemanticErrorCase::TypeMismatch {
                            got: format!("{}", typ),
                            expected: format!("tuple")
                        }
                    })
                }
            }
            ast::ExpressionData::Subscript { origin, inner } => {
                let (origin, origin_type, origin_id) = self.typecheck_expr(origin)?;
                let (inner, inner_type, _) = self.typecheck_expr(inner)?;
                if let cir::Typing::Array { typ, .. } = origin_type {
                    if inner_type != cir::Typing::Primitive(cir::Primitive::Int) {
                        return Err(SemanticError {
                            span: inner.span,
                            case: SemanticErrorCase::TypeMismatch {
                                got: format!("{}", inner_type),
                                expected: format!("integer type")
                            }
                        })
                    }
                    return Ok((
                        cir::Expression {
                            span: expr.span,
                            data: cir::ExpressionData::ArraySubscript {
                                origin: Box::new(origin),
                                index: Box::new(inner)
                            }
                        },
                        *typ,
                        origin_id
                    ))
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
                let (expression, typing, id) = self.typecheck_expr(origin)?;
                if let ExpressionIdentity::Locatable(..) = id {
                } else {
                    return Err(SemanticError {
                        span: origin.span,
                        case: SemanticErrorCase::InvalidExpression {
                            what: format!("{} expression", id),
                            should: String::from("locatable expression"),
                        },
                    });
                }
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
                                case: SemanticErrorCase::UnresolvedMember {
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
                                }
                            },
                            typ.unwrap(),
                            id,
                        ));
                    }
                    _ => {
                        return Err(SemanticError {
                            span: origin.span,
                            case: SemanticErrorCase::NonAggregateMemberAccess {
                                typ: format!("{}", typing),
                                member: field.clone(),
                            },
                        });
                    }
                }
            }
            ast::ExpressionData::FunctionCall {
                function,
                arguments,
            } => {
                if let ast::ExpressionData::Identifier { name } = &function.data {
                    return self.typecheck_function_call(name, arguments, function.span);
                } else {
                    todo!("implement other cases for function call")
                }
            }
            ast::ExpressionData::StructInit { name, fields } => {
                let span = expr.span;
                let strukt = self
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
                            case: SemanticErrorCase::UnresolvedMember {
                                name: format!("{}", &field.name),
                                from: format!("struct {}", &strukt.name),
                            },
                        })?;
                    let field_value_span = field.value.span;
                    let (expr, typing, _) = self.typecheck_expr(&field.value)?;
                    if &typing != expected_typing {
                        return Err(SemanticError {
                            span: field_value_span,
                            case: SemanticErrorCase::TypeMismatch {
                                got: format!("{}", typing),
                                expected: format!("{}", expected_typing),
                            },
                        });
                    }
                    checked_fields.push((field.name.clone(), expr));
                }
                let thing = cir::ExpressionData::StructInit {
                    origin: strukt.clone(),
                    fields: checked_fields,
                };
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: thing,
                    },
                    cir::Typing::Struct(strukt),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::IntegerLiteral { value } => {
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Integer {
                            value: *value,
                        }
                    },
                    cir::Typing::Primitive(cir::Primitive::Int),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::FloatLiteral { value } => {
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Float { value: *value }
                    },
                    cir::Typing::Primitive(cir::Primitive::Float),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => Ok((
                cir::Expression {
                    span: expr.span,
                    data: cir::ExpressionData::Bool { value: *value },
                },
                cir::Typing::Primitive(cir::Primitive::Bool),
                ExpressionIdentity::Immediate,
            )),
            ast::ExpressionData::Identifier { name } => {
                let thing = cir::Expression {
                    span: expr.span,
                    data: cir::ExpressionData::Identifier { name: name.clone() }
                };
                if let Some(typ) = self.namespace.constants.get(name) {
                    return Ok((
                        thing,
                        typ.clone(),
                        ExpressionIdentity::Immediate,
                    ));
                } else if let Some(f) = self.namespace.functions.get(name) {
                    return Ok((
                        thing,
                        f.ret.clone(), // FIXME: This should be the function pointer type
                        ExpressionIdentity::Immediate,
                    ));
                } else {
                    // Iterate the local namespace in reverse (from the most recent scope to the oldest)
                    // to find the variable.
                    // This is because the most recent scope should take precedence.
                    // If the variable is not found, return an error.
                    for i in self.namespace.locals.iter().rev() {
                        if let Some(var) = i.get(name) {
                            return Ok((
                                thing,
                                var.typing.clone(),
                                ExpressionIdentity::Locatable(var.mutable),
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
                    let (a, s1) = self.controlcheck_inner_function_block(span, block_true, false, function_name, return_type)?;
                    let (b, s2) = self.controlcheck_inner_function_block(span, block_false, false, function_name, return_type)?;
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

                let (expr, typ, _) = self.typecheck_expr(assignment)?;

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
                let (expr, typ, _) = self.typecheck_expr(assignment)?;

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
                let (expr, typ, _) = self.typecheck_expr(assignment)?;
                let annotated = self.check_type(&stmt.typing)?;
                if annotated != typ {
                    return Err(SemanticError {
                        span: stmt.typing.span,
                        case: SemanticErrorCase::TypeMismatch {
                            got: format!("{}", typ),
                            expected: format!("{}", annotated),
                        },
                    });
                }
                self.namespace.constants.insert(name.clone(), typ.clone());
                Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::Constant {
                        value: expr,
                        binding: name.clone(),
                        typing: typ,
                    }
                })
            }
            ast::Statement::ReturnStatement(stmt) => {
                if expects_return.is_none() && stmt.expr.is_some() {
                    return Err(SemanticError { span: node.span, case: SemanticErrorCase::MisplacedReturn })
                }
                if let Some(expr) = &stmt.expr {
                    let (expr, typ, _) = self.typecheck_expr(expr)?;
                    if &typ != expects_return.unwrap() {
                        if expects_return.unwrap() == &cir::Typing::Void {
                            return Err(SemanticError { span: node.span, case: SemanticErrorCase::ReturnValueOnVoidFunction {
                                function: self.current_function.clone(),
                            }})
                        }
                        return Err(SemanticError { span: node.span, case: SemanticErrorCase::MismatchedReturnType {
                            function: self.current_function.clone(),
                            got: format!("{}", typ),
                            expected: format!("{}", expects_return.unwrap()),
                        }});
                    }
                    return Ok(cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::ReturnStatement {
                            value: Some(expr),
                            typing: typ,
                        }
                    });
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
                // TODO: Add proper type-checking for the function block (check return type in all control flow branches)
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
                self.namespace.functions.insert(stmt.name.clone(), head.clone());

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
                // TODO: Add type-checking
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
                self.namespace.functions.insert(stmt.name.clone(), head.clone());
                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::ExternFnStatement(head)
                    }
                );
            }
            ast::Statement::StructStatement(stmt) => {
                let mut fields = HashMap::new();
                for TypedField { name, typing } in &stmt.fields {
                    let checked_type = self.check_type(typing)?;
                    fields.insert(name.clone(), checked_type);
                }
                let e = cir::Struct {
                    name: stmt.name,
                    fields,
                };
                self.namespace.structs.insert(e.name.clone(), e.clone());
                return Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::StructStatement(e)
                });
            }
            ast::Statement::EnumStatement(stmt) => {
                let e = cir::Enum {
                    name: stmt.name,
                    variants: stmt.variants,
                };
                self.namespace.enums.insert(e.name.clone(), e.clone());
                return Ok(cir::Statement {
                    span: node.span,
                    kind: cir::StatementKind::EnumStatement(e)
                });
            }
            ast::Statement::IfStatement(stmt) => {
                let (condition, typing, _) = self.typecheck_expr(&stmt.condition)?;
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
                // TODO: Remember to push a new scope to the namespace
                let (condition, typing, _) = self.typecheck_expr(&stmt.condition)?;
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
                        kind: cir::StatementKind::ExpressionStatement(self.typecheck_expr(&stmt)?.0)
                    }
                );
            }
        }
    }

    // Type-check, control-flow check and transform the AST into the IR of Elo code
    pub fn go(&mut self, input: Vec<ast::Node>) -> cir::Program {
        // This is why i'm making a language
        let mut stmts = Vec::new();
        for node in Box::new(input).into_iter() {
            match self.typecheck_node(node, None) {
                Ok(s) => stmts.push(s),
                Err(e) => self.errors.push(e),
            }
        }
        cir::Program { nodes: stmts }
    }
}
