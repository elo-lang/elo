use oxido_ir::inputfile::InputFile;
use oxido_ir::word::Word;
use oxido_lexer::lexer::Lexer;

fn main() {
    println!("Oxido Lexer");

    let l = Lexer::new(InputFile {
        filename: Word("main.rs".to_string()),
        content: "fn main() { println!(\"Hello, world!\"); }".to_string(),
    });
    
    for token in l {
        println!("{:?}", token);
    }
}
