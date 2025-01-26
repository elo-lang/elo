
#[test]
fn test_floats() {
    use crate::lexer::Lexer;
    use crate::inputfile::InputFile;
    
    let source_text = ".1 0.1";
    let mut lx = Lexer::new(InputFile::new(
        "test.rs",
        source_text.chars(),
    ));

    while let Some(token) = lx.next() {
        println!("{}:{}:{} \"{}\"", lx.span.line, lx.span.start, lx.span.end, &source_text[lx.span.start..lx.span.end]);
        println!("{:?}", token);
    }
}

#[test]
fn test_strings() {
    use crate::lexer::Lexer;
    use crate::inputfile::InputFile;
    
    let source_text = "\"hello\"";
    let mut lx = Lexer::new(InputFile::new(
        "test.rs",
        source_text.chars(),
    ));

    while let Some(token) = lx.next() {
        println!("{}:{}:{} \"{}\"", lx.span.line, lx.span.start, lx.span.end, &source_text[lx.span.start..lx.span.end]);
        println!("{:?}", token);
    }
}

#[test]
fn test_comments() {
    use crate::lexer::Lexer;
    use crate::inputfile::InputFile;
    
    let source_text = "# This is a comment\n";
    let mut lx = Lexer::new(InputFile::new(
        "test.rs",
        source_text.chars(),
    ));

    while let Some(token) = lx.next() {
        println!("{}:{}:{} \"{}\"", lx.span.line, lx.span.start, lx.span.end, &source_text[lx.span.start..lx.span.end]);
        println!("{:?}", token);
    }
}