use std::{collections::HashMap, fmt::Display};

use crate::{
    token::Token,
    interpreter::{Interpreter, Object},
    ast::Node,
    error::Error
};

#[derive(Clone)]
pub enum Function {
    UserDefined {
        args: Vec<Token>,
        body: Node,
        name: Token
    },
    Native {
        arg_len: usize,
        body: Box<fn(&Vec<Object>) -> Result<Object, Error>>,
        name: String
    }
}

impl Function {
    pub fn call(&mut self, interpreter: &mut Interpreter, actual_args: Vec<Object>) -> Result<Object, Error> {
        match self {
            Function::UserDefined { args, body, .. } => {
                interpreter.environments.push(HashMap::new());
        
                for (expected, actual) in args.iter().zip(actual_args.iter()) {
                    let enviro = interpreter.environments.last_mut().unwrap();
                    enviro.insert(expected.value.clone(), actual.clone());
                }
        
                match interpreter.traverse(&body) {
                    Err(Error::Runtime(v)) => return Err(Error::Runtime(v)),
                    Err(Error::Return(v)) => {
                        interpreter.environments.pop();
                        return Ok(v)
                    },
                    _ => {
                        interpreter.environments.pop();
                        return Ok(Object::None)
                    }
                };
            },

            Function::Native { body, .. } => {
                Ok(body(&actual_args)?)
            }
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::UserDefined { name, .. } => {
                write!(f, "<fn '{}' at [{}:{}]>", name.value, name.line, name.column)
            },
            Function::Native { name, .. } => {
                write!(f, "<native fn '{}'>", name)
            }
        }
    }
}