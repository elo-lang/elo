#[derive(Debug, PartialEq, Eq)]
pub enum Token {
  Keyword(String), // if, else, while, etc
  Identifier(String), // any identifier
  Numeric(String), // Any number literal
  Newline, // \n
  // This option is for the second character like in ==, <=, >=, etc
  // e.g. Token::Op('=', Some('=')) is "=="
  Op(char, Option<char>), // +, -, *, /, ==, etc
  Delimiter(char), // (, ), [, ], {, }, ., , etc
  StringLiteral(String), // "foo", "bar", "hello"
  Unknown(char),
}