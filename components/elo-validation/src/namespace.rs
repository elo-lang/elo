use std::collections::HashMap;
use elo_ir::ir;

#[derive(Debug)]
pub struct Namespace {
    pub name: Option<String>,
    pub constants: HashMap<String, ir::Typing>,
    pub structs: HashMap<String, ir::Struct>,
    pub enums: HashMap<String, ir::Enum>,
    pub functions: HashMap<String, ir::Function>
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub mutable: bool,
    pub typing: ir::Typing,
}

#[derive(Debug)]
pub struct Scope {
    pub locals: HashMap<String, Variable>,
}