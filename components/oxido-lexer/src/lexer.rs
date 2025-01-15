use oxido_ir::filespan::FileSpan;
use oxido_ir::inputfile::InputFile;
use oxido_ir::token::Token;

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub span: FileSpan<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_file: InputFile) -> Lexer {
        Lexer {
            input_file: input_file.clone(),
            span: FileSpan::empty(input_file),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.input_file.content.next() {
            match c {
                '\n' => {
                    self.span.line += 1;
                    self.span.start = 0;
                    self.span.end = 0;
                    return Some(Token::Newline);
                }
                _ => {
                    self.span.start += 1;
                    self.span.end = self.span.start + 1;
                    return Some(Token::Unknown(c))
                }
            }
        }
        None
    }
}