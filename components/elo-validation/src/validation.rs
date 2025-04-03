use std::{collections::HashMap, env::consts::EXE_SUFFIX, iter::Zip, sync::Arc};

use elo_ast::ast::{self, ExpressionData};
use elo_ir::ir::{self, Typing, ValidatedNode};
use elo_error::typeerror::{TypeError, TypeErrorCase};

pub struct Validator {
    input: ast::Program,
    structs: HashMap<String, ir::Struct>,
    enums: HashMap<String, ir::Enum>,
    constants: HashMap<String, ir::Typing>,
    fns: HashMap<String, ir::Function>
}

impl Validator {
    pub fn new(input: ast::Program) -> Validator {
        Validator {
            input,
            structs: HashMap::new(),
            enums: HashMap::new(),
            constants: HashMap::new(),
            fns: HashMap::new(),
        }
    }

    fn validate_type(&mut self, typ: &ast::Type) -> Result<ir::Typing, TypeError> {
        match &typ.typing {
            // TODO: Add generics
            ast::Typing::Named { name, .. } => {
                if let Some(t) = ir::Primitive::from_str(name) {
                    return Ok(ir::Typing::Primitive(t));
                }
                // TODO: Handle other cases of Named: Struct and Enum 
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
                    ir::Typing::Tuple { types: vec![
                        ir::Typing::Pointer { typ: Box::new(ir::Typing::Primitive(ir::Primitive::U8)) },
                        ir::Typing::Primitive(ir::Primitive::UInt),
                    ] },
                ));
            }
            ast::ExpressionData::Tuple { exprs } => {
                todo!();
            }
            ast::ExpressionData::FieldAccess { origin, field } => {
                todo!();
            }
            ast::ExpressionData::FunctionCall { function, arguments } => {
                if let ExpressionData::Identifier { name } = &function.data {
                    if let Some(func) = self.fns.get(name) {
                        let arguments_to_check: Vec<Typing> = func.arguments.iter().map(|x| x.typing.clone()).collect();
                        let len_args = func.arguments.len();
                        let return_type = func.ret.as_ref().unwrap_or(&ir::Typing::Void).clone();
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
            ast::ExpressionData::StructInit { name, fields } => {
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
                    ir::Typing::Primitive(ir::Primitive::F64),
                ))
            }
            ast::ExpressionData::BooleanLiteral { value } => {
                Ok((
                    ir::Expression::Bool { value: *value },
                    ir::Typing::Primitive(ir::Primitive::Bool),
                ))
            }
            ast::ExpressionData::Identifier { name } => {
                if let Some(typ) = self.constants.get(name) {
                    return Ok((ir::Expression::Identifier { name: name.clone() }, typ.clone()))
                }
                return Ok((ir::Expression::Identifier { name: name.clone() }, ir::Typing::Void))
            }
        }
    }

    fn validate_node(&mut self, node: &ast::Node) -> Result<ir::ValidatedNode, TypeError> {
        match &node.stmt {
            ast::Statement::LetStatement(stmt) => {
                let assignment = &stmt.assignment;
                let name = &stmt.binding;
                let (expr, typ) = self.validate_expr(assignment)?;
                Ok(ir::ValidatedNode {
                    span: node.span,
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
                Ok(ir::ValidatedNode {
                    span: node.span,
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
                self.constants.insert(name.clone(), typ.clone());
                Ok(ir::ValidatedNode {
                    span: node.span,
                    stmt: ir::Statement::ConstStatement(ir::ConstStatement {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    })
                })
            }
            ast::Statement::ReturnStatement(stmt) => {
                todo!()
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
                    Some(ret_type) => Some(self.validate_type(ret_type)?),
                    None => None,
                };
                let mut validated_block = ir::Block { content: Vec::new() };
                for a in stmt.block.content.iter() {
                    validated_block.content.push(self.validate_node(a)?);
                }
                let validated = ir::Function {
                    name: stmt.name.clone(),
                    block: validated_block,
                    ret: validated_ret_type,
                    arguments: validated_args,
                };
                self.fns.insert(stmt.name.clone(), validated.clone());
                return Ok(
                    ir::ValidatedNode {
                        span: node.span,
                        stmt: ir::Statement::FnStatement(validated)
                    }
                );
            }
            ast::Statement::StructStatement(stmt) => {
                todo!();
            }
            ast::Statement::EnumStatement(stmt) => {
                todo!();
            }
            ast::Statement::IfStatement(stmt) => {
                todo!();
            }
            ast::Statement::WhileStatement(stmt) => {
                todo!();
            }
            ast::Statement::ExpressionStatement(stmt) => {
                return Ok(
                    ir::ValidatedNode {
                        span: stmt.span,
                        stmt: ir::Statement::ExpressionStatement(self.validate_expr(stmt)?.0)
                    }
                )
            }
        }
    }

    pub fn validate(mut self) -> Result<ir::ValidatedProgram, TypeError> {
        // This is why i'm making a language
        let mut nodes = Vec::new();
        for node in std::mem::take(&mut self.input.nodes) {
            nodes.push(self.validate_node(&node)?);
        }
        Ok(ir::ValidatedProgram { nodes })
    }
}