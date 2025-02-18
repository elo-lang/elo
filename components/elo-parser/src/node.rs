use crate::ast::Statement;
use elo_lexer::span::Span;

#[derive(Debug)]
pub struct Node {
    pub span: Span,
    pub stmt: Statement,
}
