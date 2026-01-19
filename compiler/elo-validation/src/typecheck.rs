use elo_ir::ast::{self, Expression, TypedField};
use elo_error::typeerror::*;
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

pub struct Variable {
    pub name: String,
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

pub struct TypeChecker {
    namespace: Namespace,
    current_function: String,
    pub errors: Vec<TypeError>,
}

impl TypeChecker {
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

    fn check_type(&mut self, typ: &ast::Type) -> Result<cir::Typing, TypeError> {
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
                return Err(TypeError {
                    span: typ.span,
                    case: TypeErrorCase::UnresolvedName {
                        name: name.clone(),
                    },
                });
            }
            ast::Typing::Pointer { typ } => {
                let inner_typing = self.check_type(typ)?;
                return Ok(cir::Typing::Pointer {
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
    ) -> Result<(cir::BinaryOperation, cir::Typing, ExpressionIdentity), TypeError> {
        let ir_binop = cir::BinaryOperation::from_ast(&binop);
        if rhs.1 != lhs.1 {
            return Err(TypeError {
                span: span,
                case: TypeErrorCase::TypeMismatch {
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
                        return Err(TypeError {
                            span: span,
                            case: TypeErrorCase::AssignImmutable {
                                expression: format!("{}", lhs.0),
                            },
                        });
                    }
                    ExpressionIdentity::Immediate => {
                        return Err(TypeError {
                            span: span,
                            case: TypeErrorCase::InvalidExpression {
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
    ) -> Result<ExpressionMetadata, TypeError> {
        let function = self.namespace.functions.get(name).ok_or(TypeError {
            span: span,
            case: TypeErrorCase::UnresolvedName {
                name: name.to_string(),
            },
        })?;
        let return_type = function.ret.clone();
        let passed_length = arguments.len();
        let expected_len = function.arguments.len();
        if passed_length < expected_len {
            return Err(TypeError {
                span: span,
                case: TypeErrorCase::UnmatchedArguments {
                    name: name.to_string(),
                    got: passed_length,
                    expected: expected_len,
                    too_much: false,
                },
            });
        } else if (passed_length > expected_len) && !function.variadic {
            return Err(TypeError {
                span: span,
                case: TypeErrorCase::UnmatchedArguments {
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
                return Err(TypeError {
                    span: expression.span,
                    case: TypeErrorCase::TypeMismatch {
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
                }
            },
            return_type,
            ExpressionIdentity::Immediate,
        ));
    }

    fn typecheck_expr(&mut self, expr: &ast::Expression) -> Result<ExpressionMetadata, TypeError> {
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
                        if let ExpressionIdentity::Immediate = operand_id {
                            return Err(TypeError {
                                span: expr.span,
                                case: TypeErrorCase::InvalidExpression {
                                    what: format!("{}", operand),
                                    should: "valid value to reference".to_string(),
                                },
                            });
                        }
                        operation_type = cir::Typing::Pointer {
                            typ: Box::new(operand_type),
                        };
                        id = ExpressionIdentity::Immediate;
                    }
                    cir::UnaryOperation::Neg
                    | cir::UnaryOperation::Not
                    | cir::UnaryOperation::BNot => {
                        operation_type = operand_type;
                        id = ExpressionIdentity::Immediate;
                    }
                    cir::UnaryOperation::Deref => match operand_id {
                        ExpressionIdentity::Immediate => {
                            return Err(TypeError {
                                span: expr.span,
                                case: TypeErrorCase::InvalidExpression {
                                    what: format!("{}", operand),
                                    should: "valid value to dereference".to_string(),
                                },
                            });
                        }
                        e => {
                            if let cir::Typing::Pointer { typ } = operand_type {
                                operation_type = *typ;
                            } else {
                                return Err(TypeError {
                                    span: expr.span,
                                    case: TypeErrorCase::TypeMismatch {
                                        got: format!("{}", operand_type),
                                        expected: "pointer".to_string(),
                                    },
                                });
                            }
                            id = e;
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
                            return Err(TypeError {
                                span: span,
                                case: TypeErrorCase::TypeMismatch {
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
            ast::ExpressionData::FieldAccess { origin, field } => {
                let (expression, typing, id) = self.typecheck_expr(origin)?;
                if let ExpressionIdentity::Locatable(..) = id {
                } else {
                    return Err(TypeError {
                        span: origin.span,
                        case: TypeErrorCase::InvalidExpression {
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
                            return Err(TypeError {
                                span: expr.span,
                                case: TypeErrorCase::UnresolvedMember {
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
                        return Err(TypeError {
                            span: origin.span,
                            case: TypeErrorCase::NonAggregateMemberAccess {
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
                    .ok_or_else(|| TypeError {
                        span: span,
                        case: TypeErrorCase::UnresolvedName {
                            name: format!("{}", &name),
                        },
                    })?
                    .clone();
                let mut checked_fields = Vec::new();
                for field in fields {
                    let expected_typing =
                        strukt.fields.get(&field.name).ok_or_else(|| TypeError {
                            span: span,
                            case: TypeErrorCase::UnresolvedMember {
                                name: format!("{}", &field.name),
                                from: format!("struct {}", &strukt.name),
                            },
                        })?;
                    let field_value_span = field.value.span;
                    let (expr, typing, _) = self.typecheck_expr(&field.value)?;
                    if &typing != expected_typing {
                        return Err(TypeError {
                            span: field_value_span,
                            case: TypeErrorCase::TypeMismatch {
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
                let (lit, radix) = value;
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Integer {
                            value: i128::from_str_radix(lit, *radix).unwrap(),
                        }
                    },
                    cir::Typing::Primitive(cir::Primitive::Int),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::FloatLiteral { int, float } => {
                let integer = u64::from_str_radix(&int.0, int.1).unwrap();
                let fractional = u64::from_str_radix(&float.0, float.1).unwrap();
                let mut value = integer as f64;
                value += (fractional as f64)/float.1.pow(float.0.len() as u32) as f64;
                Ok((
                    cir::Expression {
                        span: expr.span,
                        data: cir::ExpressionData::Float { value }
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
                    return Err(TypeError {
                        span: expr.span,
                        case: TypeErrorCase::UnresolvedName { name: name.clone() },
                    });
                }
            }
        }
    }

    fn typecheck_generic_block(&mut self, block: Vec<ast::Node>, expects_return: Option<&cir::Typing>) -> Result<Vec<cir::Statement>, TypeError> {
        let mut blk = Vec::new();
        for a in Box::new(block).into_iter() {
            blk.push(self.typecheck_node(a, expects_return)?);
        }
        Ok(blk)
    }

    fn typecheck_inner_function_block(&mut self, span: Span, block: &Vec<cir::Statement>, is_top_level: bool, function_name: &str, return_type: &cir::Typing) -> Result<(bool, Span), TypeError> {
        let mut last_span = span;
        for i in block {
            last_span = i.span;
            match &i.kind {
                cir::StatementKind::ReturnStatement { value, .. } => {
                    return Ok((true, i.span));
                }
                cir::StatementKind::IfStatement { block_true, block_false, .. } => {
                    let (a, s1) = self.typecheck_inner_function_block(span, block_true, false, function_name, return_type)?;
                    let (b, s2) = self.typecheck_inner_function_block(span, block_false, false, function_name, return_type)?;
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
            return Err(TypeError { span: last_span, case: TypeErrorCase::NoReturn {
                function: function_name.to_string(),
                returns: format!("{}", return_type)
            }});
        }
        Ok((false, last_span))
    }

    fn typecheck_function_block(&mut self, span: Span, block: Vec<ast::Node>, function_name: &str, return_type: &cir::Typing) -> Result<Vec<cir::Statement>, TypeError> {
        self.current_function = function_name.to_string();
        let blk = self.typecheck_generic_block(block, Some(return_type))?;
        self.typecheck_inner_function_block(span, &blk, true, function_name, return_type)?;
        Ok(blk)
    }

    fn typecheck_node(&mut self, node: ast::Node, expects_return: Option<&cir::Typing>) -> Result<cir::Statement, TypeError> {
        match node.stmt {
            ast::Statement::LetStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ, _) = self.typecheck_expr(assignment)?;

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
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

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
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
                    return Err(TypeError {
                        span: stmt.typing.span,
                        case: TypeErrorCase::TypeMismatch {
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
                    return Err(TypeError { span: node.span, case: TypeErrorCase::MisplacedReturn })
                }
                if let Some(expr) = &stmt.expr {
                    let (expr, typ, _) = self.typecheck_expr(expr)?;
                    if &typ != expects_return.unwrap() {
                        if expects_return.unwrap() == &cir::Typing::Void {
                            return Err(TypeError { span: node.span, case: TypeErrorCase::ReturnValueOnVoidFunction {
                                function: self.current_function.clone(),
                            }})
                        }
                        return Err(TypeError { span: node.span, case: TypeErrorCase::MismatchedReturnType {
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

                // Create a new scope for the function
                self.namespace.locals.push(HashMap::new());

                // Add the arguments to the scope
                for arg in validated_args.iter() {
                    let (name, typing) = arg;
                    self.namespace.locals.last_mut().unwrap().insert(
                        name.clone(),
                        Variable {
                            name: name.clone(),
                            mutable: false,
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
                };
                // Insert the function into the namespace
                self.namespace.functions.insert(stmt.name.clone(), head.clone());

                let validated_block = self.typecheck_function_block(node.span, stmt.block.content, &stmt.name, &validated_ret_type)?;

                // Add extra return to the end in case of a function that returns void, or it will segfault
                // NOTE: It would if we were still using llvm, but now with C backend this doesn't matter,
                //       but it's good to keep it here anyways
                // if validated_ret_type == cir::Typing::Void {
                //     validated_block.push(cir::StatementKind::ReturnStatement {
                //         value: None,
                //         typing: cir::Typing::Void,
                //     });
                // }
                // Removed this anyway because it would be hard to know the span for this return anyway

                // Pop the scope
                self.namespace.locals.pop();

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
                    ret: validated_ret_type.clone(),
                    arguments: validated_args.clone(),
                    variadic: stmt.variadic,
                };
                self.namespace.functions.insert(stmt.name.clone(), head);
                return Ok(
                    cir::Statement {
                        span: node.span,
                        kind: cir::StatementKind::ExternFnStatement(cir::FunctionHead {
                            name: stmt.name,
                            ret: validated_ret_type,
                            arguments: validated_args,
                            variadic: stmt.variadic,
                        })
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
                    return Err(TypeError {
                        span: stmt.condition.span,
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{}", typing),
                            expected: format!("{}", cir::Typing::Primitive(cir::Primitive::Bool)),
                        },
                    });
                }

                self.namespace.locals.push(HashMap::new());
                let block_true_content = self.typecheck_generic_block(stmt.block_true.content, expects_return)?;
                self.namespace.locals.pop(); // Pop the true block scope

                self.namespace.locals.push(HashMap::new());
                let mut block_false_content = vec![];
                if let Some(block_false) = stmt.block_false {
                    block_false_content = self.typecheck_generic_block(block_false.content, expects_return)?;
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
                    return Err(TypeError {
                        span: stmt.condition.span,
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{}", typing),
                            expected: format!("{}", cir::Typing::Primitive(cir::Primitive::Bool)),
                        },
                    });
                }
                self.namespace.locals.push(HashMap::new());
                let block = self.typecheck_generic_block(stmt.block.content, expects_return)?;
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

    // Type-check and transform the AST into the IR of Elo code
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
