use crate::node::Node;

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

#[derive(Debug)]
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
    Parent {
        parent: Box<Expression>,
        child: Box<Expression>,
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
    pub content: Vec<Node>
}

#[derive(Debug)]
pub struct FnStatement {
    pub name: String,
    pub block: Block,
    pub arguments: Vec<NamedField>
}

#[derive(Debug)]
pub enum Type {
    Named {
        name: String,
        generic: Option<Box<Type>>
    },
    Array {
        typ: Box<Type>,
        amount: usize
    },
    Tuple {
        types: Vec<Type>
    },
    Pointer {
        typ: Box<Type>
    },
    FunctionPointer {
        args: Vec<Type>,
        return_: Box<Type>
    }
}

#[derive(Debug)]
pub struct NamedField {
    pub name: String,
    pub typing: Type
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    VarStatement(VarStatement),
    ConstStatement(ConstStatement),
    FnStatement(FnStatement),
    ExpressionStatement(Expression),
}
