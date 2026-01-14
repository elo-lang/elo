use crate::{span::Span, token::Token};

#[derive(Clone, PartialEq, Eq)]
pub struct Lexem {
    pub span: Span,
    pub token: Token,
}

impl Lexem {
    pub fn new(span: Span, token: Token) -> Lexem {
        Lexem { span, token }
    }
}
