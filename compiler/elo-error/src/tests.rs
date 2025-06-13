use elo_lexer::{inputfile::InputFile, span::FileSpan};

use crate::error::error;

#[test]
fn test_error() {
    let filespan = FileSpan {
        input_file: InputFile {
            filename: "test.txt",
            content: "let x = 6 + 6.7;",
        },
        line: 1, start: 8, end: 15
    };
    error(
        "Type Error",
        "unexpected type",
        &filespan,
        None,
        Some("expected Int but found F64"),
    );
}
