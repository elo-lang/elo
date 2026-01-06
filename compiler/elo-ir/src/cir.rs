// Compiled Intermediate Representation
use elo_lexer::span::Span;
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
    pub fn from_ast(x: &crate::ast::BinaryOperation) -> Self {
        match x {
            crate::ast::BinaryOperation::Add => BinaryOperation::Add,
            crate::ast::BinaryOperation::Sub => BinaryOperation::Sub,
            crate::ast::BinaryOperation::Mul => BinaryOperation::Mul,
            crate::ast::BinaryOperation::Div => BinaryOperation::Div,
            crate::ast::BinaryOperation::Mod => BinaryOperation::Mod,
            crate::ast::BinaryOperation::Lt => BinaryOperation::Lt,
            crate::ast::BinaryOperation::Gt => BinaryOperation::Gt,
            crate::ast::BinaryOperation::BAnd => BinaryOperation::BAnd,
            crate::ast::BinaryOperation::BOr => BinaryOperation::BOr,
            crate::ast::BinaryOperation::BXor => BinaryOperation::BXor,
            crate::ast::BinaryOperation::BNot => BinaryOperation::BNot,
            crate::ast::BinaryOperation::Assign => BinaryOperation::Assign,
            crate::ast::BinaryOperation::Eq => BinaryOperation::Eq,
            crate::ast::BinaryOperation::Ne => BinaryOperation::Ne,
            crate::ast::BinaryOperation::Le => BinaryOperation::Le,
            crate::ast::BinaryOperation::Ge => BinaryOperation::Ge,
            crate::ast::BinaryOperation::And => BinaryOperation::And,
            crate::ast::BinaryOperation::Or => BinaryOperation::Or,
            crate::ast::BinaryOperation::LShift => BinaryOperation::LShift,
            crate::ast::BinaryOperation::RShift => BinaryOperation::RShift,
            crate::ast::BinaryOperation::AssignAdd => BinaryOperation::AssignAdd,
            crate::ast::BinaryOperation::AssignSub => BinaryOperation::AssignSub,
            crate::ast::BinaryOperation::AssignMul => BinaryOperation::AssignMul,
            crate::ast::BinaryOperation::AssignDiv => BinaryOperation::AssignDiv,
            crate::ast::BinaryOperation::AssignMod => BinaryOperation::AssignMod,
            crate::ast::BinaryOperation::AssignBAnd => BinaryOperation::AssignBAnd,
            crate::ast::BinaryOperation::AssignBOr => BinaryOperation::AssignBOr,
            crate::ast::BinaryOperation::AssignBXor => BinaryOperation::AssignBXor,
            crate::ast::BinaryOperation::AssignBNot => BinaryOperation::AssignBNot,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Addr,
    Deref,
}

impl UnaryOperation {
    pub fn from_ast(x: &crate::ast::UnaryOperation) -> Self {
        match x {
            crate::ast::UnaryOperation::Not => UnaryOperation::Not,
            crate::ast::UnaryOperation::BNot => UnaryOperation::BNot,
            crate::ast::UnaryOperation::Neg => UnaryOperation::Neg,
            crate::ast::UnaryOperation::Addr => UnaryOperation::Addr,
            crate::ast::UnaryOperation::Deref => UnaryOperation::Deref,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryOperation {
        span: Span,
        operator: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperation {
        span: Span,
        operator: UnaryOperation,
        operand: Box<Expression>,
    },
    StringLiteral {
        span: Span,
        value: String,
    },
    ArrayLiteral {
        span: Span,
        exprs: Vec<Expression>,
        typ: Typing,
    },
    FieldAccess {
        span: Span,
        origin: Box<Expression>,
        field: String,
    },
    FunctionCall {
        span: Span,
        function: String,
        arguments: Vec<Expression>,
    },
    StructInit {
        span: Span,
        origin: Struct,
        fields: Vec<Field>,
    },
    Integer {
        span: Span,
        value: i128,
    },
    Float {
        span: Span,
        value: f64,
    },
    Bool {
        span: Span,
        value: bool,
    },
    Identifier {
        span: Span,
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
    I64,
    I32,
    I16,
    I8,
    Bool,
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
            "u8" => Some(Primitive::U8),
            "u16" => Some(Primitive::U16),
            "u32" => Some(Primitive::U32),
            "u64" => Some(Primitive::U64),
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
pub enum Statement {
    Variable {
        span: Span,
        binding: String,
        assignment: Expression,
        typing: Typing,
    },
    Constant {
        span: Span,
        binding: String,
        value: Expression,
        typing: Typing,
    },
    ReturnStatement {
        span: Span,
        value: Option<Expression>,
        typing: Typing,
    },
    IfStatement {
        span: Span,
        condition: Expression,
        block_true: Block,
        block_false: Block,
    },
    WhileStatement {
        span: Span,
        condition: Expression,
        block: Block,
    },
    FnStatement {
        span: Span,
        inner: Function
    },
    ExternFnStatement {
        span: Span,
        inner: FunctionHead
    },
    StructStatement {
        span: Span,
        inner: Struct
    },
    EnumStatement {
        span: Span,
        inner: Enum
    },
    ExpressionStatement {
        span: Span,
        inner: Expression
    },
}
