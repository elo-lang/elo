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

#[derive(Debug)]
pub enum UnaryOperation {
    Neg,
    Not,
    BNot,
    Addr,
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
    pub typing: Type,
}

#[derive(Debug)]
pub struct VarStatement {
    pub binding: String,
    pub assignment: Expression,
    pub typing: Type,
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
    pub ret: Option<Type>,
    pub arguments: Vec<TypedField>
}

#[derive(Debug)]
pub struct StructStatement {
    pub name: String,
    pub fields: Vec<TypedField>
}

#[derive(Debug)]
pub struct EnumStatement {
    pub name: String,
    pub variants: Vec<String>
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructTyping { name: String, fields: Vec<TypedField> }

#[derive(Debug, PartialEq, Eq)]
pub struct EnumTyping { name: String, vars: Vec<String> }

#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    Str, F32, F64, Uint, Int, Bool,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128
}

impl Primitive {
    fn from_string(s: String) -> Option<Primitive> {
        match s.as_str() {
            "str" => Some(Primitive::Str),
            "u128" => Some(Primitive::U128),
            "i128" => Some(Primitive::I128),
            "i64" => Some(Primitive::I64),
            "u64" => Some(Primitive::U64),
            "i32" => Some(Primitive::I32),
            "u32" => Some(Primitive::U32),
            "i16" => Some(Primitive::I16),
            "u16" => Some(Primitive::U16),
            "i8" => Some(Primitive::I8),
            "u8" => Some(Primitive::U8),
            "int" => Some(Primitive::Int),
            "uint" => Some(Primitive::Uint),
            "bool" => Some(Primitive::Bool),
            "f64" => Some(Primitive::F64),
            "f32" => Some(Primitive::F32),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Primitive {
        typ: Primitive,
    },
    Struct {
        typ: StructTyping,
        generic: Option<Box<Type>>
    },
    Enum {
        typ: EnumTyping,
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
        return_: Box<Option<Type>>
    }
}

#[derive(Debug, PartialEq, Eq)]

pub struct TypedField {
    pub name: String,
    pub typing: Type
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: Expression
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    VarStatement(VarStatement),
    ConstStatement(ConstStatement),
    FnStatement(FnStatement),
    StructStatement(StructStatement),
    EnumStatement(EnumStatement),
    ExpressionStatement(Expression),
}
