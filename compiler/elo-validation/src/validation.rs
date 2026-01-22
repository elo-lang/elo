use crate::semcheck::*;
use elo_error::semerror::*;
use elo_ir::*;

#[derive(Debug)]
pub enum ValidationError {
    SemanticChecker(SemanticError),
}

pub struct Validator {
    semchecker: SemanticChecker,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            semchecker: SemanticChecker::new(),
        }
    }

    pub fn go(mut self, input: Vec<ast::Node>) -> Result<cir::Program, Vec<ValidationError>> {
        let tc = self.semchecker.go(input);
        let mut errors = Vec::new();
        for e in self.semchecker.errors {
            errors.push(ValidationError::SemanticChecker(e));
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(tc)
    }
}
