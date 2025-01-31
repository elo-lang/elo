#[test]
fn test_floats() {
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = ".1 0.1";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

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
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = "\"hello\"";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

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
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = "# This is a comment\n";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

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
