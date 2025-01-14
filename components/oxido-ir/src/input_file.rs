use crate::{
    span::{Anchored, LineColumn},
    word::Word,
};

#[derive(Debug, Clone, PartialEq)]
pub struct InputFile {
    pub name: Word,

    /// The raw contents of this input file, as a string.
    pub source_text: String,
    // The locations of any breakpoints set in this file
    pub breakpoint_locations: Vec<LineColumn>,
}

impl InputFile {
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }
}

impl Anchored for InputFile {
    fn input_file(&self) -> InputFile {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::span::LineColumn;

    use super::*;

    #[test]
    fn test_name_str() {
        let w = Word {
            string: "test.dada".to_owned(),
        };
        let input_file = InputFile {
            name: w,
            source_text: "fn main() {\n print(\"hello world\").await\n}".to_owned(),
            breakpoint_locations: vec![LineColumn::new1(4, 3)],
        };

        assert_eq!("test.dada", input_file.name_str())
    }
}
