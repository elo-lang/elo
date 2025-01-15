pub struct Lexer {
    pub input_file: InputFile,
    pub span: FileSpan,
}

impl Lexer {
    pub fn new(input_file: InputFile) -> Lexer {
        Lexer {
            input_file: input_file.clone(),
            span: FileSpan::empty(input_file),
        }
    }
}