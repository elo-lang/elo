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
}