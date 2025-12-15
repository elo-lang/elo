use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum TypeErrorCase {
    TypeMismatch {
        got: String,
        expected: String,
    },
    InvalidType {
        what: String,
    },
    InvalidExpression {
        what: String,
        should: String,
    },
    UnresolvedName {
        name: String,
    },
    UnmatchedArguments {
        name: String,
        got: usize,
        expected: usize,
        too_much: bool,
    },
    UnresolvedMember {
        name: String,
        from: String,
    },
    NonAggregateMemberAccess {
        typ: String,
        member: String,
    },
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
                "Type Check Error",
                &format!("type mismatch: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::InvalidType { what } => {
            error(
                "Type Check Error",
                &format!("invalid type: {what}"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::InvalidExpression { what, should } => {
            error(
                "Type Check Error",
                &format!("invalid expression: expression {what} is expected to be {should}"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::UnresolvedName { name } => {
            error(
                "Type Check Error",
                &format!("unresolved name: could not find '{name}' in the current scope"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::UnmatchedArguments {
            name,
            got,
            expected,
            too_much,
        } => {
            if too_much {
                error(
                    "Type Check Error",
                    &format!("too much arguments to function {name}"),
                    filespan,
                    None,
                    Some(&format!(
                        "function {name} accepts {expected} argument(s) but got {got}",
                    )),
                );
            } else {
                error(
                    "Type Check Error",
                    &format!("too few arguments to function {name}"),
                    filespan,
                    None,
                    Some(&format!(
                        "function {name} accepts {expected} argument(s) but got {got}",
                    )),
                );
            }
        }
        TypeErrorCase::UnresolvedMember { name, from } => {
            error(
                "Type Check Error",
                &format!("unresolved member: {from} has no field named '{name}'"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::NonAggregateMemberAccess { typ, member } => {
            error(
                "Type Check Error",
                &format!(
                    "non aggregate member access: attempt to access member {member} from non-aggregate type {typ}"
                ),
                filespan,
                None,
                Some(&format!("you can't get members from {typ}")),
            );
        }
    }
}
