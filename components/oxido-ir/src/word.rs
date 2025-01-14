#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub string: String,
}

impl Word {
    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn to_string(self) -> String {
        self.string.clone()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(self) -> u32 {
        self.as_str().len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        let w = Word {
            string: "test.dada".to_owned(),
        };
        assert_eq!("test.dada", w.as_str())
    }

    #[test]
    fn test_to_string() {
        let w = Word {
            string: "test.dada".to_owned(),
        };
        assert_eq!(String::from("test.dada"), w.to_string())
    }

    #[test]
    fn test_len() {
        let w = Word {
            string: "test.dada".to_owned(),
        };
        assert_eq!(9, w.len())
    }
}
