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
    pub fn from_op(first: char, second: Option<char>) -> Self {
        let mut pat = String::from(first);
        if let Some(c) = second {
            pat.push(c);
        }
        match pat.as_str() {
            "+" => BinaryOperation::Add,
            "-" => BinaryOperation::Sub,
            "*" => BinaryOperation::Mul,
            "/" => BinaryOperation::Div,
            "%" => BinaryOperation::Mod,
            "<" => BinaryOperation::Lt,
            ">" => BinaryOperation::Gt,
            "&" => BinaryOperation::BAnd,
            "|" => BinaryOperation::BOr,
            "^" => BinaryOperation::BXor,
            "=" => BinaryOperation::Assign,
            "==" => BinaryOperation::Eq,
            "!=" => BinaryOperation::Ne,
            "<=" => BinaryOperation::Le,
            ">=" => BinaryOperation::Ge,
            "&&" => BinaryOperation::And,
            "||" => BinaryOperation::Or,
            "<<" => BinaryOperation::LShift,
            ">>" => BinaryOperation::RShift,
            _ => unreachable!("expected operator, got {:?}", pat),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Ref,
    Deref,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
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
pub enum Structure {
    LetStatement(LetStatement),
    Expression(Expression),
}
