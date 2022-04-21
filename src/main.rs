// For simple, more readable code (i hope), see ayush-shrivastava003/language.
// This is my more complicated implementation written in Rust instead of Python.
use std::fs;
mod lexer;

fn main() {
    match fs::read_to_string("test.nt") {
        Ok(contents) => {
            println!("{}", contents);
            let mut lexer = lexer::Lexer::new(contents);
            match lexer.tokenize() {
                Ok(tokens) => println!("{:?}", tokens),
                Err(msg) => println!("An error occured: {}", msg)
            }
        },
        Err(msg) => println!("A problem occured with opening the file: {}", msg)
    }
    // println!("{}", '\n'.is_whitespace());
}
