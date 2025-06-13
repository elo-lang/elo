use crate::namespace::*;
use elo_ast::ast::{self, ExpressionData};
use elo_ir::ir::{self, Typing};
use elo_error::typeerror::{TypeError, TypeErrorCase};
use std::collections::HashMap;

pub struct Validator {
    input: ast::Program,
    namespace: Namespace,
}

impl Validator {
    pub fn new(input: ast::Program) -> Validator {
        Validator {
            input,
            namespace: Namespace {
                name: None,
                structs: HashMap::new(),
                enums: HashMap::new(),
                constants: HashMap::new(),
                functions: HashMap::new(),
                locals: Vec::new(),
            }
        }
    }

    fn validate_type(&mut self, typ: &ast::Type) -> Result<ir::Typing, TypeError> {
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
                    case: TypeErrorCase::InvalidType { what: format!("{:?}", typ.typing) }
                });
            }
            x => Err(TypeError {
                span: Some(typ.span),
                case: TypeErrorCase::InvalidType { what: format!("{:?}", x) }
            }),
        }
    }

    fn validate_expr(&mut self, expr: &ast::Expression) -> Result<(ir::Expression, ir::Typing), TypeError> {
        match &expr.data {
            ast::ExpressionData::BinaryOperation { operator, left, right } => {
                let operator = ir::BinaryOperation::from_ast(operator);
                let (left, left_type) = self.validate_expr(&left)?;
                let (right, right_type) = self.validate_expr(&right)?;
                // TODO: Improve this type-checking PLEASE
                if left_type != right_type {
                    return Err(TypeError {
                        span: Some(expr.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", right_type),
                            expected: format!("{:?}", left_type), 
                        }
                    });
                }
                Ok((
                    ir::Expression::BinaryOperation { operator, left: Box::new(left), right: Box::new(right) },
                    right_type,
                ))
            }
            ast::ExpressionData::UnaryOperation { operator, operand } => {
                let operator = ir::UnaryOperation::from_ast(operator);
                let (operand, operand_type) = self.validate_expr(&operand)?;
                Ok((
                    ir::Expression::UnaryOperation { operator, operand: Box::new(operand) },
                    operand_type,
                ))
            }
            ast::ExpressionData::StringLiteral { value } => {
                return Ok((
                    ir::Expression::StringLiteral { value: value.clone() },
                    ir::Typing::Primitive(ir::Primitive::Str),
                ));
            }
            ast::ExpressionData::Tuple { exprs: _exprs } => {
                todo!();
            }
            ast::ExpressionData::FieldAccess { origin: _origin, field: _field } => {
                todo!();
            }
            ast::ExpressionData::FunctionCall { function, arguments } => {
                if let ExpressionData::Identifier { name } = &function.data {
                    if let Some(func) = self.namespace.functions.get(name) {
                        let arguments_to_check: Vec<Typing> = func.arguments.iter().map(|x| x.typing.clone()).collect();
                        let len_args = func.arguments.len();
                        let return_type = func.ret.clone();
                        if arguments.len() != len_args {
                            return Err(TypeError {
                                span: Some(function.span),
                                case: TypeErrorCase::UnmatchedArguments {
                                    name: name.clone(),
                                    got: arguments.len(),
                                    expected: len_args
                                }
                            });
                        }
                        let mut validated_args = Vec::new();
                        for (expr, expected_type) in arguments.iter().zip(arguments_to_check) {
                            let span = expr.span;
                            let (validated, got_type) = self.validate_expr(expr)?;
                            if got_type != expected_type {
                                return Err(TypeError {
                                    span: Some(span),
                                    case: TypeErrorCase::TypeMismatch {
                                        got: format!("{:?}", got_type),
                                        expected: format!("{:?}", expected_type),
                                    }
                                });
                            }
                            validated_args.push(validated);
                        }
                        return Ok((
                            ir::Expression::FunctionCall {
                                function: Box::new(self.validate_expr(function)?.0),
                                arguments: validated_args
                            },
                            return_type.clone()
                        ));
                    } else {
                        return Err(TypeError {
                            span: Some(expr.span),
                            case: TypeErrorCase::UnresolvedName { name: name.clone() }
                        })
                    }
                } else {
                    return Err(TypeError {
                        span: Some(expr.span),
                        case: TypeErrorCase::InvalidExpression {
                            what: format!("{:?}", &function.data),
                            should: "identifier".to_string()
                        },
                    })
                }
            }
            ast::ExpressionData::StructInit { name: _name, fields: _fields } => {
                todo!();
            }
            ast::ExpressionData::IntegerLiteral { value } => {
                let (lit, radix) = value;
                Ok((
                    ir::Expression::Integer { value: i128::from_str_radix(lit, *radix).unwrap() },
                    ir::Typing::Primitive(ir::Primitive::Int),
                ))
            }
            ast::ExpressionData::FloatLiteral { int, float } => {
                let integer = u64::from_str_radix(&int.0, int.1).unwrap() as f64;
                let fractional = u64::from_str_radix(&float.0, float.1).unwrap() as f64;
                let number = integer + (fractional / 10u32.pow(float.0.chars().count() as u32) as f64);
                Ok((
                    ir::Expression::Float { value: number },
                    ir::Typing::Primitive(ir::Primitive::Float),
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => {
                Ok((
                    ir::Expression::Bool { value: *value },
                    ir::Typing::Primitive(ir::Primitive::Bool),
                ))
            }
            ast::ExpressionData::Identifier { name } => {
                if let Some(typ) = self.namespace.constants.get(name) {
                    return Ok((ir::Expression::Identifier { name: name.clone() }, typ.clone()))
                } else if let Some(e) = self.namespace.enums.get(name) {
                    return Ok((ir::Expression::Identifier { name: name.clone() }, ir::Typing::Enum(e.clone())))
                } else if let Some(v) = self.namespace.locals.last().unwrap().content.get(name) {
                    return Ok((ir::Expression::Identifier { name: name.clone() }, v.typing.clone()))
                } else if let Some(f) = self.namespace.functions.get(name) {
                    return Ok((ir::Expression::Identifier { name: name.clone() }, f.ret.clone()))
                } else {
                    return Err(TypeError {
                        span: Some(expr.span),
                        case: TypeErrorCase::UnresolvedName { name: name.clone() }
                    });
                }
            }
        }
    }

    fn validate_node(&mut self, node: ast::Node) -> Result<ir::ValidatedNode, TypeError> {
        match node.stmt {
            ast::Statement::LetStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.validate_expr(assignment)?;
                
                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().content.insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
                        mutable: false,
                        typing: typ.clone(),
                    }
                );

                Ok(ir::ValidatedNode {
                    stmt: ir::Statement::LetStatement(ir::LetStatement {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    })
                })
            }
            ast::Statement::VarStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.validate_expr(assignment)?;

                // Add the variable to the current scope
                self.namespace.locals.last_mut().unwrap().content.insert(
                    name.clone(),
                    Variable {
                        name: name.clone(),
                        mutable: true,
                        typing: typ.clone(),
                    }
                );

                Ok(ir::ValidatedNode {
                    stmt: ir::Statement::VarStatement(ir::VarStatement {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    })
                })
            }
            ast::Statement::ConstStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.validate_expr(assignment)?;
                let annotated = self.validate_type(&stmt.typing)?;
                if annotated != typ {
                    return Err(TypeError {
                        span: Some(stmt.typing.span),
                        case: TypeErrorCase::TypeMismatch {
                            got: format!("{:?}", typ),
                            expected: format!("{:?}", annotated), 
                        }
                    });
                }
                self.namespace.constants.insert(name.clone(), typ.clone());
                Ok(ir::ValidatedNode {
                    stmt: ir::Statement::ConstStatement(ir::ConstStatement {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    })
                })
            }
            ast::Statement::ReturnStatement(stmt) => {
                let (expr, typ) = self.validate_expr(&stmt.expr)?;
                Ok(ir::ValidatedNode {
                    stmt: ir::Statement::ReturnStatement(ir::ReturnStatement { value: expr, typing: typ })
                })
            }
            ast::Statement::FnStatement(stmt) => {
                // TODO: Add type-checking
                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push(ir::TypedField {
                        name: a.name.clone(),
                        typing: self.validate_type(&a.typing)?
                    });
                }

                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.validate_type(ret_type)?,
                    None => ir::Typing::Void,
                };
                let mut validated_block = ir::Block { content: Vec::new() };
                let xs = Box::new(stmt.block.content);
                
                // Create a new scope for the function
                // TODO: Add the arguments to the scope
                self.namespace.locals.push(Scope {
                    content: HashMap::new(),
                });
                
                for a in xs.into_iter() {
                    validated_block.content.push(self.validate_node(a)?);
                }

                // Pop the scope
                self.namespace.locals.pop();
                
                let validated = ir::Function {
                    name: stmt.name.clone(),
                    block: validated_block,
                    ret: validated_ret_type,
                    arguments: validated_args,
                };

                // Insert the function into the namespace
                self.namespace.functions.insert(stmt.name, validated.clone());
                
                return Ok(
                    ir::ValidatedNode {
                        stmt: ir::Statement::FnStatement(validated)
                    }
                );
            }
            ast::Statement::ExternFnStatement(stmt) => {
                // TODO: Add type-checking
                let mut validated_args = Vec::new();
                for a in stmt.arguments.iter() {
                    validated_args.push(ir::TypedField {
                        name: a.name.clone(),
                        typing: self.validate_type(&a.typing)?
                    });
                }
                let validated_ret_type = match &stmt.ret {
                    Some(ret_type) => self.validate_type(ret_type)?,
                    None => ir::Typing::Void,
                };
                let validated = ir::Function {
                    name: stmt.name.clone(),
                    block: ir::Block { content: Vec::new() },
                    ret: validated_ret_type.clone(),
                    arguments: validated_args.clone(),
                };
                self.namespace.functions.insert(stmt.name.clone(), validated.clone());
                return Ok(
                    ir::ValidatedNode {
                        stmt: ir::Statement::ExternFnStatement(ir::FunctionHead {
                            name: stmt.name,
                            ret: validated_ret_type,
                            arguments: validated_args,
                        })
                    }
                );
            }
            ast::Statement::StructStatement(_stmt) => {
                todo!();
            }
            ast::Statement::EnumStatement(stmt) => {
                let e = ir::Enum {
                    name: stmt.name,
                    variants: stmt.variants,
                };
                self.namespace.enums.insert(e.name.clone(), e.clone());
                return Ok(ir::ValidatedNode {
                    stmt: ir::Statement::EnumStatement(e)
                })
            }
            ast::Statement::IfStatement(_stmt) => {
                // TODO: Remember to push a new scope to the namespace
                todo!();
            }
            ast::Statement::WhileStatement(_stmt) => {
                // TODO: Remember to push a new scope to the namespace
                todo!();
            }
            ast::Statement::ExpressionStatement(stmt) => {
                return Ok(
                    ir::ValidatedNode {
                        stmt: ir::Statement::ExpressionStatement(self.validate_expr(&stmt)?.0)
                    }
                )
            }
        }
    }

    // Type-check and transform the AST into the IR of Elo code
    pub fn validate(mut self) -> Result<ir::ValidatedProgram, TypeError> {
        // This is why i'm making a language
        let mut nodes = Vec::new();
        for node in std::mem::take(&mut self.input.nodes) {
            nodes.push(self.validate_node(node)?);
        }
        Ok(ir::ValidatedProgram { nodes })
    }
}