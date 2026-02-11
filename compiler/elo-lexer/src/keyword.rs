#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Var,
    Let,
    Const,
    Fn,
    Struct,
    Enum,
    If,
    Else,
    While,
    Return,
    Extern,
    True,
    False,
    Mut,
    As,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "var" => Some(Keyword::Var),
            "let" => Some(Keyword::Let),
            "const" => Some(Keyword::Const),
            "fn" => Some(Keyword::Fn),
            "struct" => Some(Keyword::Struct),
            "enum" => Some(Keyword::Enum),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            "ret" => Some(Keyword::Return),
            "extern" => Some(Keyword::Extern),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "mut" => Some(Keyword::Mut),
            "as" => Some(Keyword::As),
            _ => None,
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Keyword::Var => write!(f, "var"),
            Keyword::Let => write!(f, "let"),
            Keyword::Const => write!(f, "const"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::While => write!(f, "while"),
            Keyword::Return => write!(f, "return"),
            Keyword::Extern => write!(f, "extern"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Mut => write!(f, "mut"),
            Keyword::As => write!(f, "as"),
        }
    }
}
