use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum ParseErrorCase {
    UnexpectedToken { got: String, expected: String },
    ExpectedStatement,
    InvalidCharacterLiteral,
}

#[derive(Debug)]
pub struct ParseError {
    pub span: Span,
    pub case: ParseErrorCase,
}

pub fn parse_error(pe: ParseErrorCase, filespan: &FileSpan) {
    match pe {
        ParseErrorCase::UnexpectedToken { got, expected } => {
            error(
                "Parse Error",
                &format!("unexpected token: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
        ParseErrorCase::InvalidCharacterLiteral => {
            error(
                "Parse Error",
                &format!("invalid token found"),
                filespan,
                Some("if you meant to use str/string, use ' or \" instead of `"),
                Some("invalid character literal"),
            );
        }
        ParseErrorCase::ExpectedStatement => {
            error(
                "Parse Error",
                &format!("expected statement after '=>', but found nothing"),
                filespan,
                Some("if you meant to have an empty block, use just {}"),
                None,
            );
        }
    }
}
