use elo_lexer::{inputfile::InputFile, span::FileSpan};

use crate::error::error;

#[test]
fn test_error() {
    let filespan = FileSpan {
        input_file: InputFile {
            filename: "test.txt",
            content: "Hello, World!",
        },
        start: 0,
        end: 13,
        line: 1,
    };
    error(
        "Parse Error",
        "message",
        &filespan,
        Some("help"),
        Some("submessage"),
    );
}
