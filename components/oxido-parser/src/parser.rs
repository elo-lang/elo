use std::iter::Peekable;

use oxido_error::parseerror::{ParseError, ParseErrorCase};
use oxido_lexer::inputfile::InputFile;
use oxido_lexer::lexem::Lexem;
use oxido_lexer::lexer::Lexer;
use oxido_lexer::token::Token;

use crate::node::Node;
use crate::program::Program;
use crate::structure::{BinaryOperation, Expression, LetStatement, Structure};

pub const EOF: &str = "EOF";

pub struct Parser<'a> {
    pub inputfile: InputFile<'a>,
    pub lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let inputfile = lexer.span.input_file.clone();
        Parser {
            lexer: lexer.peekable(),
            inputfile,
        }
    }

    fn expect_numeric(&mut self) -> Result<String, ParseError> {
        match self.lexer.next() {
            Some(Lexem {
                token: Token::Numeric(num),
                ..
            }) => Ok(num),
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", other),
                    expected: "numeric".to_string(),
                },
            }),
            None => Err(ParseError {
                span: None,
                case: ParseErrorCase::UnexpectedToken {
                    got: EOF.to_string(),
                    expected: "numeric".to_string(),
                },
            }),
        }
    }

    fn parse_number(&mut self) -> Result<Expression, ParseError> {
        let int1 = self.expect_numeric()?;
        if let Some(lexem) = self.lexer.peek() {
            return match &lexem.token {
                Token::Delimiter('.') => {
                    self.lexer.next();
                    let int2 = self.expect_numeric()?;
                    Ok(Expression::FloatLiteral {
                        value: format!("{}.{}", int1, int2).parse().unwrap(),
                    })
                }
                _ => Ok(Expression::IntegerLiteral {
                    value: int1.parse().unwrap(),
                }),
            };
        }
        Ok(Expression::IntegerLiteral {
            value: int1.parse().unwrap(),
        })
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        let id1 = self.expect_identifier()?;
        if let Some(lexem) = self.lexer.peek() {
            return match &lexem.token {
                Token::Delimiter('.') => {
                    self.lexer.next();
                    let id2 = self.parse_identifier()?;
                    Ok(Expression::Parent {
                        parent: Box::new(Expression::Identifier { name: id1 }),
                        child: Box::new(id2),
                    })
                }
                _ => Ok(Expression::Identifier { name: id1 }),
            };
        }
        Ok(Expression::Identifier { name: id1 })
    }

    pub fn expect_token(&mut self, expect: Token) -> Result<Token, ParseError> {
        match self.lexer.next() {
            Some(lexem) => {
                let token = lexem.token;
                if token == expect {
                    Ok(token)
                } else {
                    Err(ParseError {
                        span: Some(lexem.span),
                        case: ParseErrorCase::UnexpectedToken {
                            got: format!("{:?}", token),
                            expected: format!("{:?}", expect),
                        },
                    })
                }
            }
            None => Err(ParseError {
                span: None,
                case: ParseErrorCase::UnexpectedToken {
                    got: EOF.to_string(),
                    expected: format!("{:?}", expect),
                },
            }),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.lexer.next() {
            Some(Lexem {
                token: Token::Identifier(ident),
                ..
            }) => Ok(ident),
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", other),
                    expected: "identifier".to_string(),
                },
            }),
            None => Err(ParseError {
                span: None,
                case: ParseErrorCase::UnexpectedToken {
                    got: EOF.to_string(),
                    expected: "identifier".to_string(),
                },
            }),
        }
    }

    fn expect_end(&mut self) -> Result<(), ParseError> {
        match self.lexer.next() {
            Some(Lexem {
                token: Token::Newline,
                ..
            }) => Ok(()),
            Some(Lexem {
                token: Token::Delimiter(';'),
                ..
            }) => Ok(()),
            None => Ok(()),
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", other),
                    expected: "end of statement".to_string(),
                },
            }),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        if let Some(lexem) = self.lexer.peek() {
            return match &lexem.token {
                Token::Numeric(_) => Ok(self.parse_number()?),
                Token::Identifier(_) => Ok(self.parse_identifier()?),
                Token::Op('(', None) => {
                    let expr = self.parse_expr()?;
                    self.expect_token(Token::Op(')', None))?;
                    Ok(expr)
                }
                other => Err(ParseError {
                    span: Some(lexem.span),
                    case: ParseErrorCase::UnexpectedToken {
                        got: format!("{:?}", other),
                        expected: "primary expression".to_string(),
                    },
                }),
            };
        }
        Err(ParseError {
            span: None,
            case: ParseErrorCase::UnexpectedToken {
                got: EOF.to_string(),
                expected: "primary expression".to_string(),
            },
        })
    }

    fn parse_expr(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;
        if let Some(lexem) = self.lexer.peek() {
            match lexem.token {
                Token::Op(a, b) => {
                    self.lexer.next();
                    let right: Expression = self.parse_expr()?;
                    expr = Expression::BinaryOperation {
                        operator: BinaryOperation::from_op(a, b),
                        left: Box::new(expr),
                        right: Box::new(right),
                    };
                }
                _ => {}
            }
        }
        Ok(expr)
    }

    fn parse_let_stmt(&mut self) -> Result<Structure, ParseError> {
        let ident = self.expect_identifier()?;
        let _ = self.expect_token(Token::Op('=', None))?;
        let expr = self.parse_expr()?;
        self.expect_end()?;
        Ok(Structure::LetStatement(LetStatement {
            binding: ident,
            assignment: expr,
        }))
    }

    fn parse_stmt_from_kw(&mut self, kw: &str) -> Result<Structure, ParseError> {
        match kw {
            "let" => self.parse_let_stmt(),
            _ => unreachable!("unexpected keyword: {}", kw),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut ast = vec![];
        while let Some(lexem) = self.lexer.next() {
            let x = match lexem.token {
                Token::Keyword(kw) => Node {
                    span: lexem.span,
                    structure: self.parse_stmt_from_kw(&kw)?,
                },
                _ => Node {
                    span: lexem.span,
                    structure: Structure::Expression(self.parse_expr()?),
                },
            };
            ast.push(x);
        }
        let p = Program { nodes: ast };
        Ok(p)
    }
}
