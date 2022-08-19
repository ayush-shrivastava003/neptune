use crate::interpreter::Object;

pub enum Error {
    Runtime(String),
    Syntax(String),
    Return(Object)
}