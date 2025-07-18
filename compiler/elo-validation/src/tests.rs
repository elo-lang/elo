#[test]
fn test_file() {
    use crate::validation::Validator;
    use elo_lexer::inputfile::InputFile;
    use elo_lexer::lexer::Lexer;
    use elo_parser::parser::Parser;

    use std::fs::read_to_string;
    let filename = "test.elo";
    let source_text = &read_to_string(filename).unwrap();
    let lx = Lexer::new(InputFile::new("test.rs", source_text));

    let prog = Parser::new(lx).parse().unwrap();
    let val = Validator::new(prog);
    match val.validate() {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}
