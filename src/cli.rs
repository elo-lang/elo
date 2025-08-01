pub fn fatal(program: &str, msg: &str) {
    eprintln!("{program}: fatal: {msg}");
}

pub fn warning(program: &str, msg: &str) {
    eprintln!("{program}: warning: {msg}");
}

pub fn information(program: &str, msg: &str) {
    eprintln!("{program}: info: {msg}");
}

pub fn usage(program: &str, command: Option<&Command>) {
    if let Some(cmd) = command {
        match cmd {
            Command::Run { .. } => eprintln!("usage: {program} run <input>"),
            Command::Build { .. } => {
                eprintln!("usage: {program} build <input> [-o <output>] [-O0|-O1|-O2|-O3]")
            }
            Command::Help { .. } => eprintln!("usage: {program} help [<command>]"),
        }
    } else {
        eprintln!("usage: {program} <command> ...");
    }
}

pub fn error(program: &str, msg: &str) -> ! {
    eprintln!("{program}: error: {msg}");
    std::process::exit(1);
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
            eprintln!("    -O0              Set optimization level to 0");
            eprintln!("    -O1              Set optimization level to 1 (default)");
            eprintln!("    -O2              Set optimization level to 2");
            eprintln!("    -O3              Set optimization level to 3");
        }
    }
}

pub enum O {
    None,       // -O0
    Normal,     // -O1
    Medium,     // -O2
    Aggressive, // -O3
}

pub enum Command {
    Build {
        input: String,
        output: Option<String>,
        optimization: O,
    },
    Run {
        input: String,
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
                optimization: O::Normal,
            }),
            "run" | "r" => Some(Command::Run {
                input: String::new(),
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

    let input = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            _ if input.is_none() => {
                return Ok(Command::Run {
                    input: arg.to_string(),
                });
            }
            _ if input.is_some() => {
                usage(program, Command::from_str("run").as_ref());
                fatal(program, &format!("unexpected positional argument `{arg}`"));
                return Err(());
            }
            _ => {
                usage(program, Command::from_str("run").as_ref());
                fatal(program, "expected positional argument: <input>");
                return Err(());
            }
        }
    }

    if input.is_none() {
        usage(program, Command::from_str("build").as_ref());
        fatal(program, "expected positional argument: <input>");
        return Err(());
    }
    Ok(Command::Run {
        input: input.unwrap(),
    })
}

fn parse_build(program: &str, args: &[String]) -> Result<Command, ()> {
    if args.len() < 2 {
        return Err(());
    }

    let mut input = None;
    let mut output = None;
    let mut optimization = O::Normal;

    let mut i = 2; // Start after the command and program name
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-O0" => optimization = O::None,
            "-O1" => optimization = O::Normal,
            "-O2" => optimization = O::Medium,
            "-O3" => optimization = O::Aggressive,
            _ if arg.starts_with("-o") => {
                let rest = arg[2..].to_string();
                if rest.is_empty() {
                    // get the next argument instead
                    if let Some(next_arg) = args.get(i + 1) {
                        output = Some(next_arg.to_string());
                        i += 1; // skip the next argument
                    } else {
                        usage(program, Command::from_str("build").as_ref());
                        fatal(program, "expected output file after `-o` flag");
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
                fatal(program, &format!("unexpected positional argument `{x}`"));
                return Err(());
            }
            _ => {}
        }
        i += 1;
    }

    if input.is_none() {
        usage(program, Command::from_str("build").as_ref());
        fatal(program, "expected positional argument: <input>");
        return Err(());
    }
    Ok(Command::Build {
        input: input.unwrap(),
        output,
        optimization,
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
            fatal(program, &format!("unknown command `{command}`"));
            Err(())
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, ()> {
    if args[1..].is_empty() {
        usage(&args[0], None);
        fatal(&args[0], "expected command");
        information(
            &args[0],
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
