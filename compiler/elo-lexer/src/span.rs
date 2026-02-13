// TODO: Support for multiline spans. This can cause problems later in the parser....

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    // Merges the span with the other.
    // Maintains the same line as self.
    // Example:
    // let a = Span { line: 5, start: 10, end: 11 };
    // let b = Span { line: 5, start: 15, end: 22 };
    // let c = a.merge(b);
    // assert_eq!(c, Span { line: 5, start: 10, end: 22 })

    pub fn default(input_file: InputFile<'_>) -> Self {
        Span {
            line: input_file.content.lines().count(),
            start: 0, end: 1
        }
    }

    pub fn merge(&self, other: Span) -> Span {
        Span {
            line: self.line,
            start: self.start,
            end: other.end,
        }
    }

    pub fn into_filespan<'a>(&self, input_file: InputFile<'a>) -> FileSpan<'a> {
        FileSpan {
            input_file,
            line: self.line,
            start: self.start,
            end: self.end,
        }
    }
}
