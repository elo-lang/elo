use core::panic;
use std::iter::Peekable;
use std::str::Chars;

use crate::inputfile::InputFile;
use crate::keyword::Keyword;
use crate::lexem::Lexem;
use crate::span::FileSpan;
use crate::token::Token;

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub chars: Peekable<Chars<'a>>,
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

macro_rules! numeric_hex {
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
    pub fn new(input_file: InputFile<'a>) -> Lexer<'a> {
        Lexer {
            input_file: input_file.clone(),
            chars: input_file.content.chars().peekable(),
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
        while let Some(&c) = self.chars.peek() {
            if !matches(c) {
                break;
            }
            buffer.push(c);
            self.chars.next();
        }
        buffer
    }

    fn token_numeric(&mut self, ch: &char) -> Token {
        // TODO: check suffixes like u8, i8, etc?
        if ch == &'0' {
            if let Some(c) = self.chars.peek() {
                match c {
                    'b' => {
                        self.chars.next();
                        self.advance_span(2);
                        let number = self.consume_while(None, |c| matches!(c, numeric_binary!()));
                        self.span.end += number.len();
                        return Token::Numeric(number, 2);
                    }
                    'o' => {
                        self.chars.next();
                        self.advance_span(2);
                        let number = self.consume_while(None, |c| matches!(c, numeric_octal!()));
                        self.span.end += number.len();
                        return Token::Numeric(number, 8);
                    }
                    'x' => {
                        self.chars.next();
                        self.advance_span(2);
                        let number = self.consume_while(None, |c| matches!(c, numeric_hex!()));
                        self.span.end += number.len();
                        return Token::Numeric(number, 16);
                    }
                    _ => {}
                }
            }
            self.advance_span(1);
            return Token::Numeric(String::from("0"), 10);
        }
        let number = self.consume_while(Some(ch), |c| matches!(c, numeric!()));
        self.advance_span(number.len());
        return Token::Numeric(number, 10);
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
        let next = self.chars.peek();
        let op = match next {
            Some(&b) if matches!(b, op_next!()) => {
                self.advance_span(1);
                self.chars.next();
                Some(b)
            }
            _ => None,
        };
        self.advance_span(1);
        return Token::Op(*ch, op);
    }

    fn token_string(&mut self) -> Token {
        let s = self.consume_while(None, |c| c != '"');
        if self.chars.peek() != Some(&'"') {
            panic!("Unterminated string literal");
        }
        self.chars.next(); // Compensate for the last "
        self.advance_span(s.len());
        self.span.end += 2; // Compensate span to get the last "
        return Token::StringLiteral(s);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexem;

    fn next(&mut self) -> Option<Lexem> {
        if let Some(ch) = self.chars.next() {
            return match ch {
                '#' => {
                    let _ = self.consume_while(Some(&ch), |c| c != '\n');
                    self.chars.next(); // Consume \n
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
