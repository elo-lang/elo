use crate::keyword::Keyword;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum StringKind {
    Static,
    Dynamic,
    C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),       // if, else, while, etc
    Identifier(String),     // any identifier
    Integer(String, u32),   // Any integer literal + its base (decimal: 10, hex: 16, octal: 8, etc.)
    Float(String),          // Any floating-point literal, always decimal
    Newline,                // \n
    // This option is for the second character like in ==, <=, >=, etc
    // e.g. Token::Op('=', Some('=')) is "=="
    Op(char, Option<char>), // +, -, *, /, ==, etc
    Delimiter(char),        // (, ), [, ], {, }, ., , etc
    String(StringKind, String),     // 'foo', "bar", 'hello'
    Character(String),    // `a`, `b`, `\n`
    Variadic,               // ...: Special token for variadic functions for C FFI
    Unknown(char),          // Any other character
    InterpolationBegin,
    InterpolationEnd,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::InterpolationBegin => write!(f, "\\("),
            Token::InterpolationEnd => write!(f, ")"),
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Integer(s, _) => write!(f, "{}", s),
            Token::Float(s) => write!(f, "{}", s),
            Token::Newline => write!(f, "newline"),
            Token::Op(a, b) => {
                if let Some(b) = b {
                    write!(f, "{}{}", a, b)
                } else {
                    write!(f, "{}", a)
                }
            }
            Token::Delimiter(d) => write!(f, "{}", d),
            Token::String(k, s) => {
                let quot = match k {
                    StringKind::Static => "\'",
                    StringKind::Dynamic => "\"",
                    StringKind::C => "\'",
                };
                write!(f, "{}{quot}{}{quot}", if *k == StringKind::C { "c" } else { "" }, s)
            }
            Token::Character(s) => write!(f, "`{}`", s),
            Token::Variadic => write!(f, "..."),
            Token::Unknown(c) => write!(f, "{}", c),
        }
    }
}
