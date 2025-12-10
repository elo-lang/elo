use std::{collections::HashMap, process::Output};

#[derive(Debug, Clone)]
struct Test {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    return_code: i32,
}

fn parse_test_file_header(content: &String) -> Test {
    let mut props = HashMap::new();
    for line in content.lines() {
        let line = line.trim_start();
        if line.starts_with("//") {
            // get the property name
            let mut line = line.chars().peekable();
            let prop = line
                .by_ref()
                .skip(2) // skip '//'
                .take_while(|x| *x != '=')
                .collect::<String>();
            let value = line.collect::<String>(); // skip : and then collect the value
            props.insert(prop.trim().to_string(), value.trim().to_string());
        }
    }
    return Test {
        stdout: unescape_string(&props.get("stdout").unwrap_or(&String::from("")).to_string())
            .into_bytes(),
        stderr: unescape_string(&props.get("stderr").unwrap_or(&String::from("")).to_string())
            .into_bytes(),
        return_code: props
            .get("return_code")
            .map(|x| x.parse::<i32>().unwrap())
            .unwrap_or(0),
    };
}

fn read_test_file(path: &str) -> Test {
    let content = std::fs::read_to_string(path).unwrap(); // assuming the file exists
    return parse_test_file_header(&content);
}

fn build_compiler() {
    match std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: Could not build compiler due to error: {e}");
            std::process::exit(1);
        }
    }
}

fn test_file(path: &str, output: Output, test: Test) -> bool {
    let mut success = true;
    if output.stdout != test.stdout {
        eprintln!("{}: unexpected stdout:", path);
        eprintln!(
            "   expected: {}",
            escape_string(&String::from_utf8_lossy(&test.stdout))
        );
        eprintln!(
            "   actual: {}",
            escape_string(&String::from_utf8_lossy(&output.stdout))
        );
        success = false;
    };
    if output.stderr != test.stderr {
        eprintln!("{}: unexpected stderr:", path);
        eprintln!(
            "   expected: {}",
            escape_string(&String::from_utf8_lossy(&test.stderr))
        );
        eprintln!(
            "   actual: {}",
            escape_string(&String::from_utf8_lossy(&output.stderr))
        );
        success = false;
    }
    if output.status.code().unwrap() != test.return_code {
        eprintln!("{}: unexpected return status code:", path);
        eprintln!(
            "   expected code {}, but got {}",
            test.return_code, output.status
        );
        success = false;
    }
    return success;
}

fn unescape_string(string: &str) -> String {
    let mut result = String::new();
    let mut escape = false;
    for ch in string.chars() {
        if ch == '\\' {
            escape = true;
            continue;
        }
        if escape && ch == 'n' {
            result.push('\n');
            continue;
        }
        if escape && ch == 't' {
            result.push('\t');
            continue;
        }
        if escape && ch == 'r' {
            result.push('\r');
            continue;
        }
        result.push(ch);
    }
    result
}

fn escape_string(string: &str) -> String {
    let mut result = String::new();
    for ch in string.chars() {
        if ch == '\n' {
            result.push_str("\\n");
        } else if ch == '\t' {
            result.push_str("\\t");
        } else if ch == '\r' {
            result.push_str("\\r");
        } else {
            result.push(ch);
        }
    }
    result
}

fn run_test_file(path: &str) -> Result<Output, ()> {
    match std::process::Command::new("./target/release/elo")
        .arg("run")
        .arg(path)
        .output()
    {
        Ok(out) => {
            return Ok(out);
        }
        Err(e) => {
            eprintln!(
                "ERROR: Could not run test file `{}` due to error: {e}",
                path
            );
            return Err(());
        }
    }
}

const TESTS_DIR: &'static str = "examples/tests/";

#[test]
fn main() {
    let tests_directory = std::path::Path::new(TESTS_DIR);
    if !tests_directory.is_dir() || !tests_directory.exists() {
        eprintln!(
            "Tests path {} does not exist or is not a directory",
            tests_directory.display()
        );
        std::process::exit(1);
    }

    build_compiler();

    for entry in std::fs::read_dir(tests_directory).unwrap() {
        let entry = entry.unwrap();
        let fname = entry.path().to_string_lossy().to_string();
        if fname.ends_with(".elo") {
            let test = read_test_file(&fname);
            if let Ok(out) = run_test_file(&fname) {
                assert!(test_file(&fname, out, test));
            } else {
                std::process::exit(1);
            }
        }
    }
}
