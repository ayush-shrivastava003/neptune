use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    BinaryOperator { //
        id: usize,
        left: Box<Node>,
        operator: Token,
        right: Box<Node>
    },
    Literal { //
        id: usize,
        value: Literal
    },
    UnaryOperator { //
        id: usize,
        operator: Token,
        child: Box<Node>
    },
    Logical { //
        id: usize,
        left: Box<Node>,
        operator: Token,
        right: Box<Node>
    },
    Declare { //
        id: usize,
        name: Token,
        value: Box<Node>
    },
    Assign { //
        id: usize,
        name: Token,
        value: Box<Node>
    },
    If { //
        id: usize,
        condition: Box<Node>,
        body: Box<Node>,
        else_block: Option<Box<Node>>
    },
    While {
        id: usize,
        condition: Box<Node>,
        body: Box<Node>
    },
    Variable {
        id: usize,
        name: Token
    },
    Block(Vec<Node>),
    Print(Box<Node>)
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Bool(bool),
    String(String),
    None
}