// Abstract Syntax Tree

use elo_lexer::{span::Span, token::Token};

#[derive(Debug)]
pub struct Program {
    pub filename: String,
    pub nodes: Vec<Node>,
}

pub type OperatorPrecedence = usize;

#[derive(Debug)]
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
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignBAnd,
    AssignBOr,
    AssignBXor,
}

impl BinaryOperation {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.text().as_str() {
            "+"  => Some(BinaryOperation::Add),
            "-"  => Some(BinaryOperation::Sub),
            "*"  => Some(BinaryOperation::Mul),
            "/"  => Some(BinaryOperation::Div),
            "%"  => Some(BinaryOperation::Mod),
            "<"  => Some(BinaryOperation::Lt),
            ">"  => Some(BinaryOperation::Gt),
            "&"  => Some(BinaryOperation::BAnd),
            "|"  => Some(BinaryOperation::BOr),
            "^"  => Some(BinaryOperation::BXor),
            "="  => Some(BinaryOperation::Assign),
            "==" => Some(BinaryOperation::Eq),
            "!=" => Some(BinaryOperation::Ne),
            "<=" => Some(BinaryOperation::Le),
            ">=" => Some(BinaryOperation::Ge),
            "&&" => Some(BinaryOperation::And),
            "||" => Some(BinaryOperation::Or),
            "<<" => Some(BinaryOperation::LShift),
            ">>" => Some(BinaryOperation::RShift),
            "+=" => Some(BinaryOperation::AssignAdd),
            "-=" => Some(BinaryOperation::AssignSub),
            "*=" => Some(BinaryOperation::AssignMul),
            "/=" => Some(BinaryOperation::AssignDiv),
            "%=" => Some(BinaryOperation::AssignMod),
            "&=" => Some(BinaryOperation::AssignBAnd),
            "|=" => Some(BinaryOperation::AssignBOr),
            "^=" => Some(BinaryOperation::AssignBXor),
            _    => None,
        }
    }

    pub fn precedence(&self) -> OperatorPrecedence {
        match self {
            BinaryOperation::Assign     => 1,
            BinaryOperation::AssignAdd  => 1,
            BinaryOperation::AssignSub  => 1,
            BinaryOperation::AssignMul  => 1,
            BinaryOperation::AssignDiv  => 1,
            BinaryOperation::AssignMod  => 1,
            BinaryOperation::AssignBAnd => 1,
            BinaryOperation::AssignBOr  => 1,
            BinaryOperation::AssignBXor => 1,

            BinaryOperation::Eq         => 2,
            BinaryOperation::Ne         => 2,

            BinaryOperation::Le         => 3,
            BinaryOperation::Ge         => 3,
            BinaryOperation::Lt         => 3,
            BinaryOperation::Gt         => 3,

            BinaryOperation::And        => 4,
            BinaryOperation::Or         => 4,

            BinaryOperation::BAnd       => 5,
            BinaryOperation::BOr        => 5,
            BinaryOperation::BXor       => 5,

            BinaryOperation::Add        => 6,
            BinaryOperation::Sub        => 6,

            BinaryOperation::Mul        => 7,
            BinaryOperation::Div        => 7,
            BinaryOperation::Mod        => 7,

            BinaryOperation::LShift     => 8,
            BinaryOperation::RShift     => 8,
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Addr,
    Deref,
}

impl UnaryOperation {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token.text().as_str() {
            "!" => Some(UnaryOperation::Not),
            "~" => Some(UnaryOperation::BNot),
            "-" => Some(UnaryOperation::Neg),
            "&" => Some(UnaryOperation::Addr),
            "*" => Some(UnaryOperation::Deref),
            _ => None,
        }
    }

    pub fn precedence(&self) -> OperatorPrecedence {
        match self {
            UnaryOperation::Not   => 9,
            UnaryOperation::BNot  => 9,
            UnaryOperation::Neg   => 9,
            UnaryOperation::Addr  => 9,
            UnaryOperation::Deref => 9
        }
    }
}

#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub data: ExpressionData,
}

#[derive(Debug)]
pub enum ExpressionData {
    BinaryOperation {
        operator: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperation {
        operator: UnaryOperation,
        operand: Box<Expression>,
    },
    StrLiteral {
        value: String,
    },
    StringLiteral {
        value: String,
    },
    CStrLiteral {
        value: String,
    },
    // TODO: Add proper StringLiteral when we deal with dynamic memory (standard library)
    CharacterLiteral {
        value: char,
    },
    Subscript {
        origin: Box<Expression>,
        inner: Box<Expression>,
    },
    Cast {
        expr: Box<Expression>,
        typ: Type,
    },
    FieldAccess {
        origin: Box<Expression>,
        field: String,
    },
    Tuple {
        exprs: Vec<Expression>,
    },
    TupleAccess {
        origin: Box<Expression>,
        field: usize,
    },
    Array {
        exprs: Vec<Expression>,
        amount: usize,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StructInit {
        name: String,
        fields: Vec<Field>,
    },
    IntegerLiteral {
        value: i128,
    },
    FloatLiteral {
        value: f64,
    },
    BooleanLiteral {
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
    pub typing: Type,
}

#[derive(Debug)]
pub struct Block {
    pub content: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    pub span: Span,
    pub stmt: Statement,
}

#[derive(Debug)]
pub struct FnStatement {
    pub name: String,
    pub block: Block,
    pub ret: Option<Type>,
    pub arguments: Vec<TypedField>,
}

#[derive(Debug)]
pub struct ExternFnStatement {
    pub name: String,
    pub ret: Option<Type>,
    pub arguments: Vec<TypedField>,
    pub variadic: bool,
}

#[derive(Debug)]
pub struct StructStatement {
    pub name: String,
    pub fields: Vec<TypedField>,
}

#[derive(Debug)]
pub struct EnumStatement {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub block_true: Block,
    pub block_false: Option<Block>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expr: Option<Expression>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Type {
    pub span: Span,
    pub typing: Typing,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Typing {
    Named {
        name: String,
        generic: Option<Box<Type>>,
    },
    Array {
        typ: Box<Type>,
        amount: usize,
    },
    Slice {
        typ: Box<Type>,
    },
    Tuple {
        types: Vec<Type>,
    },
    Pointer {
        mutable: bool,
        typ: Box<Type>,
    },
    Function {
        args: Vec<Type>,
        ret: Option<Box<Type>>,
    },
}

#[derive(Debug)]
pub struct TypedField {
    pub name: String,
    pub typing: Type,
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
    FnStatement(FnStatement),
    ExternFnStatement(ExternFnStatement),
    StructStatement(StructStatement),
    EnumStatement(EnumStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ExpressionStatement(Expression),
    ReturnStatement(ReturnStatement),
}
