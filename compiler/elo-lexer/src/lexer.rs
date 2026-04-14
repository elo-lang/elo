use std::iter::Peekable;
use std::str::Chars;

use crate::inputfile::InputFile;
use crate::keyword::Keyword;
use crate::lexem::Lexem;
use crate::span::FileSpan;
use crate::token::{Token, StringKind};

#[derive(Eq, PartialEq, Clone)]
enum State {
    Normal,
    String { kind: StringKind, buffer: String },
    Interpolation { string: StringKind, depth: usize }
}

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub chars: Peekable<Chars<'a>>,
    pub span: FileSpan<'a>,
    state: State,
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

macro_rules! two_char_op {
    () => {
        "+=" | "-="
            | "/="
            | "*="
            | "%="
            | "&="
            | "|="
            | "^="
            | "~="
            | ">="
            | "<="
            | "!="
            | "=="
            | "&&"
            | "||"
            | ">>"
            | "<<"
            | "=>"
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

fn unescape(escape_char: char) -> char {
    match escape_char {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\x0B',
        'f' => '\x0C',
        '\\' => '\\',
        '\'' => '\'',
        '"' => '\"',
        other => other
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input_file: InputFile<'a>) -> Lexer<'a> {
        Lexer {
            input_file: input_file.clone(),
            chars: input_file.content.chars().peekable(),
            span: FileSpan::empty(input_file),
            state: State::Normal,
        }
    }

    fn advance_span(&mut self, advance_length: usize) {
        self.span.start = self.span.end;
        self.span.end += advance_length;
    }

    fn advance_line(&mut self) {
        self.span.line += 1;
        self.span.start = 1;
        self.span.end = 1;
    }

    pub fn consume_while(
        &mut self,
        start: Option<&char>,
        matches: fn(char) -> bool,
    ) -> (String, Option<char>) {
        let mut buffer = String::new();
        if let Some(start) = start {
            buffer.push(*start);
        }
        let mut last_char = None;
        while let Some(&c) = self.chars.peek() {
            if !matches(c) {
                break;
            }
            last_char = Some(c);
            buffer.push(c);
            self.chars.next();
        }
        (buffer, last_char)
    }

    fn token_float(&mut self, integer: String) -> Token {
        self.chars.next();
        if !matches!(self.chars.peek(), Some(&numeric!())) {
            self.advance_span(integer.len() + 1); // 1 for the '.'
            return Token::Float(format!("{integer}.0"));
        }
        let (fractional, _) = self.consume_while(None, |c| matches!(c, numeric!()));
        self.advance_span(integer.len() + fractional.len() + 1); // 1 for the '.'
        return Token::Float(format!("{integer}.{fractional}"));
    }

    fn token_numeric(&mut self, ch: &char) -> Token {
        // TODO: check suffixes like u8, i8, etc?
        if ch == &'0' {
            if let Some(c) = self.chars.peek() {
                match c {
                    'b' => {
                        self.chars.next();
                        self.advance_span(2);
                        let (number, _) =
                            self.consume_while(None, |c| matches!(c, numeric_binary!()));
                        self.span.end += number.len();
                        return Token::Integer(number, 2);
                    }
                    'o' => {
                        self.chars.next();
                        self.advance_span(2);
                        let (number, _) =
                            self.consume_while(None, |c| matches!(c, numeric_octal!()));
                        self.span.end += number.len();
                        return Token::Integer(number, 8);
                    }
                    'x' => {
                        self.chars.next();
                        self.advance_span(2);
                        let (number, _) = self.consume_while(None, |c| matches!(c, numeric_hex!()));
                        self.span.end += number.len();
                        return Token::Integer(number, 16);
                    }
                    '.' => return self.token_float("0".to_string()),
                    _ => {}
                }
            }
            self.advance_span(1);
            return Token::Integer(String::from("0"), 10);
        }
        let (number, _) = self.consume_while(Some(ch), |c| matches!(c, numeric!()));
        if let Some(c) = self.chars.peek() {
            if *c == '.' { // float literal
                return self.token_float(number);
            }
        }
        self.advance_span(number.len());
        return Token::Integer(number, 10);
    }

    fn token_word(&mut self, ch: &char) -> Token {
        let (s, _) = self.consume_while(Some(ch), |c| matches!(c, identifier!()));
        self.advance_span(s.len());

        if let Some(kw) = Keyword::from_str(s.as_str()) {
            return Token::Keyword(kw);
        }

        return Token::Identifier(s);
    }

    fn token_op(&mut self, ch: &char) -> Token {
        self.advance_span(1);
        match self.chars.peek() {
            Some(&b) if matches!(format!("{ch}{b}").as_str(), two_char_op!()) => {
                self.chars.next();
                self.span.end += 1;
                return Token::Op(*ch, Some(b));
            }
            _ => {
                return Token::Op(*ch, None);
            }
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexem;

    fn next(&mut self) -> Option<Lexem> {
        while let Some(ch) = self.chars.next() {
            if let State::String { buffer, kind } = &mut self.state {
                match ch {
                    '\'' if matches!(kind, StringKind::Static | StringKind::C) => {
                        self.span.end += 1;
                        let buffer = std::mem::take(buffer);
                        let kind = *kind;
                        self.state = State::Normal;
                        return Some(Lexem::new(self.span.into_span(), Token::String(kind, buffer)));
                    }
                    '"' if *kind == StringKind::Dynamic => {
                        self.span.end += 1;
                        let buffer = std::mem::take(buffer);
                        let kind = *kind;
                        self.state = State::Normal;
                        return Some(Lexem::new(self.span.into_span(), Token::String(kind, buffer)));
                    }
                    '\\' => {
                        self.span.end += 1;
                        if let Some(c) = self.chars.peek() {
                            if *c == '(' {
                                let buffer = std::mem::take(buffer);
                                let kind = *kind;
                                self.state = State::Interpolation { string: kind, depth: 0 };
                                return Some(Lexem::new(self.span.into_span(), Token::String(kind, buffer)));
                            }
                            self.span.end += 1;
                            buffer.push(unescape(*c));
                            self.chars.next();
                            continue;
                        } else {
                            buffer.push(ch);
                        }
                    }
                    _ => {
                        buffer.push(ch);
                        self.span.end += 1;
                        continue;
                    }
                }
            }
            return match ch {
                'c' if self.chars.peek() == Some(&'\'') => {
                    self.chars.next();
                    self.advance_span(2); // acount for 'c' + quote
                    self.state = State::String { kind: StringKind::C, buffer: String::new() };
                    continue;
                }
                '\'' => {
                    self.advance_span(1); // acount for quote
                    self.state = State::String { kind: StringKind::Static, buffer: String::new() };
                    continue;
                }
                '"' => {
                    self.advance_span(1); // acount for quote
                    self.state = State::String { kind: StringKind::Dynamic, buffer: String::new() };
                    continue;
                }
                '`' => {
                    self.advance_span(1); // account for quote
                    let mut buf = String::new();
                    while let Some(c) = self.chars.peek() {
                        if *c == '`' {
                            self.chars.next();
                            break;
                        }
                        if *c == '\\' {
                            let c = *c;
                            self.chars.next();
                            self.span.end += 1;
                            if let Some(e) = self.chars.next() {
                                self.span.end += 1;
                                buf.push(unescape(e));
                            } else {
                                buf.push(unescape(c));
                            }
                            continue;
                        }
                        buf.push(self.chars.next().unwrap());
                        self.span.end += 1;
                    }
                    self.span.end += 1;
                    return Some(Lexem::new(self.span.into_span(), Token::Character(buf)));
                }
                '/' if self.state == State::Normal && self.chars.peek() == Some(&'/') => {
                    let _ = self.consume_while(Some(&ch), |c| c != '\n');
                    continue;
                }
                '\n' => {
                    self.advance_line();
                    Some(Lexem::new(self.span.into_span(), Token::Newline))
                }
                whitespace!() => {
                    self.advance_span(1);
                    continue;
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
                    if ch == '.' && self.chars.peek() == Some(&'.') {
                        self.chars.next(); // Consume the second dot
                        if self.chars.peek() == Some(&'.') {
                            self.chars.next(); // Consume the third dot
                            self.advance_span(3);
                            return Some(Lexem::new(self.span.into_span(), Token::Variadic));
                        }
                        self.advance_span(2);
                        Some(Lexem::new(self.span.into_span(), Token::Delimiter(ch)))
                    } else {
                        self.advance_span(1);
                        if ch == '(' {
                            if let State::Interpolation { string: _, depth } = &mut self.state {
                                *depth += 1;
                                if *depth == 1 {
                                    return Some(Lexem::new(self.span.into_span(), Token::InterpolationBegin));
                                }
                            }
                        }
                        if ch == ')' {
                            if let State::Interpolation { string, depth } = &mut self.state {
                                // It's kinda impossible for the depth to be less than one because
                                // it always starts with 1 anyway because of the first ( after backslash.
                                // But just in case, let's put <=...
                                if *depth <= 1 { // end
                                    self.state = State::String { kind: *string, buffer: String::new() };
                                    return Some(Lexem::new(self.span.into_span(), Token::InterpolationEnd));
                                }
                                *depth -= 1;
                            }
                        }
                        Some(Lexem::new(self.span.into_span(), Token::Delimiter(ch)))
                    }
                }
                _ => {
                    self.advance_span(1);
                    Some(Lexem::new(self.span.into_span(), Token::Unknown(ch)))
                }
            };
        }
        if let State::String { kind, buffer } = &mut self.state {
            let buffer = std::mem::take(buffer);
            let kind = *kind;
            self.state = State::Normal;
            return Some(Lexem::new(self.span.into_span(), Token::String(kind, buffer)));
        }
        None
    }
}
