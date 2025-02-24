use crate::parser::Parser;
use elo_lexer::inputfile::InputFile;
use elo_lexer::lexer::Lexer;

#[test]
fn test_binop() {
    let source_text = "(a + b * c) / 2";
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

#[test]
fn test_unop() {
    let source_text = "-a + b";
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

#[test]
fn test_let_stmt() {
    let source_text = "let x = 10";
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

#[test]
fn test_var_stmt() {
    let source_text = "var x = 69";
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

#[test]
fn test_const_stmt() {
    let source_text = "const PI: float = 3.1415";
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

#[test]
fn test_fn_stmt() {
    let source_text = "fn foo(): int {\nlet x = 10;}";
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

#[test]
fn test_struct_stmt() {
    let source_text = "struct A { a: int }";
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

#[test]
fn test_enum_stmt() {
    let source_text = "enum Week { Sun, Mon, Tue, Wed, Thu, Fri, Sat }";
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

#[test]
fn test_struct_init() {
    let source_text = "MyStruct { a: 10 + 1*(a+b), b: 10 };";
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
