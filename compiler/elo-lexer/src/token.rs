use crate::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),     // if, else, while, etc
    Identifier(String),   // any identifier
    Numeric(String, u32), // Any number literal
    Newline,              // \n
    // This option is for the second character like in ==, <=, >=, etc
    // e.g. Token::Op('=', Some('=')) is "=="
    Op(char, Option<char>), // +, -, *, /, ==, etc
    Delimiter(char),        // (, ), [, ], {, }, ., , etc
    StrLiteral(String),     // 'foo', 'bar', 'hello'
    CharLiteral(String),    // `a`, `b`, `\n`
    StringLiteral(String),  // "foo", "bar", "hello"
    Variadic,               // ...: Special token for variadic functions for C FFI
    Unknown(char),          // Any other character
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Numeric(s, _) => write!(f, "{}", s),
            Token::Newline => write!(f, "newline"),
            Token::Op(a, b) => {
                if let Some(b) = b {
                    write!(f, "{}{}", a, b)
                } else {
                    write!(f, "{}", a)
                }
            }
            Token::Delimiter(d) => write!(f, "{}", d),
            Token::StrLiteral(s) => write!(f, "\'{}\'", s),
            Token::CharLiteral(s) => write!(f, "`{}`", s),
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),
            Token::Variadic => write!(f, "..."),
            Token::Unknown(c) => write!(f, "{}", c),
        }
    }
}
