use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod runtime;

use lexer::Lexer;
use parser::Parser;
use runtime::Env;

fn main() {
    // Lire argument CLI
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: meta <file.ta>");
        return;
    }

    let filename = &args[1];

    // Lire fichier
    let code = fs::read_to_string(filename)
        .expect("Impossible de lire le fichier");

    // Lexer
    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    // Parser
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // Runtime
    let mut env = Env::new();
    let result = env.eval_block(&ast);

    println!("Result: {:?}", result);
}
