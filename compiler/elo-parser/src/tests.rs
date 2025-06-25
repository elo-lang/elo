#[test]
fn test_file() {
    use crate::parser::Parser;
    use elo_lexer::inputfile::InputFile;
    use elo_lexer::lexer::Lexer;

    use std::fs::read_to_string;
    let filename = "test.elo";
    let source_text = &read_to_string(filename).unwrap();
    let lx = Lexer::new(InputFile::new("test.rs", source_text));

    let mut parser = Parser::new(lx);
    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}
