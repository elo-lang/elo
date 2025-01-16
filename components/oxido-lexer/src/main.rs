use oxido_ir::inputfile::InputFile;
use oxido_ir::word::Word;
use oxido_lexer::lexer::Lexer;

fn main() {
    println!("Oxido Lexer");
    let string = "println(\"Hello\")";
    let mut l = Lexer::new(InputFile {
        filename: Word("main.rs".to_string()),
        content:string.chars().peekable(),
    });

    while let Some(x) = l.next() {
        let token = x;
        let start = l.span.start;
        let end = l.span.end;
        let line = l.span.line;
        let content = &string[start..end].to_string();

        println!("{token:?} {line:?}:{start:?}:{end:?} {content:?}");
    }
}
