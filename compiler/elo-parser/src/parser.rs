use std::iter::Peekable;

use elo_error::parseerror::{ParseError, ParseErrorCase};
use elo_lexer::inputfile::InputFile;
use elo_lexer::keyword::Keyword;
use elo_lexer::lexem::Lexem;
use elo_lexer::lexer::Lexer;
use elo_lexer::span::Span;
use elo_lexer::token::Token;

use elo_ir::ast::Node;
use elo_ir::ast::Program;
use elo_ir::ast::*;

pub type Precedence = u8;

fn toint(literal: &str, radix: u32) -> i128 {
    i128::from_str_radix(literal, radix).unwrap()
}

fn binop_precedence(token: &Token) -> Precedence {
    match token {
        Token::Op('=', None)      => 1,
        Token::Op('=', Some('=')) => 2,
        Token::Op('!', Some('=')) => 2,
        Token::Op('<', Some('=')) => 3,
        Token::Op('>', Some('=')) => 3,
        Token::Op('<', None)      => 3,
        Token::Op('>', None)      => 3,
        Token::Op('&', Some('&')) => 4,
        Token::Op('|', Some('|')) => 4,
        Token::Op('&', None)      => 5,
        Token::Op('|', None)      => 5,
        Token::Op('^', None)      => 5,
        Token::Op('+', None)      => 6,
        Token::Op('-', None)      => 6,
        Token::Op('*', None)      => 7,
        Token::Op('/', None)      => 7,
        Token::Op('%', None)      => 7,
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
        Token::Op('*', None) => 9,
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
    current_span: Option<Span>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let inputfile = lexer.span.input_file;
        Parser {
            lexer: lexer.peekable(),
            inputfile,
            current_span: None,
        }
    }

    fn expect_numeric(&mut self) -> Result<(String, u32), ParseError> {
        match self.next() {
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
        if let Some(lexem) = self.next() {
            match lexem.token {
                Token::Newline => return self.parse_type(),
                Token::Identifier(x) => {
                    if let Some(Lexem {
                        token: Token::Op('<', None),
                        ..
                    }) = self.lexer.peek()
                    {
                        self.next();
                        let generic = self.parse_type()?;
                        self.expect_token(Token::Op('>', None))?;
                        return Ok(Type {
                            span: lexem.span.merge(self.current_span.unwrap()),
                            typing: Typing::Named {
                                name: x,
                                generic: Some(Box::new(generic)),
                            },
                        });
                    }
                    return Ok(Type {
                        span: lexem.span,
                        typing: Typing::Named {
                            name: x,
                            generic: None,
                        },
                    });
                }
                Token::Op('*', None) => {
                    let typ = self.parse_type()?;
                    return Ok(Type {
                        span: lexem.span.merge(self.current_span.unwrap()),
                        typing: Typing::Pointer { typ: Box::new(typ) },
                    });
                }
                Token::Delimiter('{') => {
                    let typ = self.parse_type()?;
                    self.expect_token(Token::Delimiter(';'))?;
                    match self.parse_number()?.data {
                        ExpressionData::IntegerLiteral { value: x } => {
                            let x = toint(&x.0, x.1) as usize;
                            self.expect_token(Token::Delimiter('}'))?;
                            return Ok(Type {
                                span: lexem.span.merge(self.current_span.unwrap()),
                                typing: Typing::Array {
                                    typ: Box::new(typ),
                                    amount: x,
                                },
                            });
                        }
                        ExpressionData::FloatLiteral { .. } => {
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
                Token::Delimiter('(') => {
                    let mut types = Vec::new();
                    if let Ok(first) = self.parse_type() {
                        types.push(first);
                    }
                    while let Some(Lexem {
                        token: Token::Delimiter(','),
                        ..
                    }) = self.lexer.peek()
                    {
                        self.next();
                        let t = self.parse_type()?;
                        types.push(t);
                    }
                    self.expect_token(Token::Delimiter(')'))?;
                    return Ok(Type {
                        span: lexem.span.merge(self.current_span.unwrap()),
                        typing: Typing::Tuple { types },
                    });
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
        let value = self.parse_expr(1, true)?;
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
            self.next();
            let f = self.parse_typed_field()?;
            fields.push(f);
        }
        Ok(fields)
    }

    // Parse function declaration arguments, this is made to include parsing of the ... token for
    // variadic functions, in compatibility with C FFI.
    //                                                           ---- This field means if the arguments are variadic or not.
    fn parse_fn_decl_args(&mut self) -> Result<(Vec<TypedField>, bool), ParseError> {
        let mut fields = Vec::new();
        let mut variadic = false;
        // Consider also the first argument could be ...
        if let Some(_) = self.test_token(Token::Variadic, true) {
            variadic = true;
            return Ok((fields, variadic));
        }
        if let Ok(first) = self.parse_typed_field() {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.next();
            if let Some(Lexem {
                token: Token::Variadic,
                ..
            }) = self.lexer.peek()
            {
                variadic = true;
                self.next();
                break;
            }
            let f = self.parse_typed_field()?;
            fields.push(f);
        }
        Ok((fields, variadic))
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
            self.next();
            let f = self.parse_field()?;
            fields.push(f);
        }
        Ok(fields)
    }

    // expr[, expr]*[,]?
    fn parse_expression_list(&mut self, termination: Token) -> Result<Vec<Expression>, ParseError> {
        let mut fields = Vec::new();
        if let Some(Lexem {
            token: Token::Delimiter(')'),
            ..
        }) = self.lexer.peek()
        {
            return Ok(fields);
        }
        let first = self.parse_expr(1, true)?;
        fields.push(first);
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.next();
            if let Some(Lexem { token, .. }) = self.lexer.peek() {
                if token == &termination {
                    break;
                }
            }
            let expr = self.parse_expr(1, true)?;
            fields.push(expr);
        }
        Ok(fields)
    }

    // identifier[, identifier]*
    fn parse_enum_variants(&mut self) -> Result<Vec<String>, ParseError> {
        let mut fields = Vec::new();
        if let Ok(first) = self.expect_identifier() {
            fields.push(first);
        }
        while let Some(Lexem {
            token: Token::Delimiter(','),
            ..
        }) = self.lexer.peek()
        {
            self.next();
            let f = self.expect_identifier()?;
            fields.push(f);
        }
        Ok(fields)
    }

    fn parse_number(&mut self) -> Result<Expression, ParseError> {
        let int = self.expect_numeric()?;
        let span = self.current_span.unwrap();
        if let Some(lexem) = self.lexer.peek() {
            return match &lexem.token {
                Token::Delimiter('.') => {
                    self.next();
                    let float = self.expect_numeric()?;
                    Ok(Expression {
                        span: span.merge(self.current_span.unwrap()),
                        data: ExpressionData::FloatLiteral {
                            int: int,
                            float: float,
                        },
                    })
                }
                _ => Ok(Expression {
                    span: span,
                    data: ExpressionData::IntegerLiteral { value: int },
                }),
            };
        }
        Ok(Expression {
            span: span,
            data: ExpressionData::IntegerLiteral { value: int },
        })
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParseError> {
        let id1 = self.expect_identifier()?;
        Ok(Expression {
            span: self.current_span.unwrap(),
            data: ExpressionData::Identifier { name: id1 },
        })
    }

    // Check if a token is present at the next iteration. Only consumes if the condition is met.
    // Does not ignore newlines by default, unless `lazy` argument is set to true.
    // If you put a Token::Newline in the `expect` argument and `lazy` is true,
    // it will skip all newlines until it finds the expected token, it will always return None.
    pub fn test_token(&mut self, expect: Token, lazy: bool) -> Option<Lexem> {
        match self.lexer.peek() {
            Some(lexem) if lexem.token == expect => {
                let x = lexem.clone();
                self.next();
                Some(x)
            }
            Some(Lexem {
                token: Token::Newline,
                ..
            }) if lazy => {
                self.next();
                self.test_token(expect, lazy)
            }
            _ => None,
        }
    }

    // Expects a specific token in the next iteration of lexems. Always consumes the iterator.
    // If the next token is a newline, ignores it and goes to the next iteration.
    pub fn expect_token(&mut self, expect: Token) -> Result<Token, ParseError> {
        match self.next() {
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
        match self.lexer.peek() {
            Some(Lexem {
                token: Token::Identifier(ident),
                ..
            }) => {
                let ident: String = ident.clone();
                self.next();
                Ok(ident)
            }
            Some(Lexem {
                token: Token::Newline,
                ..
            }) => {
                self.next();
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
        match self.lexer.peek() {
            Some(Lexem {
                token: Token::Newline | Token::Delimiter(';'),
                ..
            }) => {
                self.next();
                return Ok(());
            }
            Some(Lexem {
                token: Token::Delimiter('}'),
                ..
            }) => Ok(()),
            Some(Lexem { token: other, span }) => Err(ParseError {
                span: Some(*span),
                case: ParseErrorCase::UnexpectedToken {
                    got: format!("{:?}", other),
                    expected: "end of statement".to_string(),
                },
            }),
            None => Ok(()),
        }
    }

    fn test_end(&mut self) -> bool {
        match self.lexer.peek() {
            Some(Lexem {
                token: Token::Newline | Token::Delimiter(';'),
                ..
            }) => true,
            Some(Lexem {
                token: Token::Delimiter('}'),
                ..
            }) => true,
            Some(Lexem { token: _, .. }) => false,
            None => true,
        }
    }

    fn parse_primary(&mut self, struct_allowed: bool) -> Result<Expression, ParseError> {
        if let Some(lexem) = self.lexer.peek() {
            match &lexem.token {
                Token::Newline => {
                    self.next();
                    return self.parse_primary(struct_allowed);
                }
                Token::Numeric(..) => return Ok(self.parse_number()?),
                Token::Identifier(_) => {
                    let i = self.parse_identifier()?;

                    // Function call (e.g. foo(), bar())
                    if let Some(_) = self.test_token(Token::Delimiter('('), false) {
                        let args = self.parse_expression_list(Token::Delimiter(')'))?;
                        self.expect_token(Token::Delimiter(')'))?;
                        return Ok(Expression {
                            span: i.span.merge(self.current_span.unwrap()),
                            data: ExpressionData::FunctionCall {
                                function: Box::new(i),
                                arguments: args,
                            },
                        });
                    }

                    if let Some(Lexem {
                        token: Token::Delimiter('{'),
                        ..
                    }) = self.lexer.peek()
                    {
                        // Rust programmers be like:
                        if let true = struct_allowed {
                            // In case of not allowed, it will just not parse it at all
                            self.next();
                            let span = i.span.merge(self.current_span.unwrap());
                            let fields = self.parse_fields()?;
                            self.expect_token(Token::Delimiter('}'))?;
                            if let ExpressionData::Identifier { name } = i.data {
                                return Ok(Expression {
                                    span: span,
                                    data: ExpressionData::StructInit { name: name, fields },
                                });
                            } else {
                                unreachable!()
                            }
                        }
                    }
                    return Ok(i);
                }
                Token::Delimiter('(') => {
                    self.next();
                    let init_span = self.current_span.unwrap();
                    let expr = self.parse_expr(1, true)?;
                    if let Some(_) = self.test_token(Token::Delimiter(','), false) {
                        let tail = self.parse_expression_list(Token::Delimiter(')'))?;
                        self.expect_token(Token::Delimiter(')'))?;
                        let span = init_span.merge(self.current_span.unwrap());
                        let mut exprs = vec![expr];
                        exprs.extend(tail);
                        return Ok(Expression {
                            span: span,
                            data: ExpressionData::Tuple { exprs: exprs },
                        });
                    }
                    self.expect_token(Token::Delimiter(')'))?;
                    return Ok(expr);
                }
                Token::Delimiter('{') => {
                    self.next();
                    let init_span = self.current_span.unwrap();
                    let exprs = self.parse_expression_list(Token::Delimiter('}'))?;
                    self.expect_token(Token::Delimiter('}'))?;
                    let amount = exprs.len();
                    let span = init_span.merge(self.current_span.unwrap());
                    return Ok(Expression {
                        span,
                        data: ExpressionData::Array { exprs, amount },
                    });
                }
                Token::CharLiteral(c) => {
                    let len = c.chars().count();
                    if len != 1 {
                        return Err(ParseError {
                            span: Some(lexem.span),
                            case: ParseErrorCase::InvalidCharacterLiteral,
                        });
                    }
                    let c = c.chars().nth(0).unwrap();
                    self.next();
                    return Ok(Expression {
                        span: self.current_span.unwrap(),
                        data: ExpressionData::CharacterLiteral { value: c },
                    });
                }
                Token::StrLiteral(s) => {
                    let s = s.clone();
                    self.next();
                    return Ok(Expression {
                        span: self.current_span.unwrap(),
                        data: ExpressionData::StrLiteral { value: s },
                    });
                }
                Token::StringLiteral(s) => {
                    let s = s.clone();
                    self.next();
                    return Ok(Expression {
                        span: self.current_span.unwrap(),
                        data: ExpressionData::StrLiteral { value: s },
                        // FIXME: This will be StringLiteral when we use proper dynamic memory
                    });
                }
                Token::Keyword(Keyword::True) => {
                    self.next();
                    return Ok(Expression {
                        span: self.current_span.unwrap(),
                        data: ExpressionData::BooleanLiteral { value: true },
                    });
                }
                Token::Keyword(Keyword::False) => {
                    self.next();
                    return Ok(Expression {
                        span: self.current_span.unwrap(),
                        data: ExpressionData::BooleanLiteral { value: false },
                    });
                }
                token @ Token::Op(a, b) => {
                    let op = UnaryOperation::from_op(*a, b.as_ref().copied());
                    if let Some(unop) = op {
                        let prec = unop_precedence(token);
                        self.next();
                        return Ok(Expression {
                            span: self.current_span.unwrap(),
                            data: ExpressionData::UnaryOperation {
                                operator: unop,
                                operand: Box::new(self.parse_expr(prec, true)?),
                            },
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
                    });
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

    fn parse_expr(
        &mut self,
        limit: Precedence,
        struct_allowed: bool,
    ) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary(struct_allowed)?;
        while let Some(Lexem { token, .. }) = self.lexer.peek() {
            // Everything put before checking the precedence automatically means it's always the highest precedence.
            let next_limit = binop_precedence(token);
            if let Some(_) = self.test_token(Token::Delimiter('.'), true) {
                // Field access (e.g. instance.method(), foo.bar)
                let field = self.expect_identifier()?;
                left = Expression {
                    span: left.span.merge(self.current_span.unwrap()),
                    data: ExpressionData::FieldAccess {
                        origin: Box::new(left),
                        field: field,
                    },
                };
                continue;
            }

            if limit > next_limit {
                break;
            }

            if let Some(Lexem {
                token: Token::Op(a, b),
                ..
            }) = self.next()
            {
                let binop = BinaryOperation::from_op(a, b);
                let right = self.parse_expr(next_limit, true)?;
                left = Expression {
                    span: left.span.merge(self.current_span.unwrap()),
                    data: ExpressionData::BinaryOperation {
                        operator: binop.unwrap(),
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                };
            }
        }

        Ok(left)
    }

    fn parse_assignment(&mut self) -> Result<(String, Expression), ParseError> {
        let ident = self.expect_identifier()?;
        let _ = self.expect_token(Token::Op('=', None))?;
        let expr = self.parse_expr(1, true)?;
        Ok((ident, expr))
    }

    fn parse_let_stmt(&mut self) -> Result<Statement, ParseError> {
        let (ident, expr) = self.parse_assignment()?;
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
        let expr = self.parse_expr(1, true)?;
        Ok(Statement::ConstStatement(ConstStatement {
            binding: ident,
            assignment: expr,
            typing,
        }))
    }

    fn parse_var_stmt(&mut self) -> Result<Statement, ParseError> {
        let (ident, expr) = self.parse_assignment()?;
        Ok(Statement::VarStatement(VarStatement {
            binding: ident,
            assignment: expr,
        }))
    }

    fn parse_stmts(&mut self) -> Result<Block, ParseError> {
        let mut ast = vec![];
        while let Some(node) = self.parse_node(true)? {
            ast.push(node);
        }
        let p = Block { content: ast };
        Ok(p)
    }

    fn parse_block(&mut self, lazy: bool, inside_block: bool) -> Result<Block, ParseError> {
        let block: Block;
        if let Some(Lexem { span, .. }) = self.test_token(Token::Op('=', Some('>')), lazy) {
            if let Some(x) = self.parse_node(inside_block)? {
                block = Block { content: vec![x] }
            } else {
                return Err(ParseError {
                    span: Some(span),
                    case: ParseErrorCase::ExpectedStatement,
                });
            }
        } else {
            self.expect_token(Token::Delimiter('{'))?;
            block = self.parse_stmts()?;
            self.expect_token(Token::Delimiter('}'))?;
        }
        Ok(block)
    }

    fn parse_fn_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('('))?;
        let arguments = self.parse_typed_fields()?;
        self.expect_token(Token::Delimiter(')'))?;
        let mut typ = None;
        if let Some(_) = self.test_token(Token::Delimiter(':'), false) {
            typ = Some(self.parse_type()?);
        }
        self.expect_token(Token::Delimiter('{'))?;
        let block = self.parse_stmts()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::FnStatement(FnStatement {
            name,
            block,
            ret: typ,
            arguments,
        }))
    }

    fn parse_extern_fn_stmt(&mut self) -> Result<Statement, ParseError> {
        let x = self.next(); // consume fn token after extern
        let expected = Token::Keyword(Keyword::Fn);
        if let Some(lexem) = x {
            if lexem.token != expected {
                return Err(ParseError {
                    span: Some(lexem.span),
                    case: ParseErrorCase::UnexpectedToken {
                        got: format!("{:?}", lexem.token),
                        expected: format!("{:?}", expected),
                    },
                });
            }
        } else {
            return Err(ParseError {
                span: None,
                case: ParseErrorCase::UnexpectedToken {
                    got: EOF.to_string(),
                    expected: format!("{:?}", expected),
                },
            });
        }

        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('('))?;
        let (arguments, variadic) = self.parse_fn_decl_args()?;
        self.expect_token(Token::Delimiter(')'))?;
        let mut typ = None;
        if let Some(_) = self.test_token(Token::Delimiter(':'), false) {
            typ = Some(self.parse_type()?);
        }
        Ok(Statement::ExternFnStatement(ExternFnStatement {
            name,
            ret: typ,
            arguments,
            variadic,
        }))
    }

    fn parse_struct_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('{'))?;
        let fields = self.parse_typed_fields()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::StructStatement(StructStatement { name, fields }))
    }

    fn parse_enum_stmt(&mut self) -> Result<Statement, ParseError> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::Delimiter('{'))?;
        let variants = self.parse_enum_variants()?;
        self.expect_token(Token::Delimiter('}'))?;
        Ok(Statement::EnumStatement(EnumStatement { name, variants }))
    }

    fn parse_if_stmt(&mut self) -> Result<Statement, ParseError> {
        let expr = self.parse_expr(1, false)?;
        let block_true = self.parse_block(true, true)?;
        let mut block_false: Option<Block> = None;
        if let Some(_) = self.test_token(Token::Keyword(Keyword::Else), true) {
            if let Some(elseif) = self.test_token(Token::Keyword(Keyword::If), true) {
                let if_node = Node {
                    span: elseif.span,
                    stmt: self.parse_if_stmt()?,
                };
                block_false = Some(Block {
                    content: vec![if_node],
                });
            } else {
                self.expect_token(Token::Delimiter('{'))?;
                block_false = Some(self.parse_stmts()?);
                self.expect_token(Token::Delimiter('}'))?;
            }
        }
        Ok(Statement::IfStatement(IfStatement {
            condition: expr,
            block_false,
            block_true,
        }))
    }

    fn parse_while_stmt(&mut self) -> Result<Statement, ParseError> {
        let condition = self.parse_expr(1, false)?;
        let block = self.parse_block(true, true)?;
        Ok(Statement::WhileStatement(WhileStatement {
            condition,
            block,
        }))
    }

    fn parse_return_stmt(&mut self) -> Result<Statement, ParseError> {
        if self.test_end() {
            return Ok(Statement::ReturnStatement(ReturnStatement { expr: None }));
        }
        let expr = self.parse_expr(1, true)?;
        Ok(Statement::ReturnStatement(ReturnStatement {
            expr: Some(expr),
        }))
    }

    fn parse_stmt(&mut self, inside_block: bool) -> Result<Statement, ParseError> {
        if let Some(Lexem {
            token: Token::Keyword(kw),
            span,
        }) = self.next()
        {
            let result = match kw {
                Keyword::Struct => self.parse_struct_stmt(),
                Keyword::Fn => self.parse_fn_stmt(),
                Keyword::Extern => self.parse_extern_fn_stmt(),
                Keyword::Enum => self.parse_enum_stmt(),
                Keyword::Const => self.parse_const_stmt(),
                kw if !inside_block => Err(ParseError {
                    span: Some(span),
                    case: ParseErrorCase::UnexpectedToken {
                        got: format!("{kw:?}"),
                        expected: "valid statement".to_string(),
                    },
                }),
                Keyword::Var => self.parse_var_stmt(),
                Keyword::Let => self.parse_let_stmt(),
                Keyword::If => self.parse_if_stmt(),
                Keyword::While => self.parse_while_stmt(),
                Keyword::Return => self.parse_return_stmt(),
                Keyword::Else => Err(ParseError {
                    span: Some(span),
                    case: ParseErrorCase::UnexpectedToken {
                        got: "else keyword".to_string(),
                        expected: "valid statement".to_string(),
                    },
                }),
                Keyword::True => unreachable!("asked to parse true keyword in statement"),
                Keyword::False => unreachable!("asked to parse false keyword in statement"),
            };
            self.expect_end()?;
            return result;
        } else {
            unreachable!("asked to parse statement without keyword")
        }
    }
}

impl<'a> Parser<'a> {
    fn next(&mut self) -> Option<Lexem> {
        if let Some(l) = self.lexer.next() {
            self.current_span = Some(l.span);
            return Some(l);
        }
        return None;
    }

    fn parse_node(&mut self, inside_block: bool) -> Result<Option<Node>, ParseError> {
        let mut x = None;
        if let Some(lexem) = self.lexer.peek() {
            x = Some(match lexem.token {
                Token::Newline => {
                    self.next();
                    return self.parse_node(inside_block);
                }
                // Account for trailing } when terminating block
                Token::Delimiter('}') if inside_block => {
                    return Ok(None);
                }
                //                         I HATE THIS PART OF THE CODE, HELP ME PLEASE
                //                         Just to explain: This part checks if the keyword is not true and not false
                //                         so the parse_stmt function does not think that true or false is a statement keyword,
                //                         so if this condition fails, it falls through to the next case (_) which parses it as an
                //                         expression, the correct way to threat true and false.
                Token::Keyword(k) if k != Keyword::True && k != Keyword::False => Node {
                    span: lexem.span,
                    stmt: self.parse_stmt(inside_block)?,
                },
                _ => {
                    let span = lexem.span;
                    // Ensure that the next token is an token valid for an expression. Otherwise, stop parsing.
                    let expr = self.parse_expr(1, true)?;
                    let node = Node {
                        span,
                        stmt: Statement::ExpressionStatement(expr),
                    };
                    self.expect_end()?;
                    node
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
        let p = Program {
            filename: self.inputfile.filename.to_string(),
            nodes: ast,
        };
        Ok(p)
    }
}
