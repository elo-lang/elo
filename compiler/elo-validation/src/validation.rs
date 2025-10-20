use crate::typecheck::*;
use elo_ast::ast;
use elo_error::typeerror::*;
use elo_ir::ir;

pub enum ValidationError {
    TypeChecking(TypeError),
}

pub struct Validator {
    typechecker: TypeChecker,
}

impl Validator {
    pub fn new(input: ast::Program) -> Validator {
        Validator {
            typechecker: TypeChecker::new(input),
        }
    }

    pub fn go(self) -> Result<ir::Program, ValidationError> {
        match self.typechecker.go() {
            Ok(ir) => {
                return Ok(ir);
            }
            Err(e) => return Err(ValidationError::TypeChecking(e)),
        }
    }
}
