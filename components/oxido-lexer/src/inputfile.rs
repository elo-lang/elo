use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct InputFile<'a> {
    pub filename: &'static str,
    pub content: Peekable<Chars<'a>>,
}