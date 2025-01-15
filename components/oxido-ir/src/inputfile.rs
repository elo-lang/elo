use crate::word::Word;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct InputFile<'a> {
    pub filename: Word,
    pub content: Chars<'a>,
}