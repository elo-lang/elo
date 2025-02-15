#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
    Var,
    Let,
    Const,
    Fn,
    Struct,
    Enum,
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
            _ => None,
        }
    }
}
