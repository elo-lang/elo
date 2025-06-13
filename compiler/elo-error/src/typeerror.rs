use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum TypeErrorCase {
    TypeMismatch { got: String, expected: String },
    InvalidType { what: String },
    InvalidExpression { what: String, should: String },
    UnresolvedName { name: String },
    UnmatchedArguments { name: String, got: usize, expected: usize }
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
        TypeErrorCase::InvalidType { what } => {
            error(
                "Type Error",
                &format!("invalid type: {what}"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::InvalidExpression { what, should } => {
            error(
                "Type Error",
                &format!("invalid expression: expression {what}. Expected to be {should}."),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::UnresolvedName { name } => {
            error(
                "Type Error",
                &format!("unresolved name: could not find {name} in the current scope."),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::UnmatchedArguments { name, got, expected } => {
            error(
                "Type Error",
                &format!("arguments to function {name}: expected {expected} argument(s) but got {got} argument(s) in the function call."),
                filespan,
                None,
                None,
            );
        }
    }
}
