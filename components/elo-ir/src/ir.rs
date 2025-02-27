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
    OpAssign(Box<BinaryOperation>),
}

impl BinaryOperation {
    pub fn from_op(first: char, second: Option<char>) -> Option<Self> {
        let mut pat = String::from(first);
        if let Some(c) = second {
            pat.push(c);
        }
        match pat.as_str() {
            "+" => Some(BinaryOperation::Add),
            "-" => Some(BinaryOperation::Sub),
            "*" => Some(BinaryOperation::Mul),
            "/" => Some(BinaryOperation::Div),
            "%" => Some(BinaryOperation::Mod),
            "<" => Some(BinaryOperation::Lt),
            ">" => Some(BinaryOperation::Gt),
            "&" => Some(BinaryOperation::BAnd),
            "|" => Some(BinaryOperation::BOr),
            "^" => Some(BinaryOperation::BXor),
            "=" => Some(BinaryOperation::Assign),
            "==" => Some(BinaryOperation::Eq),
            "!=" => Some(BinaryOperation::Ne),
            "<=" => Some(BinaryOperation::Le),
            ">=" => Some(BinaryOperation::Ge),
            "&&" => Some(BinaryOperation::And),
            "||" => Some(BinaryOperation::Or),
            "<<" => Some(BinaryOperation::LShift),
            ">>" => Some(BinaryOperation::RShift),
            _ => None,
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
    pub fn from_op(first: char, second: Option<char>) -> Option<Self> {
        let mut pat = String::from(first);
        if let Some(c) = second {
            pat.push(c);
        }
        match pat.as_str() {
            "!" => Some(UnaryOperation::Not),
            "~" => Some(UnaryOperation::BNot),
            "-" => Some(UnaryOperation::Neg),
            "&" => Some(UnaryOperation::Addr),
            _ => None,
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
    Access {
        parent: Box<Expression>,
        child: Box<Expression>,
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
}

#[derive(Debug)]
pub struct VarStatement {
    pub binding: String,
    pub assignment: Expression,
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
