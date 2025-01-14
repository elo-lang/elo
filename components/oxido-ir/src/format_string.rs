//! String literals in Dada are actually kind of complex.
//! They can include expressions and so forth.

use crate::{
  token_tree::TokenTree,
  word::Word
};

#[derive(Debug)]
pub struct FormatString {
  pub len: u32,

  /// List of sections from a string like `"foo{bar}baz" -- that example would
  /// have three sections.
  pub sections: Vec<FormatStringSection>,
}

impl FormatString {
  /// True if the format string is empty.
  pub fn is_empty(self) -> bool {
    self.len == 0
  }
}

#[derive(Debug)]
pub struct FormatStringSection {
  pub data: FormatStringSectionData,
}

#[derive(Debug)]
pub enum FormatStringSectionData {
  /// Plain text to be emitted directly.
  Text(Word),

  /// A token tree for an expression.
  TokenTree(TokenTree)
}

impl FormatStringSection {
  #[allow(clippy::len_without_is_empty)]
  pub fn len(self) -> u32 {
    match self.data {
      FormatStringSectionData::Text(w) => w.len(),
      FormatStringSectionData::TokenTree(tree) => tree.len(),
    }
  }
}