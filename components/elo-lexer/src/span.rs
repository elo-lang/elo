use crate::inputfile::InputFile;

#[derive(Debug)]
pub struct FileSpan {
    pub input_file: InputFile,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl FileSpan {
    pub fn empty(input_file: InputFile) -> FileSpan {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}
