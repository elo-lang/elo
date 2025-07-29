use std::collections::HashMap;

use elo_ir::ir::{self, ValidatedProgram};
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

#[derive(Debug)]
pub struct Namespace<'a> {
    pub variables: HashMap<String, inkwell::values::PointerValue<'a>>,
}

pub struct Generator<'a> {
    pub input: ValidatedProgram,
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
    pub namespace: Namespace<'a>,
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
            ir::Typing::Primitive(ir::Primitive::Str) => Some(
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into(),
            ),
            ir::Typing::Pointer { typ } => Some(
                self.choose_type(*typ)
                    .unwrap()
                    .ptr_type(AddressSpace::default())
                    .into(),
            ),
            ir::Typing::Void => None,
            _ => todo!(),
        };
    }

    // NOTE: I'm using an option here in case of a function call that returns void,
    // which is not a valid value in LLVM unfortunately.
    pub fn generate_expression(&mut self, expr: &ir::Expression) -> Option<BasicValueEnum<'a>> {
        match expr {
            ir::Expression::Integer { value } => {
                let const_val = self
                    .context
                    .i32_type()
                    .const_int((*value).try_into().unwrap(), false);
                return Some(const_val.into());
            }
            ir::Expression::Float { value } => {
                let const_val = self.context.f64_type().const_float(*value);
                return Some(const_val.into());
            }
            ir::Expression::StringLiteral { value } => {
                // TODO: Make this a sized string, for now it's a null-terminated string for compatibility with C
                //       See elo-validation too for the *u8 typing
                let const_val = self
                    .context
                    .const_string((value.to_owned() + "\0").as_bytes(), false);
                let global_const = self.module.add_global(
                    self.context
                        .i8_type()
                        .array_type(value.as_bytes().len() as u32),
                    None,
                    "STR",
                );
                global_const.set_initializer(&const_val);
                global_const.set_constant(true);

                // Create a pointer to the string in the global constant
                let str = self
                    .builder
                    .build_bit_cast(
                        global_const.as_pointer_value(),
                        self.context.i8_type().ptr_type(AddressSpace::default()),
                        "",
                    )
                    .unwrap();
                return Some(str.into());
            }
            ir::Expression::Bool { value } => {
                let const_val = self
                    .context
                    .bool_type()
                    .const_int(if *value { 1 } else { 0 }, false);
                return Some(const_val.into());
            }
            ir::Expression::UnaryOperation { operator, operand } => {
                let op = self.generate_expression(operand).unwrap();
                match *operator {
                    ir::UnaryOperation::Neg => {
                        let x = self
                            .builder
                            .build_int_neg(op.into_int_value(), "-")
                            .unwrap();
                        return Some(x.into());
                    }
                    ir::UnaryOperation::BNot | ir::UnaryOperation::Not => {
                        let x = self.builder.build_not(op.into_int_value(), "not").unwrap();
                        return Some(x.into());
                    }
                    ir::UnaryOperation::Addr => {
                        let x = self.builder.build_alloca(op.get_type(), "addr").unwrap();
                        self.builder.build_store(x, op).unwrap();
                        return Some(x.into());
                    } // TODO: Implement dereference operator
                      // *x syntax to load a value from a pointer
                      // ir::UnaryOperation::Deref => {
                      //     let x = self.builder.build_load(op.into_pointer_value(), "addr").unwrap();
                      //     return Some(x.into());
                      // }
                }
            }
            ir::Expression::BinaryOperation {
                operator,
                left,
                right,
            } => {
                let lhs = self.generate_expression(left).unwrap();
                let rhs = self.generate_expression(right).unwrap();
                match *operator {
                    ir::BinaryOperation::Add => {
                        let x = self
                            .builder
                            .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "add")
                            .unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Sub => {
                        let x = self
                            .builder
                            .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "sub")
                            .unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Mul => {
                        let x = self
                            .builder
                            .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "mul")
                            .unwrap();
                        return Some(x.into());
                    }
                    ir::BinaryOperation::Div => {
                        let x = self
                            .builder
                            .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "div")
                            .unwrap();
                        return Some(x.into());
                    }
                    _ => todo!(),
                }
            }
            ir::Expression::FunctionCall {
                function,
                arguments,
            } => {
                match function.as_ref() {
                    ir::Expression::Identifier { name } => {
                        let function = self.module.get_function(&name).unwrap();
                        let mut arg_vals: Vec<BasicMetadataValueEnum<'_>> = Vec::new();
                        for arg in arguments {
                            let arg = self.generate_expression(arg).unwrap();
                            arg_vals.push(arg.into());
                        }
                        let call_site = self
                            .builder
                            .build_call(function, arg_vals.as_slice(), "static_call")
                            .unwrap();
                        // NOTE: The type-checking pass should have ensured that the function has a return type
                        return call_site.try_as_basic_value().left();
                    }
                    _ => todo!(),
                }
            }
            ir::Expression::Identifier { name } => {
                if let Some(var) = self.namespace.variables.get(name) {
                    return Some(self.builder.build_load(*var, name).unwrap().into());
                }
                // TODO: In case of a function name, it should return the function pointer
                unreachable!(
                    "unreachable point at compile-time: variable {} not found",
                    name
                );
            }
            _ => todo!(),
        }
    }

    pub fn generate_from_node(
        &mut self,
        function: Option<inkwell::values::FunctionValue<'_>>,
        node: &mut ir::ValidatedNode,
        toplevel: bool,
    ) {
        match &mut node.stmt {
            ir::Statement::Constant { value, binding, typing } => match value {
                ir::Expression::Integer { value } => {
                    let const_val = self
                        .context
                        .i32_type()
                        .const_int((*value).try_into().unwrap(), false);
                    let global_const =
                        self.module
                            .add_global(self.context.i32_type(), None, binding);
                    global_const.set_initializer(&const_val);
                    global_const.set_constant(true);
                }
                ir::Expression::Float { value } => {
                    let const_val = self.context.f32_type().const_float(*value);
                    let global_const =
                        self.module
                            .add_global(self.context.f32_type(), None, binding);
                    global_const.set_initializer(&const_val);
                    global_const.set_constant(true);
                }
                ir::Expression::StringLiteral { value } => {
                    let const_val = self.context.const_string(value.as_bytes(), false);
                    let global_const = self.module.add_global(
                        self.context
                            .i8_type()
                            .array_type(value.as_bytes().len() as u32),
                        None,
                        binding,
                    );
                    global_const.set_initializer(&const_val);
                    global_const.set_constant(true);
                }
                _ => todo!(),
            },
            ir::Statement::FnStatement(stmt) => {
                let fn_type;
                let mut parameters = vec!{};
                for arg in &stmt.arguments {
                    if let Some(t) = self.choose_type(arg.typing.clone()) {
                        parameters.push(t.into());
                        continue;
                    }
                    unreachable!();
                }
                if let Some(t) = self.choose_type(stmt.ret.clone()) {
                    fn_type = t.fn_type(parameters.as_slice(), false);
                } else {
                    fn_type = self.context.void_type().fn_type(parameters.as_slice(), false);
                }
                let function = self.module.add_function(&stmt.name, fn_type, None);
                let entry_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry_block);
                for (i, arg) in function.get_params().iter().enumerate() {
                    let name = &stmt.arguments[i].name;
                    let argument = self.builder.build_alloca(arg.get_type(), &name).unwrap();
                    self.builder.build_store(argument, *arg).unwrap();
                    self.namespace.variables.insert(
                        name.clone(),
                        argument,
                    );
                }
                for mut i in std::mem::take(&mut stmt.block.content) {
                    self.generate_from_node(Some(function), &mut i, false);
                }
            }
            ir::Statement::ExternFnStatement(stmt) => {
                let fn_type;
                let args: Vec<BasicMetadataTypeEnum<'_>> = stmt
                    .arguments
                    .iter()
                    .map(|arg| {
                        if let Some(t) = self.choose_type(arg.typing.clone()) {
                            t.into()
                        } else {
                            unreachable!()
                        }
                    })
                    .collect::<Vec<_>>();
                if let Some(t) = self.choose_type(stmt.ret.clone()) {
                    fn_type = t.fn_type(args.as_slice(), stmt.variadic);
                } else {
                    fn_type = self
                        .context
                        .void_type()
                        .fn_type(args.as_slice(), stmt.variadic);
                }
                self.module.add_function(&stmt.name, fn_type, None);
            }
            ir::Statement::StructStatement(_stmt) => {
                todo!();
            }
            ir::Statement::EnumStatement(_stmt) => {
                todo!();
            }
            ir::Statement::Variable { binding, assignment, typing, mutable } if !toplevel => {
                let t = self.choose_type(typing.clone()).unwrap();
                let local = self.builder.build_alloca(t, binding).unwrap();
                let expr = self.generate_expression(assignment);
                self.builder.build_store(local, expr.unwrap()).unwrap();
                self.namespace.variables.insert(binding.clone(), local);
            }
            ir::Statement::ExpressionStatement(expr) => {
                self.generate_expression(expr);
            }
            ir::Statement::ReturnStatement { value, typing } if !toplevel => {
                if let Some(value) = value {
                    let expr = self.generate_expression(value).unwrap();
                    self.builder.build_return(Some(&expr)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
            }
            ir::Statement::IfStatement {
                condition,
                block_true,
                block_false,
            } => {
                let branch = self.context.append_basic_block(function.unwrap(), "branch");
                let other = self.context.append_basic_block(function.unwrap(), "other");
                let escape = self.context.append_basic_block(function.unwrap(), "esc");
                let comparison = self.generate_expression(&condition).unwrap();
                self.builder
                    .build_conditional_branch(comparison.into_int_value(), branch, other)
                    .unwrap();
                self.builder.position_at_end(branch);
                for mut i in std::mem::take(&mut block_true.content) {
                    self.generate_from_node(Some(function.unwrap()), &mut i, false);
                }
                self.builder.build_unconditional_branch(escape).unwrap();
                self.builder.position_at_end(other);
                for mut i in std::mem::take(&mut block_false.content) {
                    self.generate_from_node(Some(function.unwrap()), &mut i, false);
                }
                self.builder.build_unconditional_branch(escape).unwrap();
                self.builder.position_at_end(escape);
            }
            _ => todo!(),
        }
    }

    pub fn generate(&mut self) {
        for mut node in std::mem::take(&mut self.input.nodes) {
            self.generate_from_node(None, &mut node, true);
        }
    }
}
