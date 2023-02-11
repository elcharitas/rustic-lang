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

    // help command
    if (args.len() != 2) || (args[1] == "-h") || (args[1] == "--help") {
        println!("Usage: rustic <source_file>");
        println!("Options:");
        println!("  -h, --help\t\tShow this help message");
        println!("  -r, --repl\t\tStart the Rustic REPL");
        return;
    }

    // repl command
    if (args[1] == "-r") || (args[1] == "--repl") {
        println!("Rustic REPL");
        println!("Type 'exit' to exit or enter Ctrl+C\n");
        print!(">>> ");
        let mut input = String::new();
        loop {
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error reading input");
            if input.trim() == "exit" {
                break;
            }
            run(&format!("{}\n", input));
            print!(">>> ");
        }
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
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}
