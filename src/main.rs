
use elo_lexer::{inputfile::InputFile, lexer::Lexer};
use elo_parser::parser::*;
use elo_validation::validation::*;
use elo_codegen::generator::*;
use elo_ast::ast;
use elo_ir::ir;
use elo_error::{parseerror, typeerror};

use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;

use std::env::args;
use std::collections::HashMap;
use std::path::Path;

fn parse_program(p: InputFile) -> Result<ast::Program, parseerror::ParseError> {
    let lexer = Lexer::new(p);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

fn validate_program(prog: ast::Program) -> Result<ir::ValidatedProgram, typeerror::TypeError> {
    let validator = Validator::new(prog);
    validator.validate()
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if let Some(content) = std::fs::read_to_string(filename).ok() {
        let input_file = InputFile {
            filename: filename.as_str(),
            content: content.as_str(),
        };
        match parse_program(input_file) {
            Ok(program) => {
                match validate_program(program) {
                    Ok(validated_program) => {
                        let context = inkwell::context::Context::create();
                        let module = context.create_module(filename);
                        let mut r#gen = Generator {
                            input: validated_program,
                            context: &context,
                            module: module,
                            builder: context.create_builder(),
                            namespace: Namespace {
                                locals: HashMap::new(),
                                constants: HashMap::new(),
                            },
                        };
                        r#gen.generate();
                        Target::initialize_native(&InitializationConfig::default())
                            .expect("Failed to initialize native target");

                        let triple = TargetMachine::get_default_triple();
                        let target = Target::from_triple(&triple).unwrap();
                        let cpu = "generic";
                        let features = "";
                        let opt_level = OptimizationLevel::Aggressive;
                        let reloc = RelocMode::PIC;
                        let code_model = CodeModel::Default;
                        let target_file_type = FileType::Object;

                        let target_machine = target
                            .create_target_machine(&triple, cpu, features, opt_level, reloc, code_model)
                            .expect("Failed to create target machine");

                        let path = format!("{}.o", filename);
                        let path = Path::new(&path);
                        target_machine
                            .write_to_file(&r#gen.module, target_file_type, &path)
                            .expect("Failed to write object file");
                    }
                    Err(e) => {
                        typeerror::type_error(e.case, &e.span.unwrap().into_filespan(input_file));
                    }
                }
            }
            Err(e) => {
                parseerror::parse_error(e.case, &e.span.unwrap().into_filespan(input_file));
            }
        }
    } else {
        eprintln!("error: could not read file {}", filename);
        std::process::exit(1);
    }
}
