use oxido_ir::filespan::FileSpan;
use oxido_ir::inputfile::InputFile;
use oxido_ir::token::Token;
use oxido_ir::word::Word;

pub struct Lexer<'a> {
    pub input_file: InputFile<'a>,
    pub span: FileSpan<'a>,
}

pub fn is_token_whitespace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}

macro_rules! delimiter {
    () => {
        '(' | ')' | '[' | ']' | '{' | '}' | '.' | ';' | ':'
    };
}

macro_rules! op {
    () => {
        '+' | '-' | '/' | '*' | '%' | '!' | '>' | '<' | '&' | '|' | '^' | '~' | '.' | ':' | ';' | '='
    };
}

impl<'a> Lexer<'a> {
    pub fn new(input_file: InputFile) -> Lexer {
        Lexer {
            input_file: input_file.clone(),
            span: FileSpan::empty(input_file),
        }
    }

    fn advance_span(&mut self, advance_length: usize) {
        self.span.start = self.span.end;
        self.span.end += advance_length;
    }

    fn advance_line(&mut self) {
        self.span.line += 1;
        self.span.start = 0;
        self.span.end = 0;
    }

    pub fn consume_while(&mut self, start: Option<&char>, matches: fn(char) -> bool) -> String {
        let mut buffer = String::new();
        if let Some(start) = start {
            buffer.push(*start);
        }
        while let Some(&c) = self.input_file.content.peek() {
            if !matches(c) {
                break;
            }
            buffer.push(c);
            self.input_file.content.next();
        }
        buffer
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(c) = self.input_file.content.next() {
            match c {
                '#' => {
                    // Ignore comments
                    let _ = self.consume_while(Some(&c), |c| c != '\n');
                    self.advance_line();
                    return self.next();
                }
                '\n' => {
                    self.advance_line();
                    return Some(Token::Newline);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let s = self.consume_while(Some(&c), |c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'));
                    self.advance_span(s.len());
                    return Some(Token::Alphabetic(Word(s)));
                }
                a if is_token_whitespace(a) => {
                    self.advance_span(1);
                    return self.next();
                }
                '0'..='9' => {
                    let s = self.consume_while(Some(&c), |c| matches!(c, '0'..='9' | '_') || !matches!(c, delimiter!()));
                    self.advance_span(s.len());
                    return Some(Token::Numeric(Word(s)));
                }
                ',' => {
                    self.advance_span(1);
                    return Some(Token::Comma);
                }
                op!() => {
                    let next = self.input_file.content.peek();
                    let op = match next {
                        Some(&b) if matches!(b, op!()) => {
                            self.advance_span(1);
                            Some(b)
                        }
                        _ => None,
                    };
                    self.advance_span(1);
                    return Some(Token::Op(c, op));
                }
                delimiter!() => {
                    self.advance_span(1);
                    return Some(Token::Delimiter(c));
                }
                '"' => {
                    let s = self.consume_while(None, |c| c != '"');
                    if self.input_file.content.peek() != Some(&'"') {
                        panic!("Unterminated string literal");
                    }
                    self.input_file.content.next(); // Compensate for the last "
                    self.advance_span(s.len());
                    self.span.end += 2; // Compensate span to get the last "
                    return Some(Token::StringLiteral(s));
                }
                c => {
                    self.advance_span(1);
                    return Some(Token::Unknown(c))
                }
            }
        }
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_lex_print() {
        // TODO: Bug com a source_text abaixo, os spans estão desconsiderando os comentários
        // e começando a contar desconsiderando os caracteres de comentários
        //let source_text = "#foda-se isso aqi\nfn() main 4.98";
        // OUTPUT:
        //Newline 3:0:0 ""
        //Alphabetic(Word("fn")) 3:0:2 "#f"
        //Delimiter('(') 3:2:3 "o"
        //Delimiter(')') 3:3:4 "d"
        //Alphabetic(Word("main")) 3:5:9 "-se "        
        //Numeric(Word("4")) 3:10:11 "s"
        //Op('.', None) 3:11:12 "s"
        //Numeric(Word("98")) 3:12:14 "o "

        // PS: tanto consume_until quanto consume_while geram o mesmo resultado
        // dei preferência ao consume_while por ser mais intuitivo

        let source_text = "#foda-se isso aqi\nfn() main 4.98";
        let mut lx = Lexer::new(InputFile {
            filename: "main.rs",
            content: source_text.chars().peekable(),
        });

        while let Some(tk) = lx.next() {
            let token = tk;
            let start = lx.span.start;
            let end = lx.span.end;
            let line = lx.span.line;
            let content = &source_text[start..end].to_string();

            println!("{:?} {:?}:{:?}:{:?} {:?}", token, line, start, end, content);
        }
    }

    #[test]
    fn test_lex1() {
        let source_text = "fn() main";
        let lx = Lexer::new(InputFile {
            filename: "main.rs",
            content: source_text.chars().peekable(),
        });

        let tokens: Vec<Token> = lx.collect();
        assert_eq!(tokens, vec![
            Token::Alphabetic(Word("fn".to_string())),
            Token::Delimiter('('),
            Token::Delimiter(')'),
            Token::Alphabetic(Word("main".to_string())),
        ]);   
    }

    #[test]
    fn test_lex_string_literal() {
        let source_text = "fn() \"Hello, World!\"";
        let lx = Lexer::new(InputFile {
            filename: "main.rs",
            content: source_text.chars().peekable(),
        });

        let tokens: Vec<Token> = lx.collect();
        assert_eq!(tokens, vec![
            Token::Alphabetic(Word("fn".to_string())),
            Token::Delimiter('('),
            Token::Delimiter(')'),
            Token::StringLiteral("Hello, World!".to_string()),
        ]);
    }
}