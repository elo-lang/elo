use elo_ast::ast;
use elo_codegen::generator::*;
use elo_error::{parseerror, typeerror};
use elo_ir::ir;
use elo_lexer::{inputfile::InputFile, lexer::Lexer};
use elo_parser::parser::*;
use elo_validation::validation::*;

mod cli;
use crate::cli::*;

use std::collections::HashMap;
use std::env::args;
use std::io::{Read, Write};
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
        Command::Build {
            input,
            output,
            optimization,
        } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_file = InputFile {
                    filename: input.as_str(),
                    content: content.as_str(),
                };
                match parse_program(input_file) {
                    Ok(program) => match validate_program(program) {
                        Ok(validated_program) => {
                            let mut r#gen =
                                elo_codegen::generator::Generator::new(validated_program);
                            r#gen.generate();
                            let output = output.unwrap_or(format!("{input}.out.c"));
                            if let Ok(mut f) = std::fs::File::create(&output) {
                                f.write(r#gen.output.as_bytes()).unwrap();
                            } else {
                                fatal(&args[0], &format!("could not write output file {output}"));
                            }
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
        _ => {}
    }
}
