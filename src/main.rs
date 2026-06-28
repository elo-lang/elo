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

use crate::{cli::*, tcc::TCCState};
use std::env::args;
use std::env;
use std::process::{Command, Output};

#[allow(dead_code)]
enum BackendCompiler {
    CLang { path: String, args: Vec<String> },
    MSVC { path: String, args: Vec<String> },
    TCC(TCCState)
}

impl BackendCompiler {
    pub fn from_system_compiler(compiler: SystemCompiler) -> BackendCompiler {
        match compiler {
            SystemCompiler::CLang(path) => BackendCompiler::CLang { path, args: Vec::new() },
            SystemCompiler::MSVC(path) => BackendCompiler::MSVC { path, args: Vec::new() },
        }
    }
}

enum SystemCompiler {
    CLang(String),
    MSVC(String)
}

fn temporary_file(content: &str, filename: &str, mut with: impl FnMut() -> Result<(), ()>) -> Result<(), ()>  {
    std::fs::write(filename, content).unwrap();
    let result = with();
    let _ = std::fs::remove_file(filename);
    return result;
}

fn setup_elo_backend(compiler: &mut BackendCompiler, _optimize: bool, _debug: bool) {
    match compiler {
        BackendCompiler::CLang { path: _, args } => {
            args.push(String::from("-Irt/include"));
            args.push(String::from("-Lrt/bin"));
            args.push(String::from("-lelort"));
        }
        BackendCompiler::MSVC { path: _, args } => {
            args.push(String::from("/LIBPATH:\"rt/bin\""));
            args.push(String::from("/I\"rt/bin\""));
            args.push(String::from("elort.lib"));
        }
        BackendCompiler::TCC(tcc) => {
            tcc.add_library_path("rt/bin");
            tcc.add_library("elort");
            tcc.set_options("-I rt/include");
        }
    }
}

fn compile_backend_executable(
    cc: BackendCompiler,
    input: &str,
    output: &str,
    lib_search_paths: &[&str],
    libs: &[&str],
) -> Result<(), ()> {
    match cc {
        BackendCompiler::CLang { path: clang_path, mut args } => {
            let filepath = "temp.c";
            temporary_file(input, filepath, || {
                args.extend(vec![
                    filepath.to_string(),
                    "-o".to_string(),
                    output.to_string(),
                ]);
                for i in lib_search_paths {
                    args.push(format!("-L{i}"));
                }
                for i in libs {
                    args.push(format!("-l{i}"));
                }
                let args = args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                let out = invoke_program(&clang_path, &args);
                if let Ok(out) = out {
                    if !out.status.success() {
                        eprintln!("{}", String::from_utf8_lossy(&out.stdout));
                        eprintln!("{}", String::from_utf8_lossy(&out.stderr));
                        return Err(());
                    }
                    return Ok(());
                }
                return Err(());
            })
        }
        BackendCompiler::TCC(mut tcc) => {
            tcc.set_output_type(tcc::OutputType::Executable);
            if tcc.compile_string(input).is_err() {
                return Err(());
            }
            for search in lib_search_paths {
                tcc.add_library_path(search);
            }
            for lib in libs {
                tcc.add_library(lib);
            }
            tcc.output_file(output);
            return Ok(());
        }
        _ => { Err(()) }
    }
}

fn find_program(name: &str) -> Option<String> {
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some((&candidate.to_str()?).to_string());
        } else if candidate.extension().is_none() {
            let candidate_ext = candidate.with_extension("exe");
            if candidate_ext.is_file() {
                return Some((&candidate_ext.to_str()?).to_string());
            }
        }
    }
    None
}

fn invoke_program(program_path: &str, args: &[&str]) -> Result<Output, ()> {
    let out = Command::new(program_path)
                .args(args)
                .output();
    if let Ok(out) = out {
        return Ok(out);
    }
    Err(())
}

fn find_system_backend_compiler() -> Option<SystemCompiler> {
    let candidates = ["clang", "clang-19", "clang-18", "clang-17", "clang-16", "clang-15", "clang-14"];
    for candidate in candidates {
        if let Some(path) = find_program(candidate) {
            return Some(SystemCompiler::CLang(path));
        }
    }
    let candidates = ["cl.exe", "cl"];
    for candidate in candidates {
        if let Some(path) = find_program(candidate) {
            return Some(SystemCompiler::MSVC(path));
        }
    }
    return None;
}

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

fn parse_and_validate(filename: &str, source: &str) -> cir::Program {
    let input_file = InputFile {
        filename,
        content: source,
    };
    match parse_program(input_file) {
        Ok(program) => match validate_program(program) {
            Ok(validated_program) => {
                return validated_program;
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

fn tcc_compile(tcc: &mut tcc::TCCState, source: &str, output: tcc::OutputType) -> Result<(), ()> {
    tcc.set_output_type(output);
    if tcc.compile_string(source).is_err() {
        return Err(());
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = args().collect();
    let comm = parse_args(&args).unwrap_or_else(|_| {
        std::process::exit(-1);
    });

    let mut compiler = BackendCompiler::TCC(tcc::TCCState::new());

    match comm {
        CLICommand::Build { input, output, libs, lib_search_paths, c } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let input_name = strip_extension(&input);
                let program = parse_and_validate(input.as_str(), content.as_str());
                let backend_code = generate_program(program);
                if c {
                    let output_c = output.unwrap_or(format!("{}.c", input_name));
                    std::fs::write(&output_c, backend_code).unwrap();
                    return;
                }

                if let Some(comp) = find_system_backend_compiler() {
                    compiler = BackendCompiler::from_system_compiler(comp);
                } else {
                    warning("could not find CLang or MSVC compilers in your system. Please check their installations");
                    warning("the generated executable will likely not be as performant or lightweight");
                    warning("falling back to internal TCC backend compilation");
                }
                setup_elo_backend(&mut compiler, false, false);
                let output = output.clone().unwrap_or(format!("{}.out", input_name));
                let lib_search_paths = lib_search_paths.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                let libs = libs.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                if let Err(_) = compile_backend_executable(compiler, &backend_code, &output, &lib_search_paths, &libs) {
                    cli::critical("could not compile C backend source-code. This is likely a bug");
                    cli::information("if so, please report the bug at https://github.com/elo-lang/elo/issues");
                    std::process::exit(-1);
                }
            } else {
                cli::fatal(&format!("could not read input file {}", input));
                std::process::exit(-1);
            }
        }
        CLICommand::Help { command } => {
            if let Some(command) = &command {
                help(&args[0], CLICommand::from_str(command).as_ref());
            } else {
                help(&args[0], None);
            }
        }
        CLICommand::Run {
            input,
            args: arguments,
        } => {
            if let Some(content) = std::fs::read_to_string(&input).ok() {
                let validated_program = parse_and_validate(input.as_str(), content.as_str());
                let g = &generate_program(validated_program);
                setup_elo_backend(&mut compiler, false, false);
                if let BackendCompiler::TCC(mut tcc) = compiler {
                    if let Err(_) = tcc_compile(&mut tcc, g, tcc::OutputType::Memory) {
                        cli::critical("could not compile C backend source-code. This is likely a bug");
                        cli::information("if so, please report the bug at https://github.com/elo-lang/elo/issues");
                        std::process::exit(-1);
                    }
                    let arguments =
                        arguments.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
                    let code = tcc.run(&arguments);
                    std::process::exit(code);
                }
                unreachable!("At this point, TCC is mandatory");
            } else {
                cli::fatal(&format!("could not read input file {}", input));
                std::process::exit(-1);
            }
        }
    }
}
