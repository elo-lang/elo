mod cli;
mod tcc;
#[cfg(test)]
mod tests;

use elo_error::{parseerror, typeerror};
use elo_ir::*;

use elo_codegen::generator::*;
use elo_lexer::{inputfile::InputFile, lexer::Lexer};
use elo_parser::parser::*;
use elo_validation::validation::{self, *};

use crate::cli::*;
use std::env::args;

fn parse_program(p: InputFile) -> Result<ast::Program, parseerror::ParseError> {
    let lexer = Lexer::new(p);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

fn validate_program(prog: ast::Program) -> Result<cir::Program, Vec<validation::ValidationError>> {
    let validator = Validator::new();
    validator.go(prog.nodes)
}

fn generate_program(prog: cir::Program) -> String {
    let mut r#gen = Generator::new(prog);
    r#gen.go();
    return r#gen.output;
}

fn strip_extension(path: &str) -> String {
    if let Some(i) = path.find(".") {
        return path[..i].to_string();
    } else {
        return path.to_string();
    }
}

fn parse_and_validate(filename: &str, source: &str, mut callback: impl FnMut(cir::Program)) {
    let input_file = InputFile {
        filename,
        content: source,
    };
    match parse_program(input_file) {
        Ok(program) => match validate_program(program) {
            Ok(validated_program) => {
                callback(validated_program);
            }
            Err(es) => {
                for e in es {
                    match e {
                        ValidationError::TypeChecking(e) => {
                            typeerror::type_error(
                                e.case,
                                &e.span.into_filespan(input_file),
                            );
                        }
                    }
                }
            },
        },
        Err(e) => {
            parseerror::parse_error(e.case, &e.span.into_filespan(input_file));
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let comm = parse_args(&args).unwrap_or_else(|_| {
        cli::error(&args[0], "could not parse command line arguments");
    });
    let mut tcc = tcc::TCCState::new();

    match comm {
        Command::Build { input, output, c } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_name = strip_extension(&input);
                let output = output.unwrap_or(format!("{}.out", input_name));
                parse_and_validate(input.as_str(), content.as_str(), |validated_program| {
                    tcc.set_output_type(tcc::OutputType::Executable);
                    let mut r#gen = Generator::new(validated_program);
                    r#gen.go();
                    if !c {
                        tcc.compile_string(&r#gen.output).unwrap();
                        tcc.output_file(&output);
                    } else {
                        std::fs::write(&output, &r#gen.output).unwrap();
                    }
                });
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
            args: arguments,
        } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                parse_and_validate(input.as_str(), content.as_str(), |validated_program| {
                    tcc.set_output_type(tcc::OutputType::Memory);
                    let g = generate_program(validated_program);
                    tcc.compile_string(&g).unwrap();
                    let arguments =
                        arguments.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                    tcc.run(&arguments);
                });
            } else {
                cli::error(&args[0], &format!("could not read input file {}", input));
            }
        }
    }
}
