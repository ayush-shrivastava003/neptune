use std::collections::HashMap;

use crate::{
    token::Token,
    interpreter::{Interpreter, Object},
    ast::Node,
    error::Error
};

#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<Token>,
    body: Node,
    pub name: String
}

impl Function {
    pub fn new(args: Vec<Token>, body: Node, name: String) -> Self {
        Self { args, body, name }
    }

    pub fn call(&mut self, interpreter: &mut Interpreter, actual_args: Vec<Object>) -> Result<Object, Error> {
        // println!("\x1b[32mMake (function.rs).\x1b[0m");
        // println!("after len: {}", interpreter.environments.len());
        interpreter.environments.push(HashMap::new());
        // println!("{:?}", interpreter.environments);
        
        for (expected, actual) in self.args.iter().zip(actual_args.iter()) {
            let enviro = interpreter.environments.last_mut().unwrap();
            enviro.insert(expected.value.clone(), actual.clone());
        }

        match interpreter.traverse(&self.body) {
            Err(Error::Runtime(v)) => return Err(Error::Runtime(v)),
            Err(Error::Return(v)) => {
                // println!("\x1b[31mPurge (block).\x1b[0m");
                interpreter.environments.pop();
                // println!("after len: {}", interpreter.environments.len());
                return Ok(v)
            },
            _ => {
                // println!("\x1b[31mPurge (block).\x1b[0m");
                interpreter.environments.pop();
                // println!("after len: {}", interpreter.environments.len());
                return Ok(Object::None)
            }
        };
    }
}