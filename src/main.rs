// For simple, more readable code (i hope), see ayush-shrivastava003/language.
// This is my more complicated implementation written in Rust instead of Python.
use std::fs;
extern crate neptune as this;
use this::{lexer::Lexer, parser::Parser, interpreter::Interpreter, semantic_analyzer::SemanticAnalyzer};

fn main() {
    match fs::read_to_string("test.nt") {
        Ok(contents) => {
            println!("{}", contents);
            let mut lexer = Lexer::new(contents);
            match lexer.tokenize() {
                Ok(tokens) => {
                    match Parser::new(&tokens).parse() {
                        Ok(v) => {
                            let mut interpreter = Interpreter::new();
                            let mut semantic_analyzer = SemanticAnalyzer::new(&mut interpreter);
                            semantic_analyzer.resolve(&v);
                            match interpreter.run(v) {
                                Ok(v) => println!("Final value: {:?}", v),
                                Err(msg) => println!("Runtime error: {}", msg)
                            }
                        },
                        Err(msg) => return println!("Syntax error: {}", msg)
                    }
                }
                Err(msg) => return println!("{}", msg),
            };
        },
        Err(msg) => println!("A problem occured with opening the file: {}", msg)
    }
}
