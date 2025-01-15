use oxido_ir::filespan::FileSpan;
use oxido_ir::inputfile::InputFile;
use oxido_ir::token::Token;

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

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        todo!();
    }
}