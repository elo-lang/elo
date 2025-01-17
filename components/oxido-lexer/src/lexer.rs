use oxido_ir::filespan::FileSpan;
use oxido_ir::inputfile::InputFile;
use oxido_ir::token::Token;
use oxido_ir::word::Word;

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub span: FileSpan<'a>,
}

pub fn is_token_whitespace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}

pub fn is_token_newline(c: char) -> bool {
    c == '\n'
}

pub fn is_token_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_token_numeric(c: char) -> bool {
    c.is_digit(10)
}

pub fn is_token_comma(c: char) -> bool {
    c == ','
}

pub fn is_token_op(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' => true,
        _ => false,
    }
}

pub fn is_token_delimiter(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '{' | '}' | '.' | ';' | ':' => true,
        _ => false,
    }
}

pub fn is_token_string_literal(c: char) -> bool {
    c == '"'
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

    pub fn consume_until(&mut self, start: Option<&char>, predicate: fn(char) -> bool) -> String {
        let mut buffer = String::new();
        if let Some(start) = start {
            buffer.push(*start);
        }
        while let Some(&c) = self.input_file.content.peek() {
            if predicate(c) {
                break;
            }
            buffer.push(c);
            self.input_file.content.next();
        }
        buffer
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.input_file.content.next() {
            match c {
                '#' => {
                    // Ignore comments
                    let _ = self.consume_until(Some(&c), is_token_newline);
                    self.advance_line();
                    return self.next();
                }
                '\n' => {
                    self.advance_line();
                    return Some(Token::Newline);
                }
                a if is_token_alphabetic(a) => {
                    let s = self.consume_until(Some(&c), |c| !is_token_alphabetic(c));
                    self.advance_span(s.len());
                    return Some(Token::Alphabetic(Word(s)));
                }
                a if is_token_whitespace(a) => {
                    self.advance_span(1);
                    return self.next();
                }
                a if is_token_numeric(a) => {
                    let s = self.consume_until(Some(&c), |c| !is_token_numeric(c) || is_token_delimiter(c));
                    self.advance_span(s.len());
                    return Some(Token::Numeric(Word(s)));
                }
                a if is_token_comma(a) => {
                    self.advance_span(1);
                    return Some(Token::Comma);
                }
                a if is_token_op(a) => {
                    let next = self.input_file.content.peek();
                    let op = match next {
                        Some(&b) if is_token_op(b) => {
                            self.advance_span(1);
                            Some(b)
                        }
                        _ => None,
                    };
                    self.advance_span(1);
                    return Some(Token::Op(c, op));
                }
                a if is_token_delimiter(a) => {
                    self.advance_span(1);
                    return Some(Token::Delimiter(c));
                }
                a if is_token_string_literal(a) => {
                    let s = self.consume_until(None, |c| c == '"');
                    if self.input_file.content.peek() != Some(&'"') {
                        panic!("Unterminated string literal");
                    }
                    self.input_file.content.next(); // Compensate for the last "
                    self.advance_span(s.len());
                    self.span.end += 2; // Compensate span to get the last "
                    return Some(Token::StringLiteral(s));
                }
                c => {
                    self.advance_span(1);
                    return Some(Token::Unknown(c))
                }
            }
        }
        None
    }
}