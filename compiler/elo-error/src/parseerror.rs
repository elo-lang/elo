use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum ParseErrorCase {
    UnexpectedToken { got: String, expected: String },
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
    }
}
