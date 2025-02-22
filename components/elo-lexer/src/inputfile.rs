#[derive(Debug, Clone)]
pub struct InputFile {
    pub filename: &'static str,
    pub content: &'static str,
}

impl InputFile {
    pub fn new(filename: &'static str, content: &'static str) -> InputFile {
        InputFile { filename, content }
    }
}
