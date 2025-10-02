use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum ParseErrorCase {
    UnexpectedToken { got: String, expected: String },
    InvalidCharacterLiteral,
}

#[derive(Debug)]
pub struct ParseError {
    pub span: Option<Span>,
    pub case: ParseErrorCase,
}

pub fn parse_error(pe: ParseErrorCase, filespan: &FileSpan) {
    match pe {
        ParseErrorCase::UnexpectedToken { got, expected } => {
            error(
                "Parse Error",
                &format!("unexpected token while parsing: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
        ParseErrorCase::InvalidCharacterLiteral => {
            error(
                "Parse Error",
                &format!("invalid token found while parsing"),
                filespan,
                Some("if you meant to use str/string, use ' or \" instead of `"),
                Some("invalid character literal"),
            );
        }
    }
}
