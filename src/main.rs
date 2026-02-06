mod cli;
mod tcc;
#[cfg(test)]
mod tests;

use elo_error::{parseerror, semerror};
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
    return r#gen.head + &r#gen.body;
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
                let error_amount = es.len() as i32;
                for e in es {
                    match e {
                        ValidationError::SemanticChecker(e) => {
                            semerror::semantic_error(
                                e.case,
                                &e.span.into_filespan(input_file),
                            );
                        }
                    }
                }
                std::process::exit(error_amount);
            },
        },
        Err(e) => {
            parseerror::parse_error(e.case, &e.span.into_filespan(input_file));
            std::process::exit(1);
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
        Command::Build { input, output, c, h } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_name = strip_extension(&input);
                let output_exe = output.unwrap_or(format!("{}.out", input_name));
                let output_c = format!("{}.c", input_name);
                let output_h = format!("{}.h", input_name);
                parse_and_validate(input.as_str(), content.as_str(), |validated_program| {
                    tcc.set_output_type(tcc::OutputType::Executable);
                    let mut r#gen = Generator::new(validated_program);
                    r#gen.go();
                    let generated_output = &format!("{}{}", r#gen.head, r#gen.body);
                    if c {
                        if h {
                            std::fs::write(&output_c, r#gen.body).unwrap();
                        } else {
                            std::fs::write(&output_c, generated_output).unwrap();
                        }
                    }
                    if h {
                        std::fs::write(&output_h, r#gen.head).unwrap();
                    }
                    if !c && !h {
                        if tcc.compile_string(generated_output).is_err() {
                            cli::critical(&args[0], "could not compile C backend source-code. This is likely a bug");
                            cli::information(&args[0], "if so, please report the bug at https://github.com/elo-lang/elo/issues");
                            std::process::exit(-1);
                        }
                        tcc.output_file(&output_exe);
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
                    if tcc.compile_string(&g).is_err() {
                        cli::critical(&args[0], "could not compile C backend source-code. This is likely a bug");
                        cli::information(&args[0], "if so, please report the bug at https://github.com/elo-lang/elo/issues");
                        std::process::exit(-1);
                    }
                    let arguments =
                        arguments.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                    std::process::exit(tcc.run(&arguments));
                });
            } else {
                cli::error(&args[0], &format!("could not read input file {}", input));
            }
        }
    }
}
