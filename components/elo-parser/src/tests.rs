use crate::parser::Parser;
use elo_lexer::inputfile::InputFile;
use elo_lexer::lexer::Lexer;

#[test]
fn test_binop() {
    let source_text = "(a + b * c) / 2";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}

#[test]
fn test_unop() {
    let source_text = "-a + b";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}

#[test]
fn test_let_stmt() {
    let source_text = "let x = 10";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}

#[test]
fn test_var_stmt() {
    let source_text = "var x = 69";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}

#[test]
fn test_const_stmt() {
    let source_text = "const PI: float = 3.1415";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}

#[test]
fn test_fn_stmt() {
    let source_text = "fn foo() { let x = 10\n let a = 10; }";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("error:");
            if let Some(span) = e.span {
                println!(
                    "  at {:?} line {} from {} to {}",
                    parser.inputfile.filename, span.line, span.start, span.end
                );
                println!(
                    "| {}",
                    &parser.inputfile.content.collect::<String>().as_str()[span.start..span.end]
                );
            }
            println!("  {:#?}", e.case);
        }
    }
}