use crate::interpreter::Object;

#[derive(Debug)]
pub enum Error {
    Runtime(String),
    Syntax(String),
    Return(Object)
}