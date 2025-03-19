use elo_ast::ast;
use elo_ir::ir;
use elo_error::typeerror::{TypeError, TypeErrorCase};

pub struct Validator {
    input: ast::Program,
}

impl Validator {
    pub fn new(input: ast::Program) -> Validator {
        Validator {
            input
        }
    }

    fn validate_type(&self, typ: &ast::Type) -> Result<ir::Typing, TypeError> {
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

    fn validate_expr(&self, expr: &ast::Expression) -> Result<(ir::Expression, ir::Typing), TypeError> {
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
                todo!();
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
                todo!();
            }
        }
    }

    fn validate_node(&self, node: &ast::Node) -> Result<ir::ValidatedNode, TypeError> {
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
                Ok(ir::ValidatedNode {
                    span: node.span,
                    stmt: ir::Statement::ConstStatement(ir::ConstStatement {
                        assignment: expr,
                        binding: name.clone(),
                        typing: typ,
                    })
                })
            }
            ast::Statement::FnStatement(stmt) => {
                todo!();
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
                todo!();
            }
        }
    }

    pub fn validate(self) -> Result<ir::ValidatedProgram, TypeError> {
        let mut nodes = Vec::new();
        for node in self.input.nodes.iter() {
            nodes.push(self.validate_node(node)?);
        }
         Ok(ir::ValidatedProgram {
            nodes: nodes
        })
    }
}