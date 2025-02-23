use elo_lexer::{inputfile::InputFile, span::FileSpan};

use crate::error::error;

#[test]
fn test_error() {
    let filespan = FileSpan {
        input_file: InputFile {
            filename: "test.txt",
            content: "fn is_even(\na\n: int): {\n }",
        },
        line: 3,
        start: 8,
        end: 9,
    };
    error(
        "Parse Error",
        "message",
        &filespan,
        Some("help"),
        Some("submessage"),
    );
}
