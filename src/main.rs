pub mod enums;
pub mod interpreter;
pub mod lexer;
pub mod parser;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: rustic <source_file>");
        return;
    }

    let source_file = &args[1];
    let path = Path::new(source_file);
    let source_code = std::fs::read_to_string(path).expect("Error reading source file");

    // run the program
    run(&source_code);
}

fn run(source_code: &str) {
    let mut lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(&mut lexer);
    let mut interpreter = Interpreter::new(&mut parser);

    match interpreter.interpret() {
        Ok(_) => println!("Program executed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
