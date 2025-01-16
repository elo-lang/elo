use crate::word::Word;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub struct InputFile<'a> {
    pub filename: Word,
    pub content: Peekable<Chars<'a>>,
}