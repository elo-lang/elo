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
    AssignImmutable {
        expression: String,
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
    ReturnValueOnVoidFunction {
        function: String,
    },
    NoReturn {
        function: String,
        returns: String,
    },
    MismatchedReturnType {
        function: String,
        got: String,
        expected: String,
    },
    NonAggregateMemberAccess {
        typ: String,
        member: String,
    },
    MisplacedReturn,
}

#[derive(Debug)]
pub struct TypeError {
    pub span: Span,
    pub case: TypeErrorCase,
}

pub fn type_error(pe: TypeErrorCase, filespan: &FileSpan) {
    match pe {
        TypeErrorCase::MisplacedReturn => {
            error(
                "Type Check Error",
                &format!("attempt to use return statement outside of function block"),
                filespan,
                None,
                None,
            );
        }
        TypeErrorCase::NoReturn { function, returns } => {
            error(
                "Type Check Error",
                &format!("found path of {function} (which returns {returns}) that doesn't return a value"),
                filespan,
                None,
                Some(&format!("ensure that the function returns {returns} after this")),
            );
        }
        TypeErrorCase::ReturnValueOnVoidFunction { function } => {
            error(
                "Type Check Error",
                &format!("tried to return value out of function {function} that doesn't return anything"),
                filespan,
                None,
                Some(&format!("this return should not have a value")),
            );
        }
        TypeErrorCase::MismatchedReturnType { function, expected, got } => {
            error(
                "Type Check Error",
                &format!("return type of {function} is expected to be {expected} but got {got}"),
                filespan,
                None,
                Some(&format!("the value of this return should be of type {expected}")),
            );
        }
        TypeErrorCase::AssignImmutable { expression } => {
            error(
                "Type Check Error",
                &format!("tried to assign to immutable expresion {expression}"),
                filespan,
                None,
                Some("left-hand is immutable, but should be mutable to be assigned"),
            );
        }
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
