const RED: &str = "\x1b[1;31m";
const CYAN: &str = "\x1b[1;36m";
const YELLOW: &str = "\x1b[1;33m";
const RESET: &str = "\x1b[0m";

pub fn fatal(msg: &str) {
    eprintln!("{RED}fatal{RESET}: {msg}");
}

#[allow(dead_code)]
pub fn warning(msg: &str) {
    eprintln!("{YELLOW}warning{RESET}: {msg}");
}

pub fn information(msg: &str) {
    eprintln!("{CYAN}info{RESET}: {msg}");
}

pub fn critical(msg: &str) {
    eprintln!("{RED}critical{RESET}: {msg}");
}

pub fn usage(program: &str, command: Option<&Command>) {
    if let Some(cmd) = command {
        match cmd {
            Command::Run { .. } => eprintln!("usage: {program} run <input> [...<args>]"),
            Command::Build { .. } => {
                eprintln!("usage: {program} build <input> [-o <output>] [-c]")
            }
            Command::Help { .. } => eprintln!("usage: {program} help [<command>]"),
        }
    } else {
        eprintln!("usage: {program} <command> ...");
    }
}

pub fn help(program: &str, command: Option<&Command>) {
    usage(program, command);
    match command {
        Some(Command::Help { .. }) | None => {
            eprintln!("commands:");
            eprintln!("    run        Run with the given input file");
            eprintln!("    build      Build from given source code");
            eprintln!("    help       Show help message for a specific command or general help");
        }
        Some(Command::Run { .. }) => {
            eprintln!("\nBuild and automatically run the given input file\n");
            eprintln!("positional arguments:");
            eprintln!("    <input>          Source code input file to run");
        }
        Some(Command::Build { .. }) => {
            eprintln!("\nBuild the given source code input file\n");
            eprintln!("positional arguments:");
            eprintln!("    <input>          Source code input file to build");
            eprintln!("flags:");
            eprintln!("    -o <output>      Specify output file");
            eprintln!("    -c               Output C source-code file from the compiled program");
        }
    }
}

pub enum Command {
    Build {
        input: String,
        output: Option<String>,
        c: bool,
    },
    Run {
        input: String,
        args: Vec<String>,
    },
    Help {
        command: Option<String>,
    },
}

impl Command {
    pub fn from_str(command: &str) -> Option<Self> {
        match command {
            "build" | "b" => Some(Command::Build {
                input: String::new(),
                output: None,
                c: false
            }),
            "run" | "r" => Some(Command::Run {
                input: String::new(),
                args: Vec::new(),
            }),
            "help" => Some(Command::Help { command: None }),
            _ => None,
        }
    }
}

fn parse_run(program: &str, args: &[String]) -> Result<Command, ()> {
    if args.len() < 2 {
        return Err(());
    }

    let mut input = None;
    let mut arguments: Vec<String> = Vec::new();

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            _ if input.is_none() => {
                arguments.push(String::from(arg));
                input = Some(String::from(arg));
            }
            arg => {
                arguments.push(String::from(arg));
            }
        }
    }
    if input.is_none() {
        usage(program, Command::from_str("run").as_ref());
        fatal("expected positional argument: <input>");
        return Err(());
    }
    return Ok(Command::Run {
        input: input.unwrap(),
        args: arguments,
    });
}

fn parse_build(program: &str, args: &[String]) -> Result<Command, ()> {
    if args.len() < 2 {
        return Err(());
    }

    let mut input = None;
    let mut output = None;
    let mut c = false;

    let mut i = 2; // Start after the command and program name
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-c" => {
                c = true;
            }
            _ if arg.starts_with("-o") => {
                let rest = arg[2..].to_string();
                if rest.is_empty() {
                    // get the next argument instead
                    if let Some(next_arg) = args.get(i + 1) {
                        output = Some(next_arg.to_string());
                        i += 1; // skip the next argument
                    } else {
                        usage(program, Command::from_str("build").as_ref());
                        fatal("expected output file after `-o` flag");
                        return Err(());
                    }
                } else {
                    output = Some(rest);
                }
            }
            _ if input.is_none() => {
                input = Some(arg.to_string());
            }
            x if input.is_some() => {
                usage(program, Command::from_str("build").as_ref());
                fatal(&format!("unexpected positional argument `{x}`"));
                return Err(());
            }
            _ => {}
        }
        i += 1;
    }

    if input.is_none() {
        usage(program, Command::from_str("build").as_ref());
        fatal("expected positional argument: <input>");
        return Err(());
    }
    Ok(Command::Build {
        input: input.unwrap(),
        output,
        c
    })
}

fn parse_help(args: &[String]) -> Result<Command, ()> {
    Ok(Command::Help {
        command: args.iter().skip(2).next().map(|s| s.to_string()),
    })
}

fn parse_command(program: &str, args: &[String]) -> Result<Command, ()> {
    let command = &args[1];
    match command.as_str() {
        "r" | "run" => parse_run(program, args),
        "b" | "build" => parse_build(program, args),
        "h" | "help" => parse_help(args),
        _ => {
            usage(program, None);
            fatal(&format!("unknown command `{command}`"));
            Err(())
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, ()> {
    if args[1..].is_empty() {
        usage(&args[0], None);
        fatal("expected command");
        information(
            "run with `help` command to see available commands",
        );
        return Err(());
    }

    let program = &args[0];
    match parse_command(program, args) {
        Ok(action) => Ok(action),
        Err(..) => Err(()),
    }
}
