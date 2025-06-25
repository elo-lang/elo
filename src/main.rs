
use elo_lexer::{inputfile::InputFile, lexer::Lexer};
use elo_parser::parser::*;
use elo_validation::validation::*;
use elo_codegen::generator::*;
use elo_ast::ast;
use elo_ir::ir;
use elo_error::{parseerror, typeerror};

mod cli;
use crate::cli::*;

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
    let args: Vec<String> = args().collect();
    let comm = parse_args(&args).unwrap_or_else(|_| {
        cli::error(&args[0], "could not parse command line arguments");
    });

    match comm {
        Command::Build { input, output, optimization } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_file = InputFile {
                    filename: input.as_str(),
                    content: content.as_str(),
                };
                match parse_program(input_file) {
                    Ok(program) => {
                        match validate_program(program) {
                            Ok(validated_program) => {
                                let context = inkwell::context::Context::create();
                                let module = context.create_module(&input);
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
                                let opt_level = match optimization {
                                    cli::O::None => OptimizationLevel::None,
                                    cli::O::Normal => OptimizationLevel::Less,
                                    cli::O::Medium => OptimizationLevel::Default,
                                    cli::O::Aggressive => OptimizationLevel::Aggressive,
                                };
                                let reloc = RelocMode::PIC;
                                let code_model = CodeModel::Default;
                                let target_file_type = FileType::Object;

                                let target_machine = target
                                    .create_target_machine(&triple, cpu, features, opt_level, reloc, code_model)
                                    .expect("Failed to create target machine");
                                
                                let mut path = format!("{}.o", input.clone());
                                if let Some(o) = output { path = o }
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
                cli::error(&args[0], &format!("could not read input file {}", input));
            }
        }
        Command::Help { command } => {
            if let Some(command) = &command {
                help(&args[0], Command::from_str(command).as_ref());
            } else {
                help(&args[0], None);
            }
        }
        _ => {}
    }
}
