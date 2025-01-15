use crate::word::Word;

#[derive(Debug, PartialEq, Clone)]
pub struct InputFile {
    pub filename: Word,
    pub content: String,
}