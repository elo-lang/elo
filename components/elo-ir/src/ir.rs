use elo_lexer::span::Span;

#[derive(Debug, Eq, PartialEq)]
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
    BXor,
    LShift,
    RShift,
    Assign,
}

impl BinaryOperation {
    pub fn from_ast(x: elo_ast::ast::BinaryOperation) -> Self {
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
            elo_ast::ast::BinaryOperation::Assign => BinaryOperation::Assign,
            elo_ast::ast::BinaryOperation::Eq => BinaryOperation::Eq,
            elo_ast::ast::BinaryOperation::Ne => BinaryOperation::Ne,
            elo_ast::ast::BinaryOperation::Le => BinaryOperation::Le,
            elo_ast::ast::BinaryOperation::Ge => BinaryOperation::Ge,
            elo_ast::ast::BinaryOperation::And => BinaryOperation::And,
            elo_ast::ast::BinaryOperation::Or => BinaryOperation::Or,
            elo_ast::ast::BinaryOperation::LShift => BinaryOperation::LShift,
            elo_ast::ast::BinaryOperation::RShift => BinaryOperation::RShift,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Addr,
}

impl UnaryOperation {
    pub fn from_ast(x: elo_ast::ast::UnaryOperation) -> Self {
        match x {
            elo_ast::ast::UnaryOperation::Not => UnaryOperation::Not,
            elo_ast::ast::UnaryOperation::BNot => UnaryOperation::BNot,
            elo_ast::ast::UnaryOperation::Neg => UnaryOperation::Neg,
            elo_ast::ast::UnaryOperation::Addr => UnaryOperation::Addr,
        }
    }
}

#[derive(Debug)]
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
        value: f32,
    },
    Bool {
        value: bool,
    },
    Identifier {
        name: String,
    },
}

#[derive(Debug)]
pub struct LetStatement {
    pub binding: String,
    pub assignment: Expression,
    pub typing: Typing,
}

#[derive(Debug)]
pub struct VarStatement {
    pub binding: String,
    pub assignment: Expression,
    pub typing: Typing,
}

#[derive(Debug)]
pub struct ConstStatement {
    pub binding: String,
    pub assignment: Expression,
    pub typing: Typing,
}

#[derive(Debug)]
pub struct Block {
    pub content: Vec<EvaluatedNode>,
}

#[derive(Debug)]
pub struct EvaluatedProgram {
    pub nodes: Vec<EvaluatedNode>,
}

#[derive(Debug)]
pub struct EvaluatedNode {
    span: Span,
    stmt: Statement,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub block: Block,
    pub ret: Option<Typing>,
    pub arguments: Vec<TypedField>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<TypedField>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
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
    F128,
    Int,
    UInt,
    Float,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Typing {
    Primitive(Primitive),
    Struct(Struct, Option<Box<Typing>>),
    Enum(Enum, Option<Box<Typing>>),
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

#[derive(Debug, Eq, PartialEq)]
pub struct TypedField {
    pub name: String,
    pub typing: Typing,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    VarStatement(VarStatement),
    ConstStatement(ConstStatement),
    FnStatement(Function),
    StructStatement(Struct),
    EnumStatement(Enum),
    ExpressionStatement(Expression),
}
