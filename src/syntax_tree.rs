use crate::token::Token;

#[derive(Debug)]
pub enum Node {
    BinaryOperator {
        left: Box<Node>,
        operator: Token,
        right: Box<Node>        
    },
    Literal(Literal),
    UnaryOperator {
        operator: Token,
        child: Box<Node>
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Bool(bool),
    String(String)
}