use std::collections::HashMap;

use elo_ir::ir::{self, ValidatedProgram};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};
use inkwell::AddressSpace;

pub struct Generator<'a> {
    pub input: ValidatedProgram,
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
    pub variables: HashMap<String, inkwell::values::PointerValue<'a>>,
}

impl<'a> Generator<'a> {
    pub fn choose_type(&self, t: ir::Typing) -> Option<BasicTypeEnum<'a>> {
        return match t {
            ir::Typing::Primitive(ir::Primitive::I128) => Some(self.context.i128_type().into()),
            ir::Typing::Primitive(ir::Primitive::I64) => Some(self.context.i64_type().into()),
            ir::Typing::Primitive(ir::Primitive::I32) => Some(self.context.i32_type().into()),
            ir::Typing::Primitive(ir::Primitive::I16) => Some(self.context.i16_type().into()),
            ir::Typing::Primitive(ir::Primitive::I8) => Some(self.context.i8_type().into()),
            ir::Typing::Primitive(ir::Primitive::Bool) => Some(self.context.bool_type().into()),
            ir::Typing::Primitive(ir::Primitive::U128) => Some(self.context.i128_type().into()),
            ir::Typing::Primitive(ir::Primitive::U64) => Some(self.context.i64_type().into()),
            ir::Typing::Primitive(ir::Primitive::U32) => Some(self.context.i32_type().into()),
            ir::Typing::Primitive(ir::Primitive::U16) => Some(self.context.i16_type().into()),
            ir::Typing::Primitive(ir::Primitive::U8) => Some(self.context.i8_type().into()),
            ir::Typing::Primitive(ir::Primitive::F32) => Some(self.context.f32_type().into()),
            ir::Typing::Primitive(ir::Primitive::F64) => Some(self.context.f64_type().into()),
            // TODO: Make these int and uint to have the same size as the target architecture
            ir::Typing::Primitive(ir::Primitive::Int) => Some(self.context.i32_type().into()),
            ir::Typing::Primitive(ir::Primitive::UInt) => Some(self.context.i32_type().into()),
            // TODO: Make float have the size as the target architecture
            ir::Typing::Primitive(ir::Primitive::Float) => Some(self.context.f64_type().into()),
            ir::Typing::Primitive(ir::Primitive::Str) => Some(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            ir::Typing::Void => None,
            _ => todo!()
        }
    }

    pub fn generate_expression(&mut self, expr: &ir::Expression) -> Option<BasicValueEnum<'a>> {
        match expr {
            ir::Expression::Integer { value } => {
                let const_val = self.context.i32_type().const_int((*value).try_into().unwrap(), false);
                return Some(const_val.into());
            }
            ir::Expression::Float { value } => {
                let const_val = self.context.f32_type().const_float(*value);
                return Some(const_val.into());
            }
            ir::Expression::StringLiteral { value } => {
                let const_val = self.context.const_string(value.as_bytes(), false);
                return Some(const_val.into());
            }
            ir::Expression::BinaryOperation { operator, left, right } => {
                let lhs = self.generate_expression(left).unwrap();
                let rhs = self.generate_expression(right).unwrap();
                match *operator {
                    ir::BinaryOperation::Add => {
                        let x = self.builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "add").unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Sub => {
                        let x = self.builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "sub").unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Mul => {
                        let x = self.builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "mul").unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Div => {
                        let x = self.builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "div").unwrap();
                        return Some(x.into());
                    }
                    _ => todo!()
                }
            }
            ir::Expression::FunctionCall { function, arguments } => {
                match function.as_ref() {
                    ir::Expression::Identifier { name } => {
                        let function = self.module.get_function(&name).unwrap();
                        let mut arg_vals: Vec<BasicMetadataValueEnum<'_>> = Vec::new();
                        for arg in arguments {
                            let arg = self.generate_expression(arg).unwrap();
                            arg_vals.push(arg.into());
                        }
                        let call_site = self.builder.build_call(function, arg_vals.as_slice(), "static_call").unwrap();
                        // NOTE: The type-checking pass should have ensured that the function has a return type
                        return call_site.try_as_basic_value().left();
                    }
                    _ => todo!()
                }
            }
            ir::Expression::Identifier { name } => {
                if let Some(var) = self.variables.get(name) {
                    return Some(self.builder.build_load(*var, name).unwrap().into());
                }
                unreachable!("unreachable point at compile-time: variable {} not found", name);
            }
            _ => todo!()
        }
    }

    pub fn generate_from_node(&mut self, node: &mut ir::ValidatedNode, toplevel: bool) {
        match &mut node.stmt {
            ir::Statement::ConstStatement(stmt) => {
                match &stmt.assignment {
                    ir::Expression::Integer { value } => {
                        let const_val = self.context.i32_type().const_int((*value).try_into().unwrap(), false);
                        let global_const = self.module.add_global(self.context.i32_type(), None, &stmt.binding);
                        global_const.set_initializer(&const_val);
                        global_const.set_constant(true);
                    }
                    ir::Expression::Float { value } => {
                        let const_val = self.context.f32_type().const_float(*value);
                        let global_const = self.module.add_global(self.context.f32_type(), None, &stmt.binding);
                        global_const.set_initializer(&const_val);
                        global_const.set_constant(true);
                    }
                    ir::Expression::StringLiteral { value } => {
                        let const_val = self.context.const_string(value.as_bytes(), false);
                        let global_const = self.module.add_global(self.context.i8_type().array_type(value.as_bytes().len() as u32), None, &stmt.binding);
                        global_const.set_initializer(&const_val);
                        global_const.set_constant(true);
                    }
                    _ => todo!()
                }
            }
            ir::Statement::FnStatement(stmt) => {
                let fn_type;
                if let Some(t) = self.choose_type(stmt.ret.clone()) {
                    fn_type = t.fn_type(&[], false);
                } else {
                    fn_type = self.context.void_type().fn_type(&[], false);
                }
                let function = self.module.add_function(&stmt.name, fn_type, None);
                let entry_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry_block);
                for mut i in std::mem::take(&mut stmt.block.content) {
                    self.generate_from_node(&mut i, false);
                }
            }
            ir::Statement::ExternFnStatement(stmt) => {
                let fn_type;
                let args: Vec<BasicMetadataTypeEnum<'_>> = stmt.arguments.iter().map(|arg| {
                    if let Some(t) = self.choose_type(arg.typing.clone()) {
                        t.into()
                    } else {
                        unreachable!()
                    }
                }).collect::<Vec<_>>();
                if let Some(t) = self.choose_type(stmt.ret.clone()) {
                    fn_type = t.fn_type(args.as_slice(), false);
                } else {
                    fn_type = self.context.void_type().fn_type(args.as_slice(), false);
                }
                self.module.add_function(&stmt.name, fn_type, None);
            }
            ir::Statement::StructStatement(_stmt) => {
                todo!();
            }
            ir::Statement::EnumStatement(_stmt) => {
                todo!();
            }
            ir::Statement::LetStatement(stmt) if !toplevel => {
                let t = self.choose_type(stmt.typing.clone()).unwrap();
                let local = self.builder.build_alloca(t, &stmt.binding).unwrap();
                let expr = self.generate_expression(&stmt.assignment);
                self.builder.build_store(local, expr.unwrap()).unwrap();
                self.variables.insert(stmt.binding.clone(), local);
            }
            ir::Statement::VarStatement(stmt) if !toplevel => {
                let t = self.choose_type(stmt.typing.clone()).unwrap();
                let local = self.builder.build_alloca(t, &stmt.binding).unwrap();
                let expr = self.generate_expression(&stmt.assignment);
                self.builder.build_store(local, expr.unwrap()).unwrap();
                self.variables.insert(stmt.binding.clone(), local);
            }
            ir::Statement::ExpressionStatement(expr) => {
                self.generate_expression(expr);
            }
            ir::Statement::ReturnStatement(stmt) => {
                let expr = self.generate_expression(&stmt.value).unwrap();
                assert_eq!(expr.get_type(), self.context.i32_type().into());
                self.builder.build_return(Some(&expr)).unwrap();
            }
            _ => todo!(),
        }
    }

    pub fn generate(&mut self) {
        for mut node in std::mem::take(&mut self.input.nodes) {
            self.generate_from_node(&mut node, true);
        }
    }
}
