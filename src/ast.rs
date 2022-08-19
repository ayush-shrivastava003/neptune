use core::fmt;

use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
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
    DeclareFn {
        id: usize,
        name: Token,
        args: Vec<Token>,
        body: Box<Node>
    },
    FnCall {
        id: usize,
        name: Box<Node>,
        args: Vec<Node>
    },
    Return {
        id: usize,
        value: Box<Node>
    },
    Block(Vec<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::BinaryOperator {left, operator, right, .. } => write!(f, "{} {:?} {}", left, operator.value, right),
            Node::Literal { value, ..} => write!(f, "{}", value),
            Node::UnaryOperator { operator, child, .. } => write!(f, "{:?}{}", operator.value, child),
            Node::Logical { left, operator, right, .. } => write!(f, "{} {:?} {}", left, operator.value, right),
            Node::Declare { name, value, .. } => write!(f, "let {} = {}", name.value, value),
            Node::Assign { name, value, .. } => write!(f, "{:?} = {}", name, value),
            Node::If { condition, .. } => write!(f, "if ({})", condition),
            Node::While { condition, .. } => write!(f, "while ({})", condition),
            Node::Variable { name, .. } => write!(f, "{}", name.value),
            Node::DeclareFn { name, .. } => write!(f, "fn {}", name.value),
            Node::FnCall { name, args, ..} => write!(f, "{}({:?})", name, args),
            Node::Return { value, .. } => write!(f, "return {}", value),
            Node::Block(v) => write!(f, "{:?}", v)
        }   
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Bool(bool),
    String(String),
    None
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::None => write!(f, "none"),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => {
                let number = n.to_string();
                if number.ends_with(".0") {
                    write!(f, "{}", number.strip_suffix(".0").unwrap())
                } else {
                    write!(f, "{}", number)
                }
                },
            Literal::Bool(b) => write!(f, "{}", b),
            // Literal::FunctionCall(_) => write!(f, "<fn>")
        }
    }
}