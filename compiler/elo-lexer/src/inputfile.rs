#[derive(Debug, Clone, Copy)]
pub struct InputFile<'a> {
    pub filename: &'a str,
    pub content: &'a str,
}

impl<'a> InputFile<'a> {
    pub fn new(filename: &'a str, content: &'a str) -> InputFile<'a> {
        InputFile { filename, content }
    }
}
