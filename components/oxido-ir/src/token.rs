use crate::word::Word;

#[derive(Debug)]
pub enum Token {
  Alphabetic(Word), // any identifier
  Number(Word), // Any number literal
  Comma, // ,
  Newline, // \n
  Op(char), // +, -, *, /, ==, etc
  Delimiter(char), // (, ), [, ], {, }, ., etc
  StringLiteral(String), // "foo", "bar", "hello"
  Whitespace(char), // ' ', '\t', '\r', etc
  Unknown(char),
}