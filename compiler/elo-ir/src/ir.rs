use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BAnd,
    BOr,
    BNot,
    BXor,
    LShift,
    RShift,
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignBAnd,
    AssignBOr,
    AssignBXor,
    AssignBNot,
}

impl BinaryOperation {
    pub fn from_ast(x: &elo_ast::ast::BinaryOperation) -> Self {
        match x {
            elo_ast::ast::BinaryOperation::Add => BinaryOperation::Add,
            elo_ast::ast::BinaryOperation::Sub => BinaryOperation::Sub,
            elo_ast::ast::BinaryOperation::Mul => BinaryOperation::Mul,
            elo_ast::ast::BinaryOperation::Div => BinaryOperation::Div,
            elo_ast::ast::BinaryOperation::Mod => BinaryOperation::Mod,
            elo_ast::ast::BinaryOperation::Lt => BinaryOperation::Lt,
            elo_ast::ast::BinaryOperation::Gt => BinaryOperation::Gt,
            elo_ast::ast::BinaryOperation::BAnd => BinaryOperation::BAnd,
            elo_ast::ast::BinaryOperation::BOr => BinaryOperation::BOr,
            elo_ast::ast::BinaryOperation::BXor => BinaryOperation::BXor,
            elo_ast::ast::BinaryOperation::BNot => BinaryOperation::BNot,
            elo_ast::ast::BinaryOperation::Assign => BinaryOperation::Assign,
            elo_ast::ast::BinaryOperation::Eq => BinaryOperation::Eq,
            elo_ast::ast::BinaryOperation::Ne => BinaryOperation::Ne,
            elo_ast::ast::BinaryOperation::Le => BinaryOperation::Le,
            elo_ast::ast::BinaryOperation::Ge => BinaryOperation::Ge,
            elo_ast::ast::BinaryOperation::And => BinaryOperation::And,
            elo_ast::ast::BinaryOperation::Or => BinaryOperation::Or,
            elo_ast::ast::BinaryOperation::LShift => BinaryOperation::LShift,
            elo_ast::ast::BinaryOperation::RShift => BinaryOperation::RShift,
            elo_ast::ast::BinaryOperation::AssignAdd => BinaryOperation::AssignAdd,
            elo_ast::ast::BinaryOperation::AssignSub => BinaryOperation::AssignSub,
            elo_ast::ast::BinaryOperation::AssignMul => BinaryOperation::AssignMul,
            elo_ast::ast::BinaryOperation::AssignDiv => BinaryOperation::AssignDiv,
            elo_ast::ast::BinaryOperation::AssignMod => BinaryOperation::AssignMod,
            elo_ast::ast::BinaryOperation::AssignBAnd => BinaryOperation::AssignBAnd,
            elo_ast::ast::BinaryOperation::AssignBOr => BinaryOperation::AssignBOr,
            elo_ast::ast::BinaryOperation::AssignBXor => BinaryOperation::AssignBXor,
            elo_ast::ast::BinaryOperation::AssignBNot => BinaryOperation::AssignBNot,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Addr,
}

impl UnaryOperation {
    pub fn from_ast(x: &elo_ast::ast::UnaryOperation) -> Self {
        match x {
            elo_ast::ast::UnaryOperation::Not => UnaryOperation::Not,
            elo_ast::ast::UnaryOperation::BNot => UnaryOperation::BNot,
            elo_ast::ast::UnaryOperation::Neg => UnaryOperation::Neg,
            elo_ast::ast::UnaryOperation::Addr => UnaryOperation::Addr,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation {
        operator: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperation,
        operand: Box<Expression>,
    },
    StringLiteral {
        value: String,
    },
    FieldAccess {
        origin: Box<Expression>,
        field: String,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StructInit {
        origin: Struct,
        fields: Vec<Field>,
    },
    Integer {
        value: i128,
    },
    Float {
        value: f64,
    },
    Bool {
        value: bool,
    },
    Identifier {
        name: String,
    },
}

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub binding: String,
    pub assignment: Expression,
    pub typing: Typing,
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone)]
pub struct Program {
    pub nodes: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub head: FunctionHead,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct FunctionHead {
    pub name: String,
    pub ret: Typing,
    pub arguments: Vec<TypedField>,
    pub variadic: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, Typing>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Primitive {
    I128,
    I64,
    I32,
    I16,
    I8,
    Bool,
    U128,
    U64,
    U32,
    U16,
    U8,
    F32,
    F64,
    Int,
    UInt,
    Float,
    Str,
    Char,
}

impl Primitive {
    // return corresponding Primitive from str
    pub fn from_str(s: &str) -> Option<Primitive> {
        match s {
            "f32" => Some(Primitive::F32),
            "f64" => Some(Primitive::F64),
            "float" => Some(Primitive::Float),
            "int" => Some(Primitive::Int),
            "uint" => Some(Primitive::UInt),
            "i8" => Some(Primitive::I8),
            "i16" => Some(Primitive::I16),
            "i32" => Some(Primitive::I32),
            "i64" => Some(Primitive::I64),
            "i128" => Some(Primitive::I128),
            "u8" => Some(Primitive::U8),
            "u16" => Some(Primitive::U16),
            "u32" => Some(Primitive::U32),
            "u64" => Some(Primitive::U64),
            "u128" => Some(Primitive::U128),
            "bool" => Some(Primitive::Bool),
            "str" => Some(Primitive::Str),
            "char" => Some(Primitive::Char),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Typing {
    Void,
    Primitive(Primitive),
    Struct(Struct),
    Enum(Enum),
    Array {
        typ: Box<Typing>,
        amount: usize,
    },
    Tuple {
        types: Vec<Typing>,
    },
    Pointer {
        typ: Box<Typing>,
    },
    FunctionPointer {
        args: Vec<Typing>,
        ret: Box<Option<Typing>>,
    },
}

pub type TypedField = (String, Typing);
pub type Field = (String, Expression);

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
    pub typing: Typing,
}

#[derive(Debug, Clone)]
pub enum Statement {
    None,
    Variable {
        binding: String,
        assignment: Expression,
        typing: Typing,
    },
    Constant {
        binding: String,
        value: Expression,
        typing: Typing,
    },
    ReturnStatement {
        value: Option<Expression>,
        typing: Typing,
    },
    IfStatement {
        condition: Expression,
        block_true: Block,
        block_false: Block,
    },
    WhileStatement {
        condition: Expression,
        block: Block,
    },
    FnStatement(Function),
    ExternFnStatement(FunctionHead),
    StructStatement(Struct),
    EnumStatement(Enum),
    ExpressionStatement(Expression),
}
