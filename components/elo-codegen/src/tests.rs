use elo_lexer::inputfile::InputFile;
use elo_lexer::lexer::Lexer;
use elo_parser::parser::Parser;
use elo_validation::validation::Validator;

use inkwell::targets::{InitializationConfig, Target, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;

#[test]
fn test_file() {
    use std::fs::read_to_string;
    let filename = "test.elo";
    let source_text = &read_to_string(filename).unwrap();
    let lx = Lexer::new(InputFile::new("test.rs", source_text));
    
    let prog = Parser::new(lx).parse().unwrap();
    let val = Validator::new(prog).validate().unwrap();
    let context = inkwell::context::Context::create();
    let module = context.create_module("test");
    let mut r#gen = crate::generator::Generator {
        input: val,
        context: &context,
        module: module,
        builder: context.create_builder(),
    };
    r#gen.generate();

    println!("{}", r#gen.module.to_string());
    Target::initialize_native(&InitializationConfig::default())
    .expect("Failed to initialize native target");

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).unwrap();
    let cpu = "generic";
    let features = "";
    let opt_level = OptimizationLevel::Default;
    let reloc = RelocMode::PIC;
    let code_model = CodeModel::Default;

    let target_machine = target
        .create_target_machine(&triple, cpu, features, opt_level, reloc, code_model)
        .expect("Failed to create target machine");

    use std::path::Path;

    let path = Path::new("output.o");
    target_machine
      .write_to_file(&r#gen.module, FileType::Object, &path)
      .expect("Failed to write object file");
}