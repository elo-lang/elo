use crate::filespan::FileSpan;
use crate::inputfile::InputFile;
use crate::token::Token;

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub span: FileSpan<'a>,
}

macro_rules! whitespace {
    () => {
        ' ' | '\r' | '\t' | '\x0b' | '\x0c'
    };
}

macro_rules! delimiter {
    () => {
        '(' | ')' | '[' | ']' | '{' | '}' | '.' | ',' | ';' | ':'
    };
}

macro_rules! op {
    () => {
        '+' | '-' | '/' | '*' | '%' | '!' | '>' | '<' | '&' | '|' | '^' | '~' | '='
    };
}

macro_rules! op_next {
    () => {
        '=' | '+' | '-' | '&' | '|'
    };
}

macro_rules! numeric_start {
    () => {
        '0'..='9'
    };
}

macro_rules! numeric {
    () => {
        '0'..='9' | '.'
    };
}

macro_rules! alphabetic_start {
    () => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! alphabetic {
    () => {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_'
    };
}

impl<'a> Lexer<'a> {
    pub fn new(input_file: InputFile) -> Lexer {
        Lexer {
            input_file: input_file.clone(),
            span: FileSpan::empty(input_file),
        }
    }

    fn advance_span(&mut self, advance_length: usize) {
        self.span.start = self.span.end;
        self.span.end += advance_length;
    }

    fn advance_line(&mut self) {
        self.span.line += 1;
        self.span.start = 0;
        self.span.end = 0;
    }

    pub fn consume_while(&mut self, start: Option<&char>, matches: fn(char) -> bool) -> String {
        let mut buffer = String::new();
        if let Some(start) = start {
            buffer.push(*start);
        }
        while let Some(&c) = self.input_file.content.peek() {
            if !matches(c) {
                break;
            }
            buffer.push(c);
            self.input_file.content.next();
        }
        buffer
    }

    fn token_numeric(&mut self, ch: &char) -> Token {
        let s = self.consume_while(Some(ch), |c| matches!(c, numeric!()));
        self.advance_span(s.len());
        return Token::Numeric(s);
    }
    
    fn token_alphabetic(&mut self, ch: &char) -> Token {
        let s = self.consume_while(Some(&ch), |c| matches!(c, alphabetic!()));
        self.advance_span(s.len());
        return Token::Alphabetic(s);
    }
    
    fn token_op(&mut self, ch: &char) -> Token {
        let next = self.input_file.content.peek();
        let op = match next {
            Some(&b) if matches!(b, op_next!()) => {
                self.advance_span(1);
                self.input_file.content.next();
                Some(b)
            }
            _ => None,
        };
        self.advance_span(1);
        return Token::Op(*ch, op);
    }
    
    fn token_string(&mut self) -> Token {
        let s = self.consume_while(None, |c| c != '"');
        if self.input_file.content.peek() != Some(&'"') {
            panic!("Unterminated string literal");
        }
        self.input_file.content.next(); // Compensate for the last "
        self.advance_span(s.len());
        self.span.end += 2; // Compensate span to get the last "
        return Token::StringLiteral(s);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(ch) = self.input_file.content.next() {
            return match ch {
                '#' => {
                    let _ = self.consume_while(Some(&ch), |c| c != '\n');
                    self.advance_line();
                    self.next()
                }
                '\n' => {
                    self.advance_line();
                    Some(Token::Newline)
                }
                whitespace!() => {
                    self.advance_span(1);
                    self.next()
                }
                alphabetic_start!() => Some(self.token_alphabetic(&ch)),
                numeric_start!() => Some(self.token_numeric(&ch)),
                op!() => Some(self.token_op(&ch)),
                delimiter!() => {
                    self.advance_span(1);
                    Some(Token::Delimiter(ch))
                }
                '"' => Some(self.token_string()),
                _ => {
                    self.advance_span(1);
                    Some(Token::Unknown(ch))
                }
            }
        }
        None
    }
}