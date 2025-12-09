use crate::inputfile::InputFile;
use crate::lexer::Lexer;
use crate::span::Span;
use crate::token::Token;

fn get_span_interval(span: Span, source: &str) -> String {
    for (i, line) in source.lines().enumerate() {
        if i + 1 == span.line {
            return String::from(&line[span.start..span.end]);
        }
    }
    String::new()
}

#[test]
fn test_file() {
    let filename = "test.elo";
    let source_text = std::fs::read_to_string(filename).unwrap();
    println!("{}", source_text);
    let lx = Lexer::new(InputFile::new(filename, &source_text));
    for l in lx {
        println!(
            "{}:{}:{} ' {} ': {:?}",
            filename,
            l.span.line,
            l.span.start,
            get_span_interval(l.span, &source_text),
            l.token,
        )
    }
}

#[test]
fn test_integers() {
    let source_text = "69 420 1_000_000 0b01101 0xFf 0o07";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert_eq!(
        xs,
        vec![
            Token::Numeric(String::from("69"), 10),
            Token::Numeric(String::from("420"), 10),
            Token::Numeric(String::from("1_000_000"), 10),
            Token::Numeric(String::from("01101"), 2),
            Token::Numeric(String::from("Ff"), 16),
            Token::Numeric(String::from("07"), 8),
        ]
    );
}

#[test]
fn test_floats() {
    let source_text = "6.9 4.20";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert_eq!(
        xs,
        vec![
            Token::Numeric(String::from("6"), 10),
            Token::Delimiter('.'),
            Token::Numeric(String::from("9"), 10),
            Token::Numeric(String::from("4"), 10),
            Token::Delimiter('.'),
            Token::Numeric(String::from("20"), 10),
        ]
    );
}

#[test]
fn test_strings() {
    let source_text = "\"hello world\\n\" 'hello world\\n'";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert_eq!(
        xs,
        vec![
            Token::StringLiteral(String::from("hello world\\n")),
            Token::StrLiteral(String::from("hello world\\n")),
        ]
    );
}

#[test]
fn test_comments() {
    let source_text = "// This is a comment\n// Hello World\n";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert!(xs.is_empty());
}

#[test]
fn test_dot() {
    let source_text = "1.abc abc.1";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert_eq!(
        xs,
        vec![
            Token::Numeric(String::from("1"), 10),
            Token::Delimiter('.'),
            Token::Identifier(String::from("abc")),
            Token::Identifier(String::from("abc")),
            Token::Delimiter('.'),
            Token::Numeric(String::from("1"), 10),
        ]
    );
}

#[test]
fn test_whitespaces() {
    let source_text = "\t\n\x0C\x0B69 \x0C 420 foo \x0B bar     \t\n\x0C\x0B";
    let lx = Lexer::new(InputFile::new("test", source_text));
    let xs = lx.map(|lx| lx.token).collect::<Vec<Token>>();
    assert_eq!(
        xs,
        vec![
            Token::Newline,
            Token::Numeric(String::from("69"), 10),
            Token::Numeric(String::from("420"), 10),
            Token::Identifier(String::from("foo")),
            Token::Identifier(String::from("bar")),
            Token::Newline,
        ]
    );
}
