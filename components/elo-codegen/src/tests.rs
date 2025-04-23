use elo_lexer::inputfile::InputFile;
use elo_lexer::lexer::Lexer;
use elo_parser::parser::Parser;
use elo_validation::validation::Validator;

#[test]
fn test_file() {
    use std::fs::read_to_string;
    let filename = "test.elo";
    let source_text = &read_to_string(filename).unwrap();
    let lx = Lexer::new(InputFile::new("test.rs", source_text));
    
    let prog = Parser::new(lx).parse().unwrap();
    let val = Validator::new(prog).validate().unwrap();
    let context = inkwell::context::Context::create();
    let module = context.create_module("elo");
    let mut r#gen = crate::generator::Generator {
        input: val,
        context: &context,
        module: module,
        builder: context.create_builder(),
    };
    r#gen.generate();
    println!("{}", r#gen.module.to_string());
}