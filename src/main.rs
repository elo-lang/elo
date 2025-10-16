mod cli;
mod tcc;

use elo_ir::ir;
use elo_ast::ast;
use elo_error::{parseerror, typeerror};

use elo_lexer::{inputfile::InputFile, lexer::Lexer};
use elo_parser::parser::*;
use elo_validation::validation::*;
use elo_codegen::generator::*;

use crate::cli::*;
use std::env::args;

fn parse_program(p: InputFile) -> Result<ast::Program, parseerror::ParseError> {
    let lexer = Lexer::new(p);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

fn validate_program(prog: ast::Program) -> Result<ir::ValidatedProgram, typeerror::TypeError> {
    let validator = Validator::new(prog);
    validator.validate()
}

fn generate_program(prog: ir::ValidatedProgram) -> String {
    let mut r#gen = Generator::new(prog);
    r#gen.generate();
    return r#gen.output;
}

fn strip_extension(path: String) -> String {
    if let Some(i) = path.find(".") {
        return path.as_str()[..i].to_string();
    } else {
        return path;
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let comm = parse_args(&args).unwrap_or_else(|_| {
        cli::error(&args[0], "could not parse command line arguments");
    });
    let mut tcc = tcc::TCCState::new();
    
    match comm {
        Command::Build {
            input,
            output,
        } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_file = InputFile {
                    filename: input.as_str(),
                    content: content.as_str(),
                };
                match parse_program(input_file) {
                    Ok(program) => match validate_program(program) {
                        Ok(validated_program) => {
                            tcc.set_output_type(tcc::OutputType::Executable);
                            let mut r#gen = Generator::new(validated_program);
                            r#gen.generate();
                            let output = output.unwrap_or(format!("{}.out", strip_extension(input)));
                            tcc.compile_string(&r#gen.output).unwrap();
                            tcc.output_file(&output);
                        }
                        Err(e) => {
                            typeerror::type_error(
                                e.case,
                                &e.span.unwrap().into_filespan(input_file),
                            );
                        }
                    },
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
        Command::Run {
            input,
            args,
        } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_file = InputFile {
                    filename: input.as_str(),
                    content: content.as_str(),
                };
                match parse_program(input_file) {
                    Ok(program) => match validate_program(program) {
                        Ok(validated_program) => {
                            tcc.set_output_type(tcc::OutputType::Memory);
                            let g = generate_program(validated_program);
                            tcc.compile_string(&g).unwrap();
                            tcc.run(&args.iter().map(|x| x.as_str()).collect::<Vec<&str>>())
                        }
                        Err(e) => {
                            typeerror::type_error(
                                e.case,
                                &e.span.unwrap().into_filespan(input_file),
                            );
                        }
                    },
                    Err(e) => {
                        parseerror::parse_error(e.case, &e.span.unwrap().into_filespan(input_file));
                    }
                }
            } else {
                cli::error(&args[0], &format!("could not read input file {}", input));
            }
        }
    }
}
