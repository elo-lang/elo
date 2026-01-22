use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum SemanticErrorCase {
    TypeMismatch {
        got: String,
        expected: String,
    },
    InvalidTupleIndex {
        tried_to: usize,
        tuple: String,
        items_count: usize,
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
    VariableRedefinition {
        name: String,
    },
    MisplacedReturn,
}

#[derive(Debug)]
pub struct SemanticError {
    pub span: Span,
    pub case: SemanticErrorCase,
}

pub fn semantic_error(pe: SemanticErrorCase, filespan: &FileSpan) {
    match pe {
        SemanticErrorCase::VariableRedefinition { name } => {
            error(
                "Type Check Error",
                &format!("attempt to define already defined variable {name}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidTupleIndex { tried_to, tuple, items_count } => {
            error(
                "Type Check Error",
                &format!("attempt to acess tuple index {tried_to} on {tuple}"),
                filespan,
                None,
                Some(&format!("this tuple only contains {items_count} item(s) but used index {tried_to}")),
            );
        }
        SemanticErrorCase::MisplacedReturn => {
            error(
                "Type Check Error",
                &format!("attempt to use return statement outside of function block"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::NoReturn { function, returns } => {
            error(
                "Control-flow Analysis Error",
                &format!("found path of {function} (which returns {returns}) that doesn't return a value"),
                filespan,
                None,
                Some(&format!("ensure that the function returns {returns} after this")),
            );
        }
        SemanticErrorCase::ReturnValueOnVoidFunction { function } => {
            error(
                "Type Check Error",
                &format!("tried to return value out of function {function} that doesn't return anything"),
                filespan,
                None,
                Some(&format!("this return should not have a value")),
            );
        }
        SemanticErrorCase::MismatchedReturnType { function, expected, got } => {
            error(
                "Type Check Error",
                &format!("return type of {function} is expected to be {expected} but got {got}"),
                filespan,
                None,
                Some(&format!("the value of this return should be of type {expected}")),
            );
        }
        SemanticErrorCase::AssignImmutable { expression } => {
            error(
                "Type Check Error",
                &format!("tried to assign to immutable expresion {expression}"),
                filespan,
                None,
                Some("left-hand is immutable, but should be mutable to be assigned"),
            );
        }
        SemanticErrorCase::TypeMismatch { got, expected } => {
            error(
                "Type Check Error",
                &format!("type mismatch: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidType { what } => {
            error(
                "Type Check Error",
                &format!("invalid type: {what}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidExpression { what, should } => {
            error(
                "Type Check Error",
                &format!("invalid expression: expression {what} is expected to be {should}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::UnresolvedName { name } => {
            error(
                "Type Check Error",
                &format!("unresolved name: could not find '{name}' in the current scope"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::UnmatchedArguments {
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
        SemanticErrorCase::UnresolvedMember { name, from } => {
            error(
                "Type Check Error",
                &format!("unresolved member: {from} has no field named '{name}'"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::NonAggregateMemberAccess { typ, member } => {
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
