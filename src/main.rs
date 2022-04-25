// For simple, more readable code (i hope), see ayush-shrivastava003/language.
// This is my more complicated implementation written in Rust instead of Python.
use std::fs;
extern crate neptune as this;
use this::lexer::Lexer;
use this::parser::Parser;

fn main() {
    match fs::read_to_string("test.nt") {
        Ok(contents) => {
            println!("{}", contents);
            let mut lexer = Lexer::new(contents);
            match lexer.tokenize() {
                Ok(tokens) => {println!("{:?}", Parser::new(&tokens).get_expression())}
                Err(msg) => {println!("{}", msg)},
            };
        },
        Err(msg) => println!("A problem occured with opening the file: {}", msg)
    }
}
