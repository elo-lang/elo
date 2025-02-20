use elo_lexer::{inputfile::InputFile, span::FileSpan};

use crate::error::emit_error;

#[test]
fn test_throw() {
    let file_span = FileSpan {
        line: 1,
        start: 1,
        end: 6,
        input_file: InputFile::new("main.elo", "fn main(){\nlet a = 5;\n}".chars()),
    };
    emit_error("error", &file_span);
}
