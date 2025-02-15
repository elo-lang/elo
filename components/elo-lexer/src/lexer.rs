use core::panic;

use crate::inputfile::InputFile;
use crate::keyword::Keyword;
use crate::lexem::Lexem;
use crate::span::FileSpan;
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

macro_rules! numeric_first {
    () => {
        '0'..='9'
    };
}

macro_rules! numeric {
    () => {
        '0'..='9' | '_'
    };
}

macro_rules! numeric_binary {
    () => {
        '0' | '1' | '_'
    };
}

macro_rules! numeric_octal {
    () => {
        '0'..='7' | '_'
    };
}

macro_rules! numeric_alphanumeric {
    () => {
        '0'..='9' | '_' | 'a'..='f' | 'A'..='F'
    };
}

macro_rules! identifier_first {
    () => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! identifier {
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
        // TODO: check suffixes like u8, i8, etc?
        let mut number = String::new();
        let mut base = 10;
        if ch == &'0' {
            if let Some(next) = self.input_file.content.peek() {
                match *next {
                    'b' => {
                        base = 2;
                        self.input_file.content.next();
                        number = self.consume_while(
                            None, |c| matches!(c, numeric_binary!())
                        );
                    }
                    'o' => {
                        base = 8;
                        self.input_file.content.next();
                        number = self.consume_while(
                            None, |c| matches!(c, numeric_octal!())
                        );
                    }
                    'x' => {
                        base = 16;
                        self.input_file.content.next();
                        number = self.consume_while(
                            None, |c| matches!(c, numeric_alphanumeric!())
                        );
                    }
                    numeric!() => {
                        base = 10;
                        let int = self.consume_while(
                            Some(ch), |c| matches!(c, numeric!()));
                        self.advance_span(int.len());
                        return Token::Numeric(int, base);
                    }
                    _ => {
                        let int_error = self.consume_while(
                            None, |c| matches!(c, identifier!()));
                            self.advance_span(int_error.len());
                            panic!("error: invalid suffix `{int_error}` for number literal");
                    }
                }
            }
            self.advance_span(2);
            self.span.end += number.len();
            return Token::Numeric(number, base);
        }
        let int = self.consume_while(Some(ch), |c| matches!(c, numeric!()));
        self.advance_span(int.len());
        return Token::Numeric(int, 10);
    }

    fn token_word(&mut self, ch: &char) -> Token {
        let s = self.consume_while(Some(&ch), |c| matches!(c, identifier!()));
        self.advance_span(s.len());

        if let Some(kw) = Keyword::from_str(s.as_str()) {
            return Token::Keyword(kw);
        }

        return Token::Identifier(s);
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
    type Item = Lexem;

    fn next(&mut self) -> Option<Lexem> {
        if let Some(ch) = self.input_file.content.next() {
            return match ch {
                '#' => {
                    let _ = self.consume_while(Some(&ch), |c| c != '\n');
                    self.input_file.content.next(); // Consume \n
                    self.advance_line();
                    self.next()
                }
                '\n' => {
                    self.advance_line();
                    Some(Lexem::new(self.span.into_span(), Token::Newline))
                }
                whitespace!() => {
                    self.advance_span(1);
                    self.next()
                }
                identifier_first!() => {
                    let token = self.token_word(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                numeric_first!() => {
                    let token = self.token_numeric(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                op!() => {
                    let token = self.token_op(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                delimiter!() => {
                    self.advance_span(1);
                    Some(Lexem::new(self.span.into_span(), Token::Delimiter(ch)))
                }
                '"' => {
                    let token = self.token_string();
                    Some(Lexem::new(self.span.into_span(), token))
                }
                _ => {
                    self.advance_span(1);
                    Some(Lexem::new(self.span.into_span(), Token::Unknown(ch)))
                }
            };
        }
        None
    }
}
