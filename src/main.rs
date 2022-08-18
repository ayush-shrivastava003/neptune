extern crate neptune;
use neptune::{
    lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    interpreter::*,
    error::Error
};
use std::{fs::read_to_string, env::args};

fn run(file: String) -> Result<Object, Error> {
    let tokens = Lexer::new(&file).tokenize()?;
    let ast = Parser::new(&tokens).parse()?;
    let mut interpreter = Interpreter::new();
    Resolver::new(&mut interpreter).resolve_block(&ast)?;
    interpreter.run(&ast)
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        return println!("CLI is not yet implemented. Please supply a file to run.")
    }
    let file = read_to_string(args[1].as_str()).expect("Error reading the file");

    match run(file) {
        Err(Error::Runtime(v)) => println!("\x1b[31mRuntime Error: {}\x1b[0m", v),
        Err(Error::Syntax(v)) => println!("\x1b[31mSyntax error: {}\x1b[0m", v),
        _ => return
    };
}
