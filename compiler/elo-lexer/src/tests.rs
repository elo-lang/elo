use crate::inputfile::InputFile;
use crate::lexer::Lexer;

#[test]
fn test_integers() {
    let source_text = "10 14 0x16 17";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_variadic() {
    let source_text = "... . . . .. ..";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_integers_invalid_suffix_after_0() {
    let source_text = "0a254 0c778";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_numbers_with_underline() {
    let source_text = "6_000 _420";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_binary_numbers() {
    let source_text = "0b11001 0b14";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_octal_numbers() {
    let source_text = "0o47523 0o78";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_hexadecimal_numbers() {
    let source_text = "0xafcd4 0x6";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_floats() {
    let source_text = "6.9 4.20";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_strings() {
    let source_text = "\"hello world\"";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_comments() {
    let source_text = "# This is a comment\n";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_period1() {
    let source_text = "1.abc";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}

#[test]
fn test_period2() {
    let source_text = "abc.1";
    let lx = Lexer::new(InputFile::new("test", source_text));

    for lexem in lx {
        let token = lexem.token;
        let span = lexem.span;
        println!(
            "{}:{}:{} \"{}\"",
            span.line,
            span.start,
            span.end,
            &source_text[span.start..span.end]
        );
        println!("{:?}", token);
    }
}
