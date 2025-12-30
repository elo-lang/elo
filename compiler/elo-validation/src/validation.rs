use crate::typecheck::*;
use elo_ast::ast;
use elo_error::typeerror::*;
use elo_ir::ir;

#[derive(Debug)]
pub enum ValidationError {
    TypeChecking(TypeError),
}

pub struct Validator {
    typechecker: TypeChecker,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            typechecker: TypeChecker::new(),
        }
    }

    pub fn go(mut self, input: Vec<ast::Node>) -> Result<ir::Program, Vec<ValidationError>> {
        let tc = self.typechecker.go(input);
        let mut errors = Vec::new();
        for e in self.typechecker.errors {
            errors.push(ValidationError::TypeChecking(e));
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tc)
    }
}
