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
        }
    }
}

impl std::fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperation::Add => write!(f, "+"),
            BinaryOperation::Sub => write!(f, "-"),
            BinaryOperation::Mul => write!(f, "*"),
            BinaryOperation::Div => write!(f, "/"),
            BinaryOperation::Mod => write!(f, "%"),
            BinaryOperation::Eq => write!(f, "=="),
            BinaryOperation::Ne => write!(f, "!="),
            BinaryOperation::Lt => write!(f, "<"),
            BinaryOperation::Le => write!(f, "<="),
            BinaryOperation::Gt => write!(f, ">"),
            BinaryOperation::Ge => write!(f, ">="),
            BinaryOperation::And => write!(f, "&&"),
            BinaryOperation::Or => write!(f, "||"),
            BinaryOperation::BAnd => write!(f, "&"),
            BinaryOperation::BOr => write!(f, "|"),
            BinaryOperation::BXor => write!(f, "^"),
            BinaryOperation::LShift => write!(f, "<<"),
            BinaryOperation::RShift => write!(f, ">>"),
            BinaryOperation::Assign => write!(f, "="),
            BinaryOperation::AssignAdd => write!(f, "+="),
            BinaryOperation::AssignSub => write!(f, "-="),
            BinaryOperation::AssignMul => write!(f, "*="),
            BinaryOperation::AssignDiv => write!(f, "/="),
            BinaryOperation::AssignMod => write!(f, "%="),
            BinaryOperation::AssignBAnd => write!(f, "&="),
            BinaryOperation::AssignBOr => write!(f, "|="),
            BinaryOperation::AssignBXor => write!(f, "^="),
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

impl std::fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperation::Neg => write!(f, "-"),
            UnaryOperation::Not => write!(f, "!"),
            UnaryOperation::BNot => write!(f, "~"),
            UnaryOperation::Addr => write!(f, "&"),
            UnaryOperation::Deref => write!(f, "*"),
        }
    }
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
pub struct Expression {
    pub span: Span,
    pub data: ExpressionData,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.data)
    }
}

#[derive(Debug, Clone)]
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
    StringLiteral {
        value: String,
    },
    ArrayLiteral {
        exprs: Vec<Expression>,
        typ: Typing,
    },
    ArraySubscript {
        origin: Box<Expression>,
        index: Box<Expression>,
    },
    FieldAccess {
        origin: Box<Expression>,
        field: String,
    },
    EnumVariant {
        origin: String,
        variant: String,
    },
    TupleAccess {
        origin: Box<Expression>,
        field: usize,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
        extrn: bool,
    },
    StructInit {
        origin: Struct,
        fields: Vec<Field>,
    },
    Tuple {
        exprs: Vec<Expression>,
        types: Vec<Typing>,
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
    Cast {
        expr: Box<Expression>,
        typ: Typing,
    }
}

impl std::fmt::Display for ExpressionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ExpressionData::BinaryOperation { operator, left, right } => write!(f, "{left} {operator} {right}"),
            ExpressionData::UnaryOperation { operator, operand } => write!(f, "{operator}{operand}"),
            ExpressionData::StringLiteral { value } => write!(f, "\"{value}\""),
            ExpressionData::ArraySubscript { origin, index } => write!(f, "\"{origin}[{index}]\""),
            ExpressionData::ArrayLiteral { exprs, .. } => write!(f, "{{{}{}}}", exprs[0], if exprs.len() > 1 { "..." } else { "" }),
            ExpressionData::FieldAccess { origin, field } => write!(f, "{}.{}", origin, field),
            ExpressionData::TupleAccess { origin, field } => write!(f, "{}.{}", origin, field),
            ExpressionData::EnumVariant { origin, variant } => write!(f, "{}.{}", origin, variant),
            ExpressionData::FunctionCall { function, arguments, extrn: _ } => {
                let mut fmt = String::from(&format!("{function}("));
                if arguments.len() == 1 {
                    fmt.push_str(&format!("{}", arguments[0]))
                } else if arguments.len() >= 2 {
                    fmt.push_str(&format!("{}, ...", arguments[0]))
                }
                fmt.push(')');
                write!(f, "{fmt}")
            }
            ExpressionData::StructInit { origin, fields } => {
                let mut fmt = String::from(&format!("{} {{", origin.name));
                if fields.len() == 1 {
                    fmt.push_str(&format!("{}: {}", fields[0].0, fields[0].1))
                } else if fields.len() >= 2 {
                    fmt.push_str(&format!("{}: {}, ...", fields[0].0, fields[0].1))
                }
                fmt.push_str(" }");
                write!(f, "{fmt}")
            }
            ExpressionData::Tuple { exprs, types: _ } => {
                let mut fmt = String::from("(");
                if exprs.len() == 1 {
                    fmt.push_str(&format!("{}", exprs[0]))
                } else if exprs.len() >= 2 {
                    fmt.push_str(&format!("{}, ... {} more", exprs[0], exprs.len() - 1))
                }
                fmt.push_str(")");
                write!(f, "{fmt}")
            }
            ExpressionData::Cast { expr, typ } => {
                write!(f, "{expr} as {typ}")
            }
            ExpressionData::Integer { value } => write!(f, "{}", value),
            ExpressionData::Float { value } => write!(f, "{}", value),
            ExpressionData::Bool { value } => write!(f, "{}", value),
            ExpressionData::Identifier { name } => write!(f, "{}", name),
        }
    }
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
    pub extrn: bool,
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
    U64,
    U32,
    U16,
    U8,
    Int,
    UInt,
    Float,
    F32,
    F64,
    Bool,
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


