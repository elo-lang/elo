use crate::ast::Structure;
use elo_lexer::span::Span;

#[derive(Debug)]
pub struct Node {
    pub span: Span,
    pub structure: Structure,
}
