extern crate neptune;
use neptune::{
    lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    interpreter::*,
    error::Error
};
use std::{
    fs::{read_to_string},
    env::args,
};
use rustyline::{
    Editor,
    // Result,
    error::ReadlineError
};

const VERSION: &str = "1.0.0";

fn handle_errors(result: Result<Object, Error>) {
    match result {
        Err(Error::Runtime(v)) => println!("\x1b[31mRuntime Error: {}\x1b[0m", v),
        Err(Error::Syntax(v)) => println!("\x1b[31mSyntax error: {}\x1b[0m", v),
        _ => return
    }
}

fn run(file: String) -> Result<Object, Error> {
    let tokens = Lexer::new(&file).tokenize()?;
    let ast = Parser::new(&tokens).parse()?;
    let mut interpreter = Interpreter::new();
    Resolver::new(&mut interpreter).resolve_block(&ast)?;
    interpreter.run(&ast)
}

fn run_prompt() {
    println!("\x1b[32mShell version {}", VERSION);
    println!("Supported operators: +, -, *, /, ()");
    println!("Type \"exit\" or hit ^C to exit.\x1b[0m");
    
    let mut editor = Editor::<()>::new().expect("Something went wrong with initializing the prompt.");

    if editor.load_history("npt-hisory.txt").is_err() {
        if std::fs::File::create("npt-history.txt").is_err() {
            println!("An error occured with making a history file");
            return
        }
    }

    loop {
        let line = editor.readline("> ");
        match line {
            Ok(content) => {
                editor.add_history_entry(content.as_str());
                match content {
                    content if content == "exit".to_string() => break,
                    content if content.is_empty() => continue,
                    content if (content.chars().collect::<Vec<char>>()[0]).is_whitespace() => continue,
                    _ => handle_errors(run(content))
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(e) => { println!("Unexpected error: {}", e); break }
        }
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        // return println!("CLI is not yet implemented. Please supply a file to run.")
        run_prompt();
    } else {
        let file = read_to_string(args[1].as_str()).expect("Error reading the file");
        handle_errors(run(file))
    }

}
