use std::fs::read_to_string;

use elo_lexer::span::FileSpan;

use crate::parseerror::ParseErrorCase;

pub fn emit_error(
    error_type: &str,
    error_message: &str,
    file_span: &FileSpan,
    //help: Option<&str>
) {
    let mut line_with_error: &str = "";
    let file = read_to_string(file_span.input_file.filename).unwrap();

    for (i, line) in file.lines().enumerate() {
        if i + 1 == file_span.line {
            line_with_error = line;
            break;
        }
    }

    let line_with_error_len = file_span.end - file_span.start;

    const RED_BOLD: &str = "\x1b[1;31m";
    const CYAN_BOLD: &str = "\x1b[1;36m";
    const BLUE_BOLD: &str = "\x1b[1;34m";
    const GREEN_BOLD: &str = "\x1b[1;32m";
    const RESET: &str = "\x1b[0m";

    // ─┬
    eprintln!(
        "{RED_BOLD}{error_type}{RESET}: {error_message}\
        \n   {CYAN_BOLD}╭─[{BLUE_BOLD}{}{RESET}:{GREEN_BOLD}{}{RESET}:{GREEN_BOLD}{}{CYAN_BOLD}]{RESET}\
        \n   {CYAN_BOLD}│{RESET}\
        \n {CYAN_BOLD}{} │{RESET} {line_with_error}\
        \n   {CYAN_BOLD}·{RESET}{}{}┬{}\n   {CYAN_BOLD}·{RESET}{}╰──── here\
        \n{CYAN_BOLD}───╯{RESET}",
        file_span.input_file.filename,
        file_span.line,
        file_span.start,
        file_span.line,
        " ".repeat(file_span.start),
        "─".repeat(line_with_error_len / 2),
        "─".repeat(if line_with_error_len % 2 != 0 {
            line_with_error_len / 2
        } else {
            line_with_error_len / 2 - 1
        }),
        " ".repeat(file_span.start + line_with_error_len / 2),
    )
}

pub fn parse_error(pe: ParseErrorCase, file_span: &FileSpan) {
    match pe {
        ParseErrorCase::UnexpectedToken { got, expected } => {
            emit_error(
                "Parse Error",
                &format!("unexpected token while parsing: expected {expected} but got {got}"),
                file_span,
            );
        }
    }
}
