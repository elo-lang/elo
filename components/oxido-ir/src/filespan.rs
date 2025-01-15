use crate::inputfile::InputFile;

#[derive(Debug, PartialEq)]
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
}