use elo_lexer::span::{FileSpan, Span};

use crate::error::error;

#[derive(Debug)]
pub enum SemanticErrorCase {
    MissingReturnValue {
        fn_name: String, typ: String
    },
    TopLevelImperativeStatement {
        statement: String,
    },
    UseExternFnAsExpr {
        name: String,
    },
    InvalidMainSignature {
        should_be: String,
    },
    TypeMismatch {
        got: String,
        expected: String,
    },
    InvalidTupleMember {
        member: usize,
        tuple: String,
        member_count: usize,
    },
    UnknownEnumVariant {
        enumeration: String,
        variant: String,
    },
    InvalidType {
        what: String,
    },
    InvalidCast {
        from: String,
        into: String,
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
        function: String,
        got: usize,
        expected: usize,
        too_much: bool,
    },
    UnresolvedField {
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
    IndexNonIndexable {
        thing: String,
        got: String,
    },
    CallNonFunction {
        typ: String,
    },
    NonAggregateFieldAccess {
        typ: String,
        field: String,
    },
    NonTupleMemberAccess {
        thing: String,
        typ: String,
    },
    VariableRedefinition {
        name: String,
    },
    NameRedefinition {
        name: String,
        defined: Span,
    },
    MisplacedReturn,
}

#[derive(Debug)]
pub struct SemanticError {
    pub span: Span,
    pub case: SemanticErrorCase,
}

pub fn semantic_error(pe: SemanticErrorCase, filespan: &FileSpan) {
    let error_name = "Semantic Error";
    match pe {
        SemanticErrorCase::MissingReturnValue { fn_name, typ } => {
            error(
                error_name,
                &format!("missing return value in {fn_name}"),
                filespan,
                None,
                Some(&format!("add a value of type {typ} after this return")),
            );
        }
        SemanticErrorCase::TopLevelImperativeStatement { statement } => {
            error(
                error_name,
                &format!("found {statement} outside function scope"),
                filespan,
                None,
                Some("you can only use this inside a function block"),
            );
        }
        SemanticErrorCase::UseExternFnAsExpr { name } => {
            error(
                error_name,
                &format!("attempt to use extern function as expression"),
                filespan,
                Some(&format!("if you want to use {name} as a first-class citizen, create a wrapper function")),
                Some("you can only use extern functions by calling it directly"),
            );
        }
        SemanticErrorCase::InvalidMainSignature { should_be } => {
            error(
                error_name,
                &format!("invalid signature for main function"),
                filespan,
                None,
                Some(&format!("this function signature should be {should_be}")),
            );
        }
        SemanticErrorCase::InvalidCast { from, into } => {
            error(
                error_name,
                &format!("invalid cast from {from} to {into}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::CallNonFunction { typ } => {
            error(
                error_name,
                &format!("attempt to call non-function type {typ}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::UnknownEnumVariant { enumeration, variant } => {
            error(
                error_name,
                &format!("unknown variant '{variant}' in enumeration {enumeration}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::NameRedefinition { name, defined } => {
            error(
                error_name,
                &format!("attempt to redefine name {name}"),
                filespan,
                None,
                Some(&format!("note: defined originally at {}:{}:{}", filespan.input_file.filename, defined.line, defined.start)),
            );
        }
        SemanticErrorCase::IndexNonIndexable { thing, got } => {
            error(
                error_name,
                &format!("attempt to index {thing}, of type {got}, but it is not indexable"),
                filespan,
                None,
                Some(&format!("type {got} cannot be used with subscript syntax")),
            );
        }
        SemanticErrorCase::VariableRedefinition { name } => {
            error(
                error_name,
                &format!("attempt to define already defined variable {name}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidTupleMember { member, tuple, member_count } => {
            error(
                error_name,
                &format!("attempt to acess tuple member {member} on {tuple}"),
                filespan,
                None,
                Some(&format!("this tuple only contains only {member_count} member(s) but used {member}")),
            );
        }
        SemanticErrorCase::MisplacedReturn => {
            error(
                error_name,
                &format!("attempt to use return statement outside of function block"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::NoReturn { function, returns } => {
            error(
                error_name,
                &format!("found path of {function} (which returns {returns}) that doesn't return a value"),
                filespan,
                None,
                Some(&format!("ensure that the function returns {returns} after this")),
            );
        }
        SemanticErrorCase::ReturnValueOnVoidFunction { function } => {
            error(
                error_name,
                &format!("tried to return value out of function {function} that doesn't return anything"),
                filespan,
                None,
                Some(&format!("this return should not have a value")),
            );
        }
        SemanticErrorCase::MismatchedReturnType { function, expected, got } => {
            error(
                error_name,
                &format!("return type of {function} is expected to be {expected} but got {got}"),
                filespan,
                None,
                Some(&format!("the value of this return should be of type {expected}")),
            );
        }
        SemanticErrorCase::AssignImmutable { expression } => {
            error(
                error_name,
                &format!("tried to assign to immutable expresion {expression}"),
                filespan,
                None,
                Some("left-hand is immutable, but should be mutable to be assigned"),
            );
        }
        SemanticErrorCase::TypeMismatch { got, expected } => {
            error(
                error_name,
                &format!("type mismatch: expected {expected} but got {got}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidType { what } => {
            error(
                error_name,
                &format!("invalid type: {what}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::InvalidExpression { what, should } => {
            error(
                error_name,
                &format!("invalid expression: expression {what} is expected to be {should}"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::UnresolvedName { name } => {
            error(
                error_name,
                &format!("unresolved name: could not find '{name}' in the current scope"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::UnmatchedArguments {
            function,
            got,
            expected,
            too_much,
        } => {
            if too_much {
                error(
                    error_name,
                    &format!("too much arguments to {function}"),
                    filespan,
                    None,
                    Some(&format!(
                        "function accepts {expected} argument(s) but got {got}",
                    )),
                );
            } else {
                error(
                    error_name,
                    &format!("too few arguments to {function}"),
                    filespan,
                    None,
                    Some(&format!(
                        "{function} accepts {expected} argument(s) but got {got}",
                    )),
                );
            }
        }
        SemanticErrorCase::UnresolvedField { name, from } => {
            error(
                error_name,
                &format!("{from} has no field named '{name}'"),
                filespan,
                None,
                None,
            );
        }
        SemanticErrorCase::NonTupleMemberAccess { thing, typ } => {
            error(
                error_name,
                &format!(
                    "attempt to access member from non-tuple value {thing}"
                ),
                filespan,
                None,
                Some(&format!("expected a tuple here, but got {typ}")),
            );
        }
        SemanticErrorCase::NonAggregateFieldAccess { typ, field } => {
            error(
                error_name,
                &format!(
                    "attempt to access field {field} from non-aggregate type {typ}"
                ),
                filespan,
                None,
                Some(&format!("you can't get fields from {typ}")),
            );
        }
    }
}
