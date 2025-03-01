use std::iter::Peekable;

use elo_error::parseerror::{ParseError, ParseErrorCase};
use elo_lexer::inputfile::InputFile;
use elo_lexer::keyword::Keyword;
use elo_lexer::lexem::Lexem;
use elo_lexer::lexer::Lexer;
use elo_lexer::token::Token;

use elo_ast::ast::Node;
use elo_ast::ast::Program;
use elo_ast::ast::*;

pub type Precedence = u8;

fn toint(literal: &str, radix: u32) -> i128 {
    i128::from_str_radix(literal, radix).unwrap()
}

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

    fn expect_numeric(&mut self) -> Result<(String, u32), ParseError> {
        match self.lexer.next() {
            Some(Lexem {
                token: Token::Numeric(num, base),
                ..
            }) => Ok((num, base)),
            Some(Lexem {
                token: Token::Newline,
                ..
            }) => return self.expect_numeric(),
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

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        if let Some(lexem) = self.lexer.next() {
            match lexem.token {
                Token::Identifier(x) => {
                    if let Some(Lexem {
                        token: Token::Op('<', None),
                        ..
                    }) = self.lexer.peek()
                    {
                        self.lexer.next();
                        let generic = self.parse_type()?;
                        self.expect_token(Token::Op('>', None))?;
                        return Ok(Type::Named {
                            name: x,
                            generic: Some(Box::new(generic)),
                        });
                    }
                    return Ok(Type::Named {
                        name: x,
                        generic: None,
                    });
                }
                Token::Op('*', None) => {
                    let typ = self.parse_type()?;
                    return Ok(Type::Pointer { typ: Box::new(typ) });
                }
                Token::Delimiter('[') => {
                    let typ = self.parse_type()?;
                    self.expect_token(Token::Delimiter(','))?;
                    match self.parse_number()? {
                        Expression::IntegerLiteral { value: x } => {
                            let x = toint(&x.0, x.1) as usize;
                            self.expect_token(Token::Delimiter(']'))?;
                            return Ok(Type::Array {
                                typ: Box::new(typ),
                                amount: x,
                            });
                        }
                        Expression::FloatLiteral { .. } => {
                            return Err(ParseError {
                                span: Some(lexem.span),
                                case: ParseErrorCase::UnexpectedToken {
                                    got: "float literal".to_string(),
                                    expected: "integer literal".to_string(),
                                },
                            });
                        }
                        _ => unreachable!(),
                    }
                }
                x => {
                    return Err(ParseError {
                        span: Some(lexem.span),
                        case: ParseErrorCase::UnexpectedToken {
                            got: format!("{:?}", x),
                            expected: "type".to_string(),
                        },
                    });
                }
            }
        }
        return Err(ParseError {
            span: None,
            case: ParseErrorCase::UnexpectedToken {
                got: EOF.to_string(),
                expected: "type".to_string(),
            },
        });
    }

    fn parse_typed_field(&mut self) -> Result<TypedField, ParseError> {
        let ident = self.expect_identifier()?;
        self.expect_token(Token::Delimiter(':'))?;
        let typ = self.parse_type()?;
        return Ok(TypedField {
            name: ident,
            typing: typ,
        });
    }

    fn parse_field(&mut self) -> Result<Field, ParseError> {
        let ident = self.expect_identifier()?;
        self.expect_token(Token::Delimiter(':'))?;
        let value = self.parse_expr(1)?;
        return Ok(Field {
            name: ident,
            value: value,
        });
    }

    // identifier: type[, identifier: type]*
    fn parse_typed_fields(&mut self) -> Result<Vec<TypedField>, ParseError> {
        let mut fields = Vec::new();
        if let Ok(first) = self.parse_typed_field() {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
            let f = self.parse_typed_field()?;
            fields.push(f);
        }
        Ok(fields)
    }

    // identifier: expr[, identifier: expr]*
    fn parse_fields(&mut self) -> Result<Vec<Field>, ParseError> {
        let mut fields = Vec::new();
        if let Ok(first) = self.parse_field() {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
            let f = self.parse_field()?;
            fields.push(f);
        }
        Ok(fields)
    }

    // expr[, expr]*
    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut fields = Vec::new();
        if let Ok(first) = self.parse_expr(1) {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
            let f = self.parse_expr(1)?;
            fields.push(f);
        }
        Ok(fields)
    }

    // identifier[, identifier]*
    fn parse_identifier_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut fields = Vec::new();
        if let Ok(first) = self.expect_identifier() {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
            let f = self.expect_identifier()?;
            fields.push(f);
        }
        Ok(fields)
    }

    fn parse_number(&mut self) -> Result<Expression, ParseError> {
        let int = self.expect_numeric()?;
        if let Some(lexem) = self.lexer.peek() {
            return match &lexem.token {
                Token::Delimiter('.') => {
                    self.lexer.next();
                    let float = self.expect_numeric()?;
                    Ok(Expression::FloatLiteral {
                        int: int,
                        float: float,
                    })
                }
                _ => Ok(Expression::IntegerLiteral { value: int }),
            };
        }
        Ok(Expression::IntegerLiteral { value: int })
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        let id1 = self.expect_identifier()?;
        Ok(Expression::Identifier { name: id1 })
    }

    // Check if a token is present at the next iteration. Only consumes if the condition is met.
    // Ignores newlines.
    pub fn test_token(&mut self, expect: Token) -> Result<Lexem, ParseError> {
        match self.lexer.peek() {
            Some(lexem) if lexem.token == expect => {
                let x = lexem.clone();
                self.lexer.next();
                Ok(x)
            }
            Some(Lexem { token: Token::Newline, .. }) => {
                self.lexer.next();
                return self.test_token(expect)
            },
            Some(lexem) => Err(ParseError {
                span: Some(lexem.span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", lexem.token),
                    expected: format!("{:?}", expect),
                },
            }),
            None => Err(ParseError {
                span: None,
                case: ParseErrorCase::UnexpectedToken {
                    got: EOF.to_string(),
                    expected: format!("{:?}", expect),
                },
            }),
        }
    }

    // Expects a specific token in the next iteration of lexems. Always consumes the iterator.
    // If the next token is a newline, ignores it and goes to the next iteration.
    pub fn expect_token(&mut self, expect: Token) -> Result<Token, ParseError> {
        match self.lexer.next() {
            Some(Lexem {
                token: Token::Newline,
                ..
            }) => return self.expect_token(expect),
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
        let a = self.lexer.peek();
        match a {
            Some(Lexem {
                token: Token::Identifier(ident),
                ..
            }) => {
                let ident: String = ident.clone();
                self.lexer.next();
                Ok(ident)
            }
            Some(Lexem {
                token: Token::Newline,
                ..
            }) => {
                self.lexer.next();
                return self.expect_identifier();
            }
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(*span),
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
                Token::Newline => {
                    self.lexer.next();
                    return self.parse_primary();
                }
                Token::Numeric(..) => return Ok(self.parse_number()?),
                Token::Identifier(_) => return Ok(self.parse_identifier()?),
                Token::Delimiter('(') => {
                    self.lexer.next();
                    let expr = self.parse_expr(1)?;
                    self.expect_token(Token::Delimiter(')'))?;
                    return Ok(expr);
                }
                Token::Keyword(Keyword::Struct) => {
                    self.lexer.next();
                    let i = self.expect_identifier()?;
                    self.expect_token(Token::Delimiter('{'))?;
                    let fields = self.parse_fields()?;
                    self.expect_token(Token::Delimiter('}'))?;
                    return Ok(Expression::StructInit { name: i, fields });
                }
                token @ Token::Op(a, b) => {
                    let op = UnaryOperation::from_op(*a, b.as_ref().copied());
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
                }
                other => {
                    return Err(ParseError {
                        span: Some(lexem.span),
                        case: ParseErrorCase::UnexpectedToken {
                            got: format!("{:?}", other),
                            expected: "primary expression".to_string(),
                        },
                    })
                }
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
                let binop = BinaryOperation::from_op(a, b);
                let right = self.parse_expr(next_limit)?;
                left = Expression::BinaryOperation {
                    operator: binop.unwrap(),
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
        }
        // Field access (e.g. instance.method(), foo.bar)
        if let Ok(_) = self.test_token(Token::Delimiter('.')) {
            let field = self.expect_identifier()?;
            left = Expression::FieldAccess {
                origin: Box::new(left),
                field: field,
            };
        }
        // Function call (e.g. foo(), bar())
        if let Ok(_) = self.test_token(Token::Delimiter('(')) {
            let func = left;
            let args = self.parse_expression_list()?;
            self.expect_token(Token::Delimiter(')'))?;
            left = Expression::FunctionCall {
                function: Box::new(func),
                arguments: args,
            };
        }
        Ok(left)
    }

    fn parse_assignment(&mut self) -> Result<(String, Expression), ParseError> {
        let ident = self.expect_identifier()?;
        let _ = self.expect_token(Token::Op('=', None))?;
        let expr = self.parse_expr(1)?;
        Ok((ident, expr))
    }

    fn parse_let_stmt(&mut self) -> Result<Statement, ParseError> {
        let (ident, expr) = self.parse_assignment()?;
        self.expect_end()?;
        Ok(Statement::LetStatement(LetStatement {
            binding: ident,
            assignment: expr,
        }))
    }

    fn parse_const_stmt(&mut self) -> Result<Statement, ParseError> {
        let ident = self.expect_identifier()?;
        let _ = self.expect_token(Token::Delimiter(':'))?;
        let typing = self.parse_type()?;
        let _ = self.expect_token(Token::Op('=', None))?;
        let expr = self.parse_expr(1)?;
        self.expect_end()?;
        Ok(Statement::ConstStatement(ConstStatement {
            binding: ident,
            assignment: expr,
            typing: typing,
        }))
    }

    fn parse_var_stmt(&mut self) -> Result<Statement, ParseError> {
        let (ident, expr) = self.parse_assignment()?;
        self.expect_end()?;
        Ok(Statement::VarStatement(VarStatement {
            binding: ident,
            assignment: expr,
        }))
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut ast = vec![];
        while let Some(node) = self.parse_node(true)? {
            ast.push(node);
        }
        let p = Block { content: ast };
        Ok(p)
    }

    fn parse_fn_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('('))?;
        let arguments = self.parse_typed_fields()?;
        self.expect_token(Token::Delimiter(')'))?;
        let mut typ = None;
        if let Ok(_) = self.test_token(Token::Delimiter(':')) {
            typ = Some(self.parse_type()?);
        }
        self.expect_token(Token::Delimiter('{'))?;
        let block = self.parse_block()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::FnStatement(FnStatement {
            name: name,
            block: block,
            ret: typ,
            arguments: arguments,
        }))
    }

    fn parse_struct_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('{'))?;
        let fields = self.parse_typed_fields()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::StructStatement(StructStatement {
            name: name,
            fields: fields,
        }))
    }

    fn parse_enum_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('{'))?;
        let vars = self.parse_identifier_list()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::EnumStatement(EnumStatement {
            name: name,
            variants: vars,
        }))
    }

    fn parse_if_stmt(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expr(1)?;
        self.expect_token(Token::Delimiter('{'))?;
        let block_true = self.parse_block()?;
        self.expect_token(Token::Delimiter('}'))?;
        let mut block_false: Option<Block> = None;
        if let Ok(_) = self.test_token(Token::Keyword(Keyword::Else)) {
            if let Ok(elseif) = self.test_token(Token::Keyword(Keyword::If)) {
                let if_node = Node {
                    span: elseif.span,
                    stmt: self.parse_if_stmt()?,
                };
                block_false = Some(Block { content: vec![if_node] });
            } else {
                self.expect_token(Token::Delimiter('{'))?;
                block_false = Some(self.parse_block()?);
                self.expect_token(Token::Delimiter('}'))?;
            }
        }
        Ok(Statement::IfStatement(IfStatement {
            condition: expr,
            block_false, block_true
        }))
    }

    fn parse_stmt(&mut self) -> Result<Statement, ParseError> {
        if let Some(Lexem {
            token: Token::Keyword(kw),
            span,
        }) = self.lexer.next()
        {
            match kw {
                Keyword::Var => return self.parse_var_stmt(),
                Keyword::Let => return self.parse_let_stmt(),
                Keyword::Const => return self.parse_const_stmt(),
                Keyword::Fn => return self.parse_fn_stmt(),
                Keyword::Struct => return self.parse_struct_stmt(),
                Keyword::Enum => return self.parse_enum_stmt(),
                Keyword::If => return self.parse_if_stmt(),
                Keyword::Else => {
                    return Err(ParseError {
                        span: Some(span),
                        case: ParseErrorCase::UnexpectedToken {
                            got: "else keyword".to_string(),
                            expected: "valid statement".to_string(),
                        },
                    });
                }
            }
        } else {
            unreachable!("asked to parse statement without keyword")
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_node(&mut self, inside_block: bool) -> Result<Option<Node>, ParseError> {
        let mut x = None;
        if let Some(lexem) = self.lexer.peek() {
            x = Some(match lexem.token {
                Token::Newline => {
                    self.lexer.next();
                    return self.parse_node(inside_block);
                }
                // Account for trailing } when terminating block
                Token::Delimiter('}') if inside_block => {
                    return Ok(None);
                }
                Token::Keyword(..) => Node {
                    span: lexem.span,
                    stmt: self.parse_stmt()?,
                },
                _ => {
                    let span = lexem.span;
                    let token = lexem.token.clone();
                    // Ensure that the next token is an token valid for an expression. Otherwise, stop parsing.
                    if let Ok(expr) = self.parse_expr(1) {
                        let node = Node {
                            span,
                            stmt: Statement::ExpressionStatement(expr),
                        };
                        self.expect_end()?;
                        node
                    } else {
                        return Err(ParseError {
                            span: Some(span),
                            case: ParseErrorCase::UnexpectedToken {
                                got: format!("{:?}", token),
                                expected: "expression".to_string(),
                            },
                        });
                    }
                }
            });
        }
        return Ok(x);
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut ast = vec![];
        while let Some(node) = self.parse_node(false)? {
            ast.push(node);
        }
        let p = Program { nodes: ast };
        Ok(p)
    }
}
