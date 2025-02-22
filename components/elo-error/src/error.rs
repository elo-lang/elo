use elo_lexer::span::FileSpan;

const RED_BOLD: &str = "\x1b[1;31m";
const CYAN_BOLD: &str = "\x1b[1;36m";
const BLUE_BOLD: &str = "\x1b[1;34m";
const GREEN_BOLD: &str = "\x1b[1;32m";
const RESET: &str = "\x1b[0m";

pub fn error(
    error: &str,
    message: &str,
    filespan: &FileSpan,
    help: Option<&str>,
    submessage: Option<&str>,
) {
    let mut line: &str = "";

    let file_content = filespan.input_file.content;

    for (i, l) in file_content.lines().enumerate() {
        if i + 1 == filespan.line {
            line = l;
            break;
        }
    }

    let span_length = filespan.end - filespan.start;

    let line_number_digits = filespan.line.to_string().len();
    let indent_n = line_number_digits + 2;
    let indent = " ".repeat(line_number_digits + 2);

    eprintln!("{RED_BOLD}{error}{RESET}: {message}");
    eprintln!(
        "{indent}{CYAN_BOLD}╭─[{BLUE_BOLD}{}{RESET}:{GREEN_BOLD}{}{RESET}:{GREEN_BOLD}{}{CYAN_BOLD}]{RESET}",
        filespan.input_file.filename,
        filespan.line,
        filespan.start
    );
    eprintln!("{indent}{CYAN_BOLD}│{RESET}");
    eprintln!(" {} {CYAN_BOLD}│{RESET} {line}", filespan.line);
    eprintln!(
        "{indent}{CYAN_BOLD}·{RESET}{}{GREEN_BOLD}╰{}┬{}╯{RESET}",
        " ".repeat(filespan.start),
        "─".repeat(span_length / 2),
        "─".repeat(if span_length % 2 != 0 {
            span_length / 2
        } else {
            span_length / 2 - 1
        })
    );
    eprintln!(
        "{indent}{CYAN_BOLD}·{RESET}{}{GREEN_BOLD}{}─{RESET} {}",
        " ".repeat((filespan.start + span_length / 2) + 1),
        if help.is_some() { "├" } else { "╰" },
        if submessage.is_some() {
            submessage.unwrap()
        } else {
            "here"
        }
    );
    if let Some(h) = help {
        eprintln!(
            "{indent}{CYAN_BOLD}·{RESET}{}{GREEN_BOLD}╰─ Help{RESET}: {h}",
            " ".repeat((filespan.start + span_length / 2) + 1)
        );
    }
    eprintln!("{CYAN_BOLD}{}╯{RESET}", "─".repeat(indent_n));
}
