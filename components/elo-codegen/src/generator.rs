use elo_ir::ir::{self, ValidatedProgram};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;

pub struct Generator<'a> {
    pub input: ValidatedProgram,
    pub context: &'a Context,
    pub module: Module<'a>,
    pub builder: Builder<'a>,
}

impl<'a> Generator<'a> {
    pub fn foo(&self, t: ir::Typing) -> BasicTypeEnum<'a> {
        match t {
            ir::Typing::Primitive(ir::Primitive::Int) => self.context.i32_type().into(),
            ir::Typing::Primitive(ir::Primitive::UInt) => self.context.i32_type().into(),
            _ => todo!()
        }
    }

    pub fn generate(&mut self) {
        for node in self.input.nodes.iter() {
            match &node.stmt {
                ir::Statement::ConstStatement(stmt) => {
                    let t = self.foo(stmt.typing.clone());
                    match &stmt.typing {
                        ir::Typing::Primitive(ir::Primitive::Int) => {
                            match stmt.assignment {
                                ir::Expression::Integer { value } => {
                                    let const_val = self.context.i32_type().const_int(value.try_into().unwrap(), false);
                                    let global_const = self.module.add_global(self.context.i32_type(), None, &stmt.binding);
                                    global_const.set_initializer(&const_val);
                                    global_const.set_constant(true);
                                }
                                _ => todo!()
                            }
                        }
                        _ => todo!()
                    }
                }
                ir::Statement::FnStatement(Function) => {
                    todo!();
                }
                ir::Statement::StructStatement(Struct) => {
                    todo!();
                }
                ir::Statement::EnumStatement(Enum) => {
                    todo!();
                }
                _ => unreachable!("The parser should have caught this"),
            }
        }
        // let i32_type = self.context.i32_type();
        // let i8_type = self.context.i8_type();

        // // === Declare `main` function ===
        // let main_fn_type = i32_type.fn_type(&[], false);
        // let main_func = self.module.add_function("main", main_fn_type, None);
        // let entry_block = self.context.append_basic_block(main_func, "entry");
        // self.builder.position_at_end(entry_block);
        // let t = BasicTypeEnum::PointerType(i8_type.ptr_type(AddressSpace::default())).into();
        // self.builder.build_malloc::<BasicTypeEnum>(
        //     t,
        //     "x"
        // ).unwrap();
        
        // let bb = self.context.append_basic_block(main_func, "bb");
        // self.builder.build_unconditional_branch(bb).unwrap();
        
        // self.builder.position_at_end(bb);
        // // === Declare `printf` ===
        // let printf_type = i32_type.fn_type(
        //     &[BasicTypeEnum::PointerType(i8_type.ptr_type(AddressSpace::default())).into()],
        //     true, // variadic
        // );
        // let printf = self.module.add_function("printf", printf_type, None);
        
        // // === Build call to printf("%d\n", 42) ===
        // // Allocate [4 x i8]
        // let array_type = i8_type.array_type(4);
        // let format_alloca = self.builder.build_alloca(array_type, "").unwrap();
        
        // // Store the string
        // let string_constant = self.context.const_string(b"%d\n\0", false);
        // self.builder.build_store(format_alloca, string_constant).unwrap();
        
        // // Bitcast [4 x i8]* to i8*
        // let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());
        // let format_ptr = self.builder.build_bit_cast(format_alloca, i8_ptr_type, "casted_fmt")
        // .unwrap();

        // let value_to_print = i32_type.const_int(42, false);
        // self.builder.build_call(printf, &[format_ptr.into(), value_to_print.into()], "printf_call").unwrap();

        // self.builder.build_unconditional_branch(bb).unwrap();
        // // === Return 0 from main ===
        // self.builder.build_return(Some(&i32_type.const_int(0, false))).unwrap();
    }
}
