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

    pub fn as_text(&self) -> String {
        match self {
            Keyword::Var => String::from("var"),
            Keyword::Let => String::from("let"),
            Keyword::Const => String::from("const"),
            Keyword::Fn => String::from("fn"),
            Keyword::Struct => String::from("struct"),
            Keyword::Enum => String::from("enum"),
        }
    }
}
