use crate::parser::Parser;
use oxido_lexer::inputfile::InputFile;
use oxido_lexer::lexer::Lexer;

#[test]
fn test_let_stmt() {
    let source_text = "0.143535345435 1.1";
    let lx = Lexer::new(InputFile::new("test.rs", source_text.chars()));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:?}", ast);
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
            println!("  {:?}", e.case);
        }
    }
}
