use elo_ast::ast::{self, TypedField};
use elo_error::typeerror::*;
use elo_ir::ir;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Namespace {
    pub name: Option<String>,
    pub constants: HashMap<String, ir::Typing>,
    pub structs: HashMap<String, ir::Struct>,
    pub enums: HashMap<String, ir::Enum>,
    pub functions: HashMap<String, ir::Function>,
    pub locals: Vec<Scope>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub mutable: bool,
    pub typing: ir::Typing,
}

#[derive(Debug)]
pub struct Scope {
    pub content: HashMap<String, Variable>,
}

pub struct TypeChecker {
    input: ast::Program,
    namespace: Namespace,
}

impl TypeChecker {
    pub fn new(input: ast::Program) -> Self {
        Self {
            input,
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
        lhs_type: ir::Typing,
        rhs_type: ir::Typing,
        binop: &ast::BinaryOperation,
        span: elo_lexer::span::Span,
    ) -> Result<(ir::BinaryOperation, ir::Typing), TypeError> {
        let ir_binop = ir::BinaryOperation::from_ast(&binop);
        if rhs_type != lhs_type {
            return Err(TypeError {
                span: Some(span),
                case: TypeErrorCase::TypeMismatch {
                    got: format!("{:?}", rhs_type),
                    expected: format!("{:?}", lhs_type),
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
            | ir::BinaryOperation::RShift => lhs_type,
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
            | ir::BinaryOperation::AssignBNot => ir::Typing::Void,
        };
        Ok((ir_binop, typing))
    }

    fn typecheck_expr(
        &mut self,
        expr: &ast::Expression,
    ) -> Result<(ir::Expression, ir::Typing), TypeError> {
        match &expr.data {
            ast::ExpressionData::BinaryOperation {
                operator,
                left,
                right,
            } => {
                let (left, left_t) = self.typecheck_expr(left)?;
                let (right, right_t) = self.typecheck_expr(right)?;
                let (operator, typing) =
                    self.typecheck_binop(left_t, right_t, operator, expr.span)?;
                Ok((
                    ir::Expression::BinaryOperation {
                        operator,
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    typing,
                ))
            }
            ast::ExpressionData::UnaryOperation { operator, operand } => {
                let operator = ir::UnaryOperation::from_ast(operator);
                let (operand, operand_type) = self.typecheck_expr(&operand)?;
                Ok((
                    ir::Expression::UnaryOperation {
                        operator,
                        operand: Box::new(operand),
                    },
                    operand_type,
                ))
            }
            ast::ExpressionData::CharacterLiteral { value } => {
                return Ok((
                    ir::Expression::StringLiteral {
                        value: String::from(*value),
                    },
                    ir::Typing::Primitive(ir::Primitive::Char),
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
                ));
            }
            ast::ExpressionData::Tuple { exprs: _exprs } => {
                todo!();
            }
            ast::ExpressionData::FieldAccess {
                origin: _origin,
                field: _field,
            } => {
                todo!();
            }
            ast::ExpressionData::FunctionCall {
                function,
                arguments,
            } => {
                if let ast::ExpressionData::Identifier { name } = &function.data {
                    if let Some(func) = self.namespace.functions.get(name) {
                        let arguments_to_check: Vec<ir::Typing> = func
                            .head
                            .arguments
                            .iter()
                            .map(|(_, typing)| typing.clone())
                            .collect();
                        let len_args = func.head.arguments.len();
                        let return_type = func.head.ret.clone();
                        if func.head.variadic && arguments.len() < len_args {
                            return Err(TypeError {
                                span: Some(function.span),
                                case: TypeErrorCase::UnmatchedArguments {
                                    name: name.clone(),
                                    got: arguments.len(),
                                    expected: len_args,
                                },
                            });
                        }
                        if !func.head.variadic && arguments.len() != len_args {
                            return Err(TypeError {
                                span: Some(function.span),
                                case: TypeErrorCase::UnmatchedArguments {
                                    name: name.clone(),
                                    got: arguments.len(),
                                    expected: len_args,
                                },
                            });
                        }
                        let mut validated_args = Vec::new();

                        // having to create these variables below so the rust borrow checker shuts up
                        // makes you ask why you're using this language a few times
                        let declared_arguments_len = arguments_to_check.len();
                        let variadic = func.head.variadic;
                        let zip = arguments.iter().zip(arguments_to_check);
                        for (expr, expected_type) in zip {
                            let span = expr.span;
                            let (validated, got_type) = self.typecheck_expr(expr)?;
                            if got_type != expected_type {
                                return Err(TypeError {
                                    span: Some(span),
                                    case: TypeErrorCase::TypeMismatch {
                                        got: format!("{:?}", got_type),
                                        expected: format!("{:?}", expected_type),
                                    },
                                });
                            }
                            validated_args.push(validated);
                        }
                        // Also add the remaining variadic values.
                        if variadic && arguments.len() > declared_arguments_len {
                            for expr in arguments.iter().skip(declared_arguments_len) {
                                let (validated, _) = self.typecheck_expr(expr)?;
                                validated_args.push(validated);
                            }
                        }
                        return Ok((
                            ir::Expression::FunctionCall {
                                function: Box::new(self.typecheck_expr(function)?.0),
                                arguments: validated_args,
                            },
                            return_type.clone(),
                        ));
                    } else {
                        return Err(TypeError {
                            span: Some(expr.span),
                            case: TypeErrorCase::UnresolvedName { name: name.clone() },
                        });
                    }
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
                    let (expr, typing) = self.typecheck_expr(&field.value)?;
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
                Ok((thing, ir::Typing::Struct(strukt)))
            }
            ast::ExpressionData::IntegerLiteral { value } => {
                let (lit, radix) = value;
                Ok((
                    ir::Expression::Integer {
                        value: i128::from_str_radix(lit, *radix).unwrap(),
                    },
                    ir::Typing::Primitive(ir::Primitive::Int),
                ))
            }
            ast::ExpressionData::FloatLiteral { int, float } => {
                let integer = u64::from_str_radix(&int.0, int.1).unwrap();
                let fractional = u64::from_str_radix(&float.0, float.1).unwrap();
                let value = format!("{}.{}", integer, fractional).parse().unwrap();
                Ok((
                    ir::Expression::Float { value },
                    ir::Typing::Primitive(ir::Primitive::Float),
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => Ok((
                ir::Expression::Bool { value: *value },
                ir::Typing::Primitive(ir::Primitive::Bool),
            )),
            ast::ExpressionData::Identifier { name } => {
                if let Some(typ) = self.namespace.constants.get(name) {
                    return Ok((
                        ir::Expression::Identifier { name: name.clone() },
                        typ.clone(),
                    ));
                } else if let Some(e) = self.namespace.enums.get(name) {
                    return Ok((
                        ir::Expression::Identifier { name: name.clone() },
                        ir::Typing::Enum(e.clone()),
                    ));
                } else if let Some(f) = self.namespace.functions.get(name) {
                    return Ok((
                        ir::Expression::Identifier { name: name.clone() },
                        f.head.ret.clone(), // FIXME: This should be the function pointer type
                    ));
                } else {
                    // Iterate the local namespace in reverse (from the most recent scope to the oldest)
                    // to find the variable.
                    // This is because the most recent scope should take precedence.
                    // If the variable is not found, return an error.
                    for i in self.namespace.locals.iter().rev() {
                        if let Some(var) = i.content.get(name) {
                            return Ok((
                                ir::Expression::Identifier { name: name.clone() },
                                var.typing.clone(),
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

    fn typecheck_node(&mut self, node: ast::Node) -> Result<ir::Statement, TypeError> {
        match node.stmt {
            ast::Statement::LetStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.typecheck_expr(assignment)?;

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().content.insert(
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
                let (expr, typ) = self.typecheck_expr(assignment)?;

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().content.insert(
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
                let (expr, typ) = self.typecheck_expr(assignment)?;
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
                    let (expr, typ) = self.typecheck_expr(expr)?;
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
                // TODO: Add type-checking
                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push((a.name.clone(), self.check_type(&a.typing)?));
                }

                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.check_type(ret_type)?,
                    None => ir::Typing::Void,
                };
                let mut validated_block = Vec::new();
                let xs = Box::new(stmt.block.content);

                // Create a new scope for the function
                self.namespace.locals.push(Scope {
                    content: HashMap::new(),
                });

                // Add the arguments to the scope
                for arg in validated_args.iter() {
                    let (name, typing) = arg;
                    self.namespace.locals.last_mut().unwrap().content.insert(
                        name.clone(),
                        Variable {
                            name: name.clone(),
                            mutable: false,
                            typing: typing.clone(),
                        },
                    );
                }
                for a in xs.into_iter() {
                    validated_block.push(self.typecheck_node(a)?);
                }
                // Add extra return to the end in case of a function that returns void, or it will segfault
                if validated_ret_type == ir::Typing::Void {
                    validated_block.push(ir::Statement::ReturnStatement {
                        value: None,
                        typing: ir::Typing::Void,
                    });
                }

                // Pop the scope
                self.namespace.locals.pop();

                let validated = ir::Function {
                    head: ir::FunctionHead {
                        name: stmt.name.clone(),
                        ret: validated_ret_type,
                        arguments: validated_args,
                        variadic: false, // In this case, variadic is ALWAYS false
                                         // Because Elo is not meant to support variadic functions at all.
                    },
                    block: validated_block,
                };

                // Insert the function into the namespace
                self.namespace
                    .functions
                    .insert(stmt.name, validated.clone());

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
                let validated = ir::Function {
                    head: ir::FunctionHead {
                        name: stmt.name.clone(),
                        ret: validated_ret_type.clone(),
                        arguments: validated_args.clone(),
                        variadic: stmt.variadic,
                    },
                    block: Vec::new(),
                };
                self.namespace
                    .functions
                    .insert(stmt.name.clone(), validated.clone());
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
                let (condition, typing) = self.typecheck_expr(&stmt.condition)?;
                if typing != ir::Typing::Primitive(ir::Primitive::Bool) {
                    return Err(TypeError {
                        span: Some(stmt.condition.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typing),
                            expected: format!("{:?}", ir::Typing::Primitive(ir::Primitive::Bool)),
                        },
                    });
                }
                self.namespace.locals.push(Scope {
                    content: HashMap::new(),
                });
                let mut block_true_content = Vec::new();
                for a in Box::new(stmt.block_true.content).into_iter() {
                    block_true_content.push(self.typecheck_node(a)?);
                }
                self.namespace.locals.pop(); // Pop the true block scope
                self.namespace.locals.push(Scope {
                    // Push a new scope for the false block
                    content: HashMap::new(),
                });
                let mut block_false_content = Vec::new();
                if let Some(block_false) = stmt.block_false {
                    for a in Box::new(block_false.content).into_iter() {
                        block_false_content.push(self.typecheck_node(a)?);
                    }
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
                let (condition, typing) = self.typecheck_expr(&stmt.condition)?;
                if typing != ir::Typing::Primitive(ir::Primitive::Bool) {
                    return Err(TypeError {
                        span: Some(stmt.condition.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typing),
                            expected: format!("{:?}", ir::Typing::Primitive(ir::Primitive::Bool)),
                        },
                    });
                }
                self.namespace.locals.push(Scope {
                    content: HashMap::new(),
                });
                let mut block = Vec::new();
                for a in Box::new(stmt.block.content).into_iter() {
                    block.push(self.typecheck_node(a)?);
                }
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
    pub fn go(mut self) -> Result<ir::Program, TypeError> {
        // This is why i'm making a language
        let mut nodes = Vec::new();
        for node in std::mem::take(&mut self.input.nodes) {
            nodes.push(self.typecheck_node(node)?);
        }
        Ok(ir::Program { nodes })
    }
}
