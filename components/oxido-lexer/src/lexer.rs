use crate::inputfile::InputFile;
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

macro_rules! numeric {
    () => {
        '0'..='9'
    };
}

macro_rules! identifier_start {
    () => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

macro_rules! identifier {
    () => {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '_'
    };
}

macro_rules! keyword {
    () => {
        "var" | "const" | "let" | "fn" | "struct" | "enum"
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

        if self.input_file.content.peek() == Some(&'.') {
            self.input_file.content.next();
            let s2 = self.consume_while(None, |c| matches!(c, numeric!()));
            if s2.is_empty() {
                panic!("Invalid number literal");
            }

            self.advance_span(s.len() + s2.len() + 1);
            return Token::Numeric(format!("{}.{}", s, s2));
        }

        self.advance_span(s.len());

        return Token::Numeric(s);
    }

    fn consume_numeric(&mut self, ch: &char) -> Token {
        let s = self.consume_while(Some(ch), |c| matches!(c, numeric!()));
        self.advance_span(s.len());
        return Token::Numeric(s);
    }

    fn token_word(&mut self, ch: &char) -> Token {
        let s = self.consume_while(Some(&ch), |c| matches!(c, identifier!()));
        self.advance_span(s.len());

        if matches!(s.as_str(), keyword!()) {
            return Token::Keyword(s);
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
                identifier_start!() => {
                    let token = self.token_word(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                numeric!() => {
                    let token = self.token_numeric(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                op!() => {
                    let token = self.token_op(&ch);
                    Some(Lexem::new(self.span.into_span(), token))
                }
                delimiter!() => {
                    if ch == '.' {
                        if let Some(c) = self.input_file.content.peek() {
                            if matches!(c, numeric!()) {
                                let token = self.consume_numeric(&ch);
                                return Some(Lexem::new(self.span.into_span(), token));
                            }
                        }

                        self.advance_span(1);
                        return Some(Lexem::new(self.span.into_span(), Token::Delimiter(ch)));
                    }
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
