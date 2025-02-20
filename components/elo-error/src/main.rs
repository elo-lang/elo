use elo_error::error::emit_error;
use elo_lexer::{inputfile::InputFile, span::FileSpan};

fn main() {
    let file_span = FileSpan {
        line: 2,
        start: 5,
        end: 12,
        input_file: InputFile::new("main.elo", "fn main(){\nlet a = 5;\n}".chars()),
    };
    emit_error("Parse Error", "unexpected token", &file_span);
}
