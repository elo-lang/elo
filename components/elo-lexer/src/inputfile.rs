#[derive(Debug, Clone)]
pub struct InputFile<'a> {
    pub filename: &'static str,
    pub content: &'a str,
}

impl<'a> InputFile<'a> {
    pub fn new(filename: &'static str, content: &'a str) -> InputFile<'a> {
        InputFile { filename, content }
    }
}
