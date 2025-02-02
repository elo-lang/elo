use crate::lexem::Lexem;

#[test]
fn test_floats() {
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = "0.1 1.1";
    let lx = Lexer::new(InputFile::new("test", source_text.chars()));

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
    let lx = Lexer::new(InputFile::new("test", source_text.chars()));

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
    let lx = Lexer::new(InputFile::new("test", source_text.chars()));

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
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = "1.abc";
    let lx = Lexer::new(InputFile::new("test", source_text.chars()));

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
    use crate::inputfile::InputFile;
    use crate::lexer::Lexer;

    let source_text = "abc.1";
    let lx = Lexer::new(InputFile::new("test", source_text.chars()));

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
