use oxido_ir::inputfile::InputFile;
use oxido_ir::word::Word;
use oxido_lexer::lexer::Lexer;

fn main() {
    println!("Oxido Lexer");

    let mut l = Lexer::new(InputFile {
        filename: Word("main.rs".to_string()),
        content: "fn main()\n { println!(\"Hello, world!\"); }".chars(),
    });

    while let Some(x) = l.next() {
        println!("{:?}:{:?}:{:?}", l.span.line, l.span.start, l.span.end);
        println!("{:?}", x);
    }
}
