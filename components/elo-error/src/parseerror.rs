use elo_lexer::span::Span;

#[derive(Debug)]
pub enum ParseErrorCase {
    UnexpectedToken { got: String, expected: String },
}

#[derive(Debug)]
pub struct ParseError {
    pub span: Option<Span>,
    pub case: ParseErrorCase,
}
