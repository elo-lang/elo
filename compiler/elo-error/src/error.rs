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
    let file_content = filespan.input_file.content;
    let mut line = file_content.lines().nth(filespan.line - 1);
    if line.is_none() {
        line = Some("")
    }
    let line = line.unwrap();

    let span_length = filespan.end - filespan.start;

    let line_number_digits = filespan.line.to_string().len();
    let indent = " ".repeat(line_number_digits + 2);

    eprintln!("{RED_BOLD}{error}{RESET}: {message}");
    eprintln!(
        "{indent}{CYAN_BOLD}-> {BLUE_BOLD}{}:{}:{}",
        filespan.input_file.filename, filespan.line, filespan.start
    );
    eprintln!("{indent}{CYAN_BOLD}|{RESET}");
    eprintln!(" {} {CYAN_BOLD}|{RESET} {line}", filespan.line);
    eprintln!(
        "{indent}{CYAN_BOLD} {RESET}{}{GREEN_BOLD} ^{} {RESET}",
        " ".repeat(filespan.start),
        "-".repeat(
            if span_length == 0 { 0 } else { span_length - 1 }
        ),
    );
    if submessage.is_some() {
        eprintln!(
            "{indent}{CYAN_BOLD} {RESET}{}{GREEN_BOLD}{RESET} {}",
            " ".repeat(filespan.start),
            submessage.unwrap()
        );
    }
    if let Some(h) = help {
        eprintln!("{indent}{CYAN_BOLD} {RESET}",);
        eprintln!("{indent}{CYAN_BOLD} {RESET}{GREEN_BOLD} Help{RESET}: {h}",);
    }
    eprintln!();
}
