use elo_ast::ast::{self, Expression, TypedField};
use elo_error::typeerror::*;
use elo_ir::ir;
use elo_lexer::span::Span;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    pub name: Option<String>,
    pub constants: HashMap<String, ir::Typing>,
    pub structs: HashMap<String, ir::Struct>,
    pub enums: HashMap<String, ir::Enum>,
    pub functions: HashMap<String, ir::FunctionHead>,
    pub locals: Vec<Scope>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub mutable: bool,
    pub typing: ir::Typing,
}

pub type Scope = HashMap<String, Variable>;

#[derive(Debug, PartialEq, Eq)]
// This enum is like an extended version of the concept of Lvalues and Rvalues in C/C++ terms
// read more
enum ExpressionIdentity {
    Locatable(bool), // bool: mutable
    Immediate,
}

type ExpressionMetadata = (ir::Expression, ir::Typing, ExpressionIdentity);

pub struct TypeChecker {
    namespace: Namespace,
    pub errors: Vec<TypeError>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
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

    fn check_type(&mut self, typ: &ast::Type) -> Result<ir::Typing, TypeError> {
        match &typ.typing {
            // TODO: Add generics
            ast::Typing::Named { name, .. } => {
                if let Some(t) = ir::Primitive::from_str(name) {
                    return Ok(ir::Typing::Primitive(t));
                } else if let Some(e) = self.namespace.enums.get(name) {
                    return Ok(ir::Typing::Enum(e.clone()));
                } else if let Some(e) = self.namespace.structs.get(name) {
                    return Ok(ir::Typing::Struct(e.clone()));
                }
                return Err(TypeError {
                    span: Some(typ.span),
                    case: TypeErrorCase::InvalidType {
                        what: format!("{:?}", typ.typing),
                    },
                });
            }
            ast::Typing::Pointer { typ } => {
                let inner_typing = self.check_type(typ)?;
                return Ok(ir::Typing::Pointer {
                    typ: Box::new(inner_typing),
                });
            }
            x => Err(TypeError {
                span: Some(typ.span),
                case: TypeErrorCase::InvalidType {
                    what: format!("{:?}", x),
                },
            }),
        }
    }

