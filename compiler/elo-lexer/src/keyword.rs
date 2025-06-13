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
            _ => None,
        }
    }
}
