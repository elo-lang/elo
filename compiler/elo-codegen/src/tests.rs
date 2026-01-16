use elo_lexer::inputfile::InputFile;
use elo_lexer::lexer::Lexer;
use elo_parser::parser::Parser;
use elo_validation::validation::Validator;

#[test]
fn test_file() {
    use std::fs::read_to_string;
    let filename = "test.elo";
    let source_text = &read_to_string(filename).unwrap();
    let lx = Lexer::new(InputFile::new(filename, source_text));
    let prog = Parser::new(lx).parse().unwrap();
    let val = Validator::new().go(prog.nodes).unwrap();
    let mut r#gen = crate::generator::Generator::new(val);
    r#gen.go();
    println!("{}{}", r#gen.head, r#gen.body);
}
