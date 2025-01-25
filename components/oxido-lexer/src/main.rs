use oxido_lexer::inputfile::InputFile;
use oxido_lexer::token::Token;
use oxido_lexer::lexer::Lexer;

fn main() {
    // Tive que remover os testes, perdão igor :(
    // Não sabia como rodar os testes de um projeto de dentro de outro
    let source_text = "let x = \"Hello\";"; // Apparently it works
    let lx = Lexer::new(InputFile {
        filename: "main.rs",
        content: source_text.chars().peekable(),
    });

    let tokens: Vec<Token> = lx.collect();
    println!("{:?}", tokens);
}