    fn typecheck_binop(
        &mut self,
        lhs: &ExpressionMetadata,
        rhs: &ExpressionMetadata,
        binop: &ast::BinaryOperation,
        span: elo_lexer::span::Span,
    ) -> Result<(ir::BinaryOperation, ir::Typing, ExpressionIdentity), TypeError> {
        let ir_binop = ir::BinaryOperation::from_ast(&binop);
        if rhs.1 != lhs.1 {
            return Err(TypeError {
                span: Some(span),
                case: TypeErrorCase::TypeMismatch {
                    got: format!("{:?}", rhs.1),
                    expected: format!("{:?}", lhs.1),
                },
            });
        }
        let typing = match ir_binop {
            ir::BinaryOperation::Add
            | ir::BinaryOperation::Sub
            | ir::BinaryOperation::Mul
            | ir::BinaryOperation::Div
            | ir::BinaryOperation::Mod
            | ir::BinaryOperation::BAnd
            | ir::BinaryOperation::BOr
            | ir::BinaryOperation::BNot
            | ir::BinaryOperation::BXor
            | ir::BinaryOperation::LShift
            | ir::BinaryOperation::RShift => lhs.1.clone(),
            ir::BinaryOperation::Eq
            | ir::BinaryOperation::Ne
            | ir::BinaryOperation::Lt
            | ir::BinaryOperation::Le
            | ir::BinaryOperation::Gt
            | ir::BinaryOperation::Ge
            | ir::BinaryOperation::And
            | ir::BinaryOperation::Or => ir::Typing::Primitive(ir::Primitive::Bool),
            ir::BinaryOperation::Assign
            | ir::BinaryOperation::AssignAdd
            | ir::BinaryOperation::AssignSub
            | ir::BinaryOperation::AssignMul
            | ir::BinaryOperation::AssignDiv
            | ir::BinaryOperation::AssignMod
            | ir::BinaryOperation::AssignBAnd
            | ir::BinaryOperation::AssignBOr
            | ir::BinaryOperation::AssignBXor
            | ir::BinaryOperation::AssignBNot => {
                match lhs.2 {
                    ExpressionIdentity::Locatable(false) => {
                        return Err(TypeError {
                            span: Some(span),
                            case: TypeErrorCase::InvalidExpression {
                                what: format!("{:?}", lhs.0),
                                should: "mutable left-hand-side operand".to_string(),
                            },
                        });
                    }
                    ExpressionIdentity::Immediate => {
                        return Err(TypeError {
                            span: Some(span),
                            case: TypeErrorCase::InvalidExpression {
                                what: format!("{:?}", lhs.0),
                                should: "valid left-hand-side operand".to_string(),
                            },
                        });
                    }
                    _ => {} // Ok! For assignment, the lhs must be a mutable locatable value
                }
                ir::Typing::Void
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
            span: Some(span),
            case: TypeErrorCase::UnresolvedName {
                name: name.to_string(),
            },
        })?;
        let return_type = function.ret.clone();
        let passed_length = arguments.len();
        let expected_len = function.arguments.len();
        if passed_length < expected_len {
            return Err(TypeError {
                span: Some(span),
                case: TypeErrorCase::UnmatchedArguments {
                    name: name.to_string(),
                    got: passed_length,
                    expected: expected_len,
                    too_much: false,
                },
            });
        } else if (passed_length > expected_len) && !function.variadic {
            return Err(TypeError {
                span: Some(span),
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
                    span: Some(expression.span),
                    case: TypeErrorCase::TypeMismatch {
                        got: format!("{:?}", got_type),
                        expected: format!("{:?}", expected_type),
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
            ir::Expression::FunctionCall {
                function: Box::new(ir::Expression::Identifier {
                    name: name.to_string(),
                }),
                arguments: checked_arguments,
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
                    ir::Expression::BinaryOperation {
                        operator,
                        left: Box::new(lhs.0),
                        right: Box::new(rhs.0),
                    },
                    typing,
                    op_id,
                ))
            }
            ast::ExpressionData::UnaryOperation { operator, operand } => {
                let operator = ir::UnaryOperation::from_ast(operator);
                let (operand, operand_type, operand_id) = self.typecheck_expr(&operand)?;
                let operation_type;
                let id;
                match operator {
                    ir::UnaryOperation::Addr => {
                        if let ExpressionIdentity::Immediate = operand_id {
                            return Err(TypeError {
                                span: Some(expr.span),
                                case: TypeErrorCase::InvalidExpression {
                                    what: format!("{:?}", operand),
                                    should: "valid value to reference".to_string(),
                                },
                            });
                        }
                        operation_type = ir::Typing::Pointer {
                            typ: Box::new(operand_type),
                        };
                        id = ExpressionIdentity::Immediate;
                    }
                    ir::UnaryOperation::Neg
                    | ir::UnaryOperation::Not
                    | ir::UnaryOperation::BNot => {
                        operation_type = operand_type;
                        id = ExpressionIdentity::Immediate;
                    }
                    ir::UnaryOperation::Deref => match operand_id {
                        ExpressionIdentity::Immediate => {
                            return Err(TypeError {
                                span: Some(expr.span),
                                case: TypeErrorCase::InvalidExpression {
                                    what: format!("{:?}", operand),
                                    should: "valid value to dereference".to_string(),
                                },
                            });
                        }
                        e => {
                            if let ir::Typing::Pointer { typ } = operand_type {
                                operation_type = *typ;
                            } else {
                                return Err(TypeError {
                                    span: Some(expr.span),
                                    case: TypeErrorCase::TypeMismatch {
                                        got: format!("{:?}", operand_type),
                                        expected: "pointer".to_string(),
                                    },
                                });
                            }
                            id = e;
                        }
                    },
                };
                Ok((
                    ir::Expression::UnaryOperation {
                        operator,
                        operand: Box::new(operand),
                    },
                    operation_type,
                    id,
                ))
            }
            ast::ExpressionData::CharacterLiteral { value } => {
                return Ok((
                    ir::Expression::StringLiteral {
                        value: String::from(*value),
                    },
                    ir::Typing::Primitive(ir::Primitive::Char),
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::StrLiteral { value } => {
                return Ok((
                    ir::Expression::StringLiteral {
                        value: value.clone(),
                    },
                    // TODO: Change this to `str` type.
                    ir::Typing::Pointer {
                        typ: Box::new(ir::Typing::Primitive(ir::Primitive::U8)),
                    },
                    ExpressionIdentity::Immediate,
                ));
            }
            ast::ExpressionData::Tuple { exprs: _exprs } => {
                todo!();
            }
            ast::ExpressionData::Array { exprs, amount } => {
                let mut checked_exprs = Vec::new();
                let mut r#type: Option<ir::Typing> = None;
                for i in exprs {
                    let (expr, expr_typing, _) = self.typecheck_expr(i)?;
                    let span = i.span;
                    checked_exprs.push(expr);
                    if let Some(ref expected) = r#type {
                        if expected != &expr_typing {
                            return Err(TypeError {
                                span: Some(span),
                                case: TypeErrorCase::TypeMismatch {
                                    got: format!("{:?}", expr_typing),
                                    expected: format!("{:?}", expected),
                                },
                            });
                        }
                    } else {
                        r#type = Some(expr_typing);
                    }
                }
                return Ok((
                    ir::Expression::ArrayLiteral {
                        exprs: checked_exprs,
                        typ: r#type.clone().unwrap(),
                    },
                    // TODO: Change this to `str` type.
                    ir::Typing::Array {
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
                        span: Some(origin.span),
                        case: TypeErrorCase::InvalidExpression {
                            what: format!("{:?} expression", id),
                            should: String::from("locatable expression"),
                        },
                    });
                }
                match typing {
                    ir::Typing::Struct(st) => {
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
                                span: Some(expr.span),
                                case: TypeErrorCase::UnresolvedMember {
                                    name: format!("{field}"),
                                    from: format!("struct {}", st.name),
                                },
                            });
                        }
                        return Ok((
                            ir::Expression::FieldAccess {
                                origin: Box::new(expression),
                                field: field.clone(),
                            },
                            typ.unwrap(),
                            id,
                        ));
                    }
                    _ => {
                        return Err(TypeError {
                            span: Some(origin.span),
                            case: TypeErrorCase::NonAggregateMemberAccess {
                                typ: format!("{:?}", typing),
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
                    return Err(TypeError {
                        span: Some(expr.span),
                        case: TypeErrorCase::InvalidExpression {
                            what: format!("{:?}", &function.data),
                            should: "identifier".to_string(),
                        },
                    });
                }
            }
            ast::ExpressionData::StructInit { name, fields } => {
                let span = expr.span;
                let strukt = self
                    .namespace
                    .structs
                    .get(name)
                    .ok_or_else(|| TypeError {
                        span: Some(span),
                        case: TypeErrorCase::UnresolvedName {
                            name: format!("{}", &name),
                        },
                    })?
                    .clone();
                let mut checked_fields = Vec::new();
                for field in fields {
                    let expected_typing =
                        strukt.fields.get(&field.name).ok_or_else(|| TypeError {
                            span: Some(span),
                            case: TypeErrorCase::UnresolvedMember {
                                name: format!("{}", &field.name),
                                from: format!("struct {}", &strukt.name),
                            },
                        })?;
                    let field_value_span = field.value.span;
                    let (expr, typing, _) = self.typecheck_expr(&field.value)?;
                    if &typing != expected_typing {
                        return Err(TypeError {
                            span: Some(field_value_span),
                            case: TypeErrorCase::TypeMismatch {
                                got: format!("{:?}", typing),
                                expected: format!("{:?}", expected_typing),
                            },
                        });
                    }
                    checked_fields.push((field.name.clone(), expr));
                }
                let thing = ir::Expression::StructInit {
                    origin: strukt.clone(),
                    fields: checked_fields,
                };
                Ok((
                    thing,
                    ir::Typing::Struct(strukt),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::IntegerLiteral { value } => {
                let (lit, radix) = value;
                Ok((
                    ir::Expression::Integer {
                        value: i128::from_str_radix(lit, *radix).unwrap(),
                    },
                    ir::Typing::Primitive(ir::Primitive::Int),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::FloatLiteral { int, float } => {
                let integer = u64::from_str_radix(&int.0, int.1).unwrap();
                let fractional = u64::from_str_radix(&float.0, float.1).unwrap();
                let value = format!("{}.{}", integer, fractional).parse().unwrap();
                Ok((
                    ir::Expression::Float { value },
                    ir::Typing::Primitive(ir::Primitive::Float),
                    ExpressionIdentity::Immediate,
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => Ok((
                ir::Expression::Bool { value: *value },
                ir::Typing::Primitive(ir::Primitive::Bool),
                ExpressionIdentity::Immediate,
            )),
            ast::ExpressionData::Identifier { name } => {
                if let Some(typ) = self.namespace.constants.get(name) {
                    return Ok((
                        ir::Expression::Identifier { name: name.clone() },
                        typ.clone(),
                        ExpressionIdentity::Immediate,
                    ));
                } else if let Some(f) = self.namespace.functions.get(name) {
                    return Ok((
                        ir::Expression::Identifier { name: name.clone() },
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
                                ir::Expression::Identifier { name: name.clone() },
                                var.typing.clone(),
                                ExpressionIdentity::Locatable(var.mutable),
                            ));
                        }
                    }
                    return Err(TypeError {
                        span: Some(expr.span),
                        case: TypeErrorCase::UnresolvedName { name: name.clone() },
                    });
                }
            }
        }
    }

    fn typecheck_generic_block(&mut self, block: Vec<ast::Node>) -> Vec<ir::Statement> {
        let mut blk = Vec::new();
        for a in Box::new(block).into_iter() {
            match self.typecheck_node(a) {
                Ok(stmt) => blk.push(stmt),
                Err(e) => self.errors.push(e),
            }
        }
        blk
    }

    fn typecheck_node(&mut self, node: ast::Node) -> Result<ir::Statement, TypeError> {
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
                Ok(ir::Statement::Variable {
                    assignment: expr,
                    binding: name.clone(),
                    typing: typ,
                })
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
                Ok(ir::Statement::Variable {
                    assignment: expr,
                    binding: name.clone(),
                    typing: typ,
                })
            }
            ast::Statement::ConstStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ, _) = self.typecheck_expr(assignment)?;
                let annotated = self.check_type(&stmt.typing)?;
                if annotated != typ {
                    return Err(TypeError {
                        span: Some(stmt.typing.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typ),
                            expected: format!("{:?}", annotated),
                        },
                    });
                }
                self.namespace.constants.insert(name.clone(), typ.clone());
                Ok(ir::Statement::Constant {
                    value: expr,
                    binding: name.clone(),
                    typing: typ,
                })
            }
            ast::Statement::ReturnStatement(stmt) => {
                if let Some(expr) = &stmt.expr {
                    let (expr, typ, _) = self.typecheck_expr(expr)?;
                    return Ok(ir::Statement::ReturnStatement {
                        value: Some(expr),
                        typing: typ,
                    });
                }
                Ok(ir::Statement::ReturnStatement {
                    value: None,
                    typing: ir::Typing::Void,
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
                    None => ir::Typing::Void,
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

                let mut validated_block = self.typecheck_generic_block(stmt.block.content);

                // Add extra return to the end in case of a function that returns void, or it will segfault
                // NOTE: It would if we were still using llvm, but now with C backend this doesn't matter,
                //       but it's good to keep it here anyways
                if validated_ret_type == ir::Typing::Void {
                    validated_block.push(ir::Statement::ReturnStatement {
                        value: None,
                        typing: ir::Typing::Void,
                    });
                }

                // Pop the scope
                self.namespace.locals.pop();

                let head = ir::FunctionHead {
                    name: stmt.name.clone(),
                    ret: validated_ret_type,
                    arguments: validated_args,
                    variadic: false, // In this case, variadic is ALWAYS false
                                     // Because Elo is not meant to support variadic functions at all.
                };

                let validated = ir::Function {
                    head: head.clone(),
                    block: validated_block,
                };

                // Insert the function into the namespace
                self.namespace.functions.insert(stmt.name, head);

                return Ok(ir::Statement::FnStatement(validated));
            }
            ast::Statement::ExternFnStatement(stmt) => {
                // TODO: Add type-checking
                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push((a.name.clone(), self.check_type(&a.typing)?));
                }
                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.check_type(ret_type)?,
                    None => ir::Typing::Void,
                };
                let head = ir::FunctionHead {
                    name: stmt.name.clone(),
                    ret: validated_ret_type.clone(),
                    arguments: validated_args.clone(),
                    variadic: stmt.variadic,
                };
                self.namespace.functions.insert(stmt.name.clone(), head);
                return Ok(ir::Statement::ExternFnStatement(ir::FunctionHead {
                    name: stmt.name,
                    ret: validated_ret_type,
                    arguments: validated_args,
                    variadic: stmt.variadic,
                }));
            }
            ast::Statement::StructStatement(stmt) => {
                let mut fields = HashMap::new();
                for TypedField { name, typing } in &stmt.fields {
                    let checked_type = self.check_type(typing)?;
                    fields.insert(name.clone(), checked_type);
                }
                let e = ir::Struct {
                    name: stmt.name,
                    fields,
                };
                self.namespace.structs.insert(e.name.clone(), e.clone());
                return Ok(ir::Statement::StructStatement(e));
            }
            ast::Statement::EnumStatement(stmt) => {
                let e = ir::Enum {
                    name: stmt.name,
                    variants: stmt.variants,
                };
                self.namespace.enums.insert(e.name.clone(), e.clone());
                return Ok(ir::Statement::EnumStatement(e));
            }
            ast::Statement::IfStatement(stmt) => {
                let (condition, typing, _) = self.typecheck_expr(&stmt.condition)?;
                if typing != ir::Typing::Primitive(ir::Primitive::Bool) {
                    return Err(TypeError {
                        span: Some(stmt.condition.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typing),
                            expected: format!("{:?}", ir::Typing::Primitive(ir::Primitive::Bool)),
                        },
                    });
                }

                self.namespace.locals.push(HashMap::new());
                let block_true_content = self.typecheck_generic_block(stmt.block_true.content);
                self.namespace.locals.pop(); // Pop the true block scope

                self.namespace.locals.push(HashMap::new());
                let mut block_false_content = vec![];
                if let Some(block_false) = stmt.block_false {
                    block_false_content = self.typecheck_generic_block(block_false.content);
                }
                self.namespace.locals.pop(); // Pop the false block scope

                return Ok(ir::Statement::IfStatement {
                    condition,
                    block_true: block_true_content,
                    block_false: block_false_content,
                });
            }
            ast::Statement::WhileStatement(stmt) => {
                // TODO: Remember to push a new scope to the namespace
                let (condition, typing, _) = self.typecheck_expr(&stmt.condition)?;
                if typing != ir::Typing::Primitive(ir::Primitive::Bool) {
                    return Err(TypeError {
                        span: Some(stmt.condition.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typing),
                            expected: format!("{:?}", ir::Typing::Primitive(ir::Primitive::Bool)),
                        },
                    });
                }
                self.namespace.locals.push(HashMap::new());
                let block = self.typecheck_generic_block(stmt.block.content);
                self.namespace.locals.pop();
                return Ok(ir::Statement::WhileStatement { condition, block });
            }
            ast::Statement::ExpressionStatement(stmt) => {
                return Ok(ir::Statement::ExpressionStatement(
                    self.typecheck_expr(&stmt)?.0,
                ));
            }
        }
    }

    // Type-check and transform the AST into the IR of Elo code
    pub fn go(&mut self, input: Vec<ast::Node>) -> ir::Program {
        // This is why i'm making a language
        let nodes = self.typecheck_generic_block(input);
        ir::Program { nodes }
    }
}
