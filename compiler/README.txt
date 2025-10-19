This is Elo's full compiler implementation.

It is divided into simple categories for the sake of organization.

Category           Description
--------           -----------
elo-ast            Abstract Syntax Tree definition for Elo's syntax

elo-codegen        Implementation of code generation process

elo-error          Definition and implementation for all errors raised
                   by compiler

elo-ir             Intermediate Representation definition for Elo

elo-lexer          Implementation of lexical analysis system for source code

elo-parser         Implementation of syntactical analysis system for source code

elo-validation     Group of systems regarding the validation of source code
                   process, like type-checking or safe-checking


Copyright (c) 2025 Igor Ferreira, Marcio Dantas