impl std::fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Primitive::F32 => write!(f, "f32"),
            Primitive::F64 => write!(f, "f64"),
            Primitive::Float => write!(f, "float"),
            Primitive::Int => write!(f, "int"),
            Primitive::UInt => write!(f, "uint"),
            Primitive::I8 => write!(f, "i8"),
            Primitive::I16 => write!(f, "i16"),
            Primitive::I32 => write!(f, "i32"),
            Primitive::I64 => write!(f, "i64"),
            Primitive::U8 => write!(f, "u8"),
            Primitive::U16 => write!(f, "u16"),
            Primitive::U32 => write!(f, "u32"),
            Primitive::U64 => write!(f, "u64"),
            Primitive::Bool => write!(f, "bool"),
            Primitive::Str => write!(f, "str"),
            Primitive::Char => write!(f, "char"),
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
        mutable: bool,
        typ: Box<Typing>,
    },
    Function {
        ret: Box<Typing>,
        arguments: Vec<Typing>,
        variadic: bool,
        extrn: bool,
    },
}

impl Typing {
    pub fn is_integer(&self) -> bool {
        match self {
              Typing::Primitive(Primitive::I64)
            | Typing::Primitive(Primitive::I32)
            | Typing::Primitive(Primitive::I16)
            | Typing::Primitive(Primitive::I8)
            | Typing::Primitive(Primitive::U64)
            | Typing::Primitive(Primitive::U32)
            | Typing::Primitive(Primitive::U16)
            | Typing::Primitive(Primitive::U8)
            | Typing::Primitive(Primitive::Int)
            | Typing::Primitive(Primitive::UInt) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        if let Typing::Primitive(Primitive::Bool) = self {
            return true;
        }
        false
    }

    pub fn is_float(&self) -> bool {
        match self {
              Typing::Primitive(Primitive::F64)
            | Typing::Primitive(Primitive::F32)
            | Typing::Primitive(Primitive::Float) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Typing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Typing::Void => unreachable!("format void type is not allowed"),
            Typing::Primitive(p) => write!(f, "{}", p),
            Typing::Struct(s) => write!(f, "struct {}", s.name),
            Typing::Enum(e) => write!(f, "enum {}", e.name),
            Typing::Array { typ, amount } => write!(f, "{{{}; {}}}", typ, amount),
            Typing::Tuple { types } => {
                let mut fmt = String::from("(");
                for (i, typ) in types.iter().enumerate() {
                    fmt.push_str(&format!("{}", typ));
                    if i < types.len()-1 {
                        fmt.push_str(", ");
                    }
                }
                fmt.push(')');
                write!(f, "{}", fmt)
            }
            Typing::Pointer { typ, mutable } => write!(f, "*{}{}", if *mutable { "mut " } else { "" }, typ),
            Typing::Function { ret, arguments, variadic, extrn: _ } => {
                let mut fmt = String::from("fn (");
                for (i, typ) in arguments.iter().enumerate() {
                    fmt.push_str(&format!("{}", typ));
                    if i < arguments.len()-1 {
                        fmt.push_str(", ");
                    }
                }
                if *variadic {
                    fmt.push_str(", ...");
                }
                fmt.push(')');
                if let Typing::Void = **ret {} else {
                    fmt.push_str(&format!(": {}", ret))
                }
                write!(f, "{}", fmt)
            }
        }
    }
}

pub type TypedField = (String, Typing);
pub type Field = (String, Expression);

#[derive(Debug, Clone)]
pub struct Statement {
    pub span: Span,
    pub kind: StatementKind,
}

#[derive(Debug, Clone)]
pub enum StatementKind {
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
