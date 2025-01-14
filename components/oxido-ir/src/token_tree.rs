use crate::{
  input_file::InputFile,
  span::Span,
  token::Token
};

#[derive(Debug)]
pub struct TokenTree {
  pub input_file: InputFile,
  pub span: Span,
  pub tokens: Vec<Token>,
}

impl TokenTree {
  #[allow(clippy::len_without_is_empty)]
  pub fn len(self) -> u32 {
    self.span.len()
  }

  /*pub fn spanned_tokens(self) -> impl Iterator<Item = (Span, Token)> + '_ {
    let mut start = self.span.start;
    self.tokens.iter().map(move |token| {
      let len = token.span_len();
      let span = Span::from(start, start + len);
      start = start + len;
      (span, *token)
    })
  }*/
}