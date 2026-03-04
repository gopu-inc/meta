mod lexer;
mod parser;
mod ast;
mod runtime;

use lexer::Lexer;
use parser::Parser;
use runtime::Env;

fn main() {
    let code = r#"
        let x = 10;
        let y = 20;
        fn add(a, b) { return a + b; }
        add(x, y);
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut env = Env::new();
    let result = env.eval_block(&ast);

    println!("Result: {:?}", result);
}


