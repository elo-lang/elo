use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum TypeErrorCase {
    TypeMismatch { got: String, expected: String },
}

#[derive(Debug)]
pub struct TypeError {
    pub span: Option<Span>,
    pub case: TypeErrorCase,
}

pub fn type_error(pe: TypeErrorCase, filespan: &FileSpan) {
    match pe {
        TypeErrorCase::TypeMismatch { got, expected } => {
            error(
                "Type Error",
                &format!("type mismatch: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
    }
}
