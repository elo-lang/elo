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
    CharLiteral(String),      // `a`, `b`, `\n`
    StringLiteral(String),  // "foo", "bar", "hello"
    Variadic,               // ...: Special token for variadic functions for C FFI
    Unknown(char),          // Any other character
}
