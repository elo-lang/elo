use elo_ast::ast::UnaryOperation;
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
            ir::Statement::StructStatement(Struct) => {
                todo!();
            }
            ir::Statement::EnumStatement(Enum) => {
                todo!();
            }
            ir::Statement::LetStatement(stmt) if !toplevel => {
                let t = self.choose_type(stmt.typing.clone()).unwrap();
                let local = self.builder.build_alloca(t, &stmt.binding).unwrap();
                // store
                match &stmt.assignment {
                    ir::Expression::Integer { value } => {
                        let const_val = self.context.i32_type().const_int((*value).try_into().unwrap(), false);
                        self.builder.build_store(local, const_val).unwrap();
                    }
                    _ => todo!()
                }
            }
            ir::Statement::ExpressionStatement(expr) => {
                match &expr {
                    ir::Expression::FunctionCall { function, arguments } => {
                        match &**function {
                            ir::Expression::Identifier { name } => {
                                let function = self.module.get_function(&name).unwrap();
                                let mut arg_vals: Vec<BasicMetadataValueEnum<'_>> = Vec::new();
                                for arg in arguments {
                                    match arg {
                                        ir::Expression::Integer { value } => {
                                            let const_val = self.context.i32_type().const_int((*value).try_into().unwrap(), false);
                                            arg_vals.push(const_val.into());
                                        }
                                        _ => todo!()
                                    }
                                }
                                self.builder.build_call(function, arg_vals.as_slice(), "call").unwrap();
                            }
                            _ => todo!()
                        }
                    }
                    _ => todo!()
                }
            }
            ir::Statement::ReturnStatement(stmt) => {
                match &stmt.value {
                    ir::Expression::Integer { value } => {
                        let const_val = self.context.i32_type().const_int((*value).try_into().unwrap(), false);
                        self.builder.build_return(Some(&const_val)).unwrap();
                    }
                    _ => todo!()
                }
            }
            _ => unreachable!("The parser should have caught this"),
        }
    }

    pub fn generate(&mut self) {
        for mut node in std::mem::take(&mut self.input.nodes) {
            self.generate_from_node(&mut node, true);
        }
    }
}
