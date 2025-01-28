use crate::inputfile::InputFile;

#[derive(Debug)]
pub struct FileSpan<'a> {
    pub input_file: InputFile<'a>,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl<'a> FileSpan<'a> {
    pub fn empty(input_file: InputFile<'a>) -> FileSpan<'a> {
        FileSpan {
            input_file,
            line: 1,
            start: 0,
            end: 0,
        }
    }

    pub fn into_span(&self) -> Span {
        Span {
            line: self.line,
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}