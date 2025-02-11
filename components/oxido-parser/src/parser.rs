use std::iter::Peekable;

use oxido_error::parseerror::{ParseError, ParseErrorCase};
use oxido_lexer::inputfile::InputFile;
use oxido_lexer::keyword::Keyword;
use oxido_lexer::lexem::Lexem;
use oxido_lexer::lexer::Lexer;
use oxido_lexer::token::Token;

use crate::node::Node;
use crate::program::Program;
use crate::ast::{BinaryOperation, Expression, LetStatement, Structure, UnaryOperation};

pub type Precedence = u8;

fn binop_precedence(token: &Token) -> Precedence {
    match token {
        Token::Op('=', None) => 1,
        Token::Op('=', Some('=')) => 2,
        Token::Op('!', Some('=')) => 2,
        Token::Op('<', Some('=')) => 3,
        Token::Op('>', Some('=')) => 3,
        Token::Op('<', None) => 3,
        Token::Op('>', None) => 3,
        Token::Op('&', Some('&')) => 4,
        Token::Op('|', Some('|')) => 4,
        Token::Op('&', None) => 5,
        Token::Op('|', None) => 5,
        Token::Op('^', None) => 5,
        Token::Op('+', None) => 6,
        Token::Op('-', None) => 6,
        Token::Op('*', None) => 7,
        Token::Op('/', None) => 7,
        Token::Op('%', None) => 7,
        Token::Op('<', Some('<')) => 8,
        Token::Op('>', Some('>')) => 8,
        // This is zero because the check made in parse_expr() will be false because
        // you have to start parsing expressions starting from limit = 1.
        // This way it just stops parsing when it finds something strange
        _ => 0,
    }
}

fn unop_precedence(op: &Token) -> Precedence {
    match op {
        Token::Op('!', None) => 9,
        Token::Op('-', None) => 9,
        Token::Op('~', None) => 9,
        Token::Op('&', None) => 9,
        // This is zero because the check made in parse_expr() will be false because
        // you have to start parsing expressions starting from limit = 1.
        // This way it just stops parsing when it finds something strange
        _ => 0,
    }
}

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
            None => {
                Err(ParseError {
                    span: None,
                    case: ParseErrorCase::UnexpectedToken {
                        got: EOF.to_string(),
                        expected: format!("{:?}", expect),
                    },
                })
            },
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
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", other),
                    expected: "end of statement".to_string(),
                },
            }),
            None => Ok(()),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        if let Some(lexem) = self.lexer.peek() {
            match &lexem.token {
                Token::Numeric(_) => return Ok(self.parse_number()?),
                Token::Identifier(_) => return Ok(self.parse_identifier()?),
                Token::Delimiter('(') => {
                    self.lexer.next();
                    let expr = self.parse_expr(1)?;
                    self.expect_token(Token::Delimiter(')'))?;
                    return Ok(expr);
                },
                token@Token::Op(a, b) => {
                    let op = UnaryOperation::from_op(*a,b.as_ref().copied());
                    if let Some(unop) = op {
                        let prec = unop_precedence(token);
                        self.lexer.next();
                        return Ok(Expression::UnaryOperation {
                            operator: unop,
                            operand: Box::new(self.parse_expr(prec)?),
                        });
                    }
                    return Err(ParseError {
                        span: Some(lexem.span),
                        case: ParseErrorCase::UnexpectedToken {
                            got: format!("{:?}", token),
                            expected: "primary expression".to_string(),
                        },
                    });
                },
                other => return Err(ParseError {
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

    fn parse_expr(&mut self, limit: Precedence) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;
        while let Some(op) = self.lexer.peek() {
            let next_limit = binop_precedence(&op.token);
            if limit > next_limit {
                break;
            }
            if let Some(Lexem {
                token: Token::Op(a, b),
                ..
            }) = self.lexer.next()
            {
                let right = self.parse_expr(next_limit)?;
                left = Expression::BinaryOperation {
                    operator: BinaryOperation::from_op(a, b).unwrap(),
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }
        Ok(left)
    }

    fn parse_let_stmt(&mut self) -> Result<Structure, ParseError> {
        let ident = self.expect_identifier()?;
        let _ = self.expect_token(Token::Op('=', None))?;
        let expr = self.parse_expr(1)?;
        self.expect_end()?;
        Ok(Structure::LetStatement(LetStatement {
            binding: ident,
            assignment: expr,
        }))
    }

    fn parse_stmt(&mut self) -> Result<Structure, ParseError> {
        if let Some(Lexem {
            token: Token::Keyword(kw),
            ..
        }) = self.lexer.next()
        {
            match kw {
                Keyword::Var => todo!("var statement"),
                Keyword::Let => return self.parse_let_stmt(),
                Keyword::Const => todo!("const statement"),
                Keyword::Fn => todo!("fn statement"),
                Keyword::Struct => todo!("struct statement"),
                Keyword::Enum => todo!("enum statement"),
            }
        } else {
            unreachable!("asked to parse statement without keyword")
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut ast = vec![];
        while let Some(lexem) = self.lexer.peek() {
            let x = match lexem.token {
                Token::Keyword(..) => Node {
                    span: lexem.span,
                    structure: self.parse_stmt()?,
                },
                _ => Node {
                    span: lexem.span,
                    structure: Structure::Expression(self.parse_expr(1)?),
                },
            };
            ast.push(x);
        }
        let p = Program { nodes: ast };
        Ok(p)
    }
}
