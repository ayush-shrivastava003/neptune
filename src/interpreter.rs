use std::collections::HashMap;

use crate::environment::Environment;
use crate::syntax_tree::{Node, Literal};
use crate::token::*;

#[derive(Debug, Clone)]
pub enum Object { // wrapper for multiple data types
    Number(f64),
    Bool(bool),
    String(String),
    None
}
pub struct Interpreter {
    environments: Vec<Environment>,
    pub depths: HashMap<usize, usize>,
    enviro_idx: usize
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environments = vec![];
        environments.push(Environment::new());
        Self {
            environments,
            depths: HashMap::<usize, usize>::new(),
            enviro_idx: 0
        }
    }

    pub fn run(&mut self, expression: Node) -> Result<Object, String> {
        self.traverse(expression)
    }

    pub fn resolve(&mut self, expr: usize, depth: usize) {
        self.depths.insert(expr, depth);
    }

    fn lookup(&mut self, name: String, depth: usize) -> Result<Object, String> {
        let distance = self.depths.get(&depth).unwrap();
        
        if distance >= &self.environments.len() {
            return Err(format!("Variable '{}' is not defined in this scope", name))
        }

        let obj = self.environments[distance.clone()].get(name);
        Ok(obj.unwrap().clone())
    }
    
    fn is_truthy(&self, expr: &Object) -> bool {
        return match expr {
          &Object::Bool(v) => v,
          &Object::None => false,
          _ => true
        }
    }

    fn current_environment(&mut self) -> &mut Environment {
        &mut self.environments[self.enviro_idx]
    }

    fn ancestor(&mut self, distance: usize) -> &mut Environment {
        &mut self.environments[distance]
    }

    pub fn traverse(&mut self, node: Node) -> Result<Object, String> {
        match node {
            Node::BinaryOperator {left: l, operator: o, right: r, ..} => Ok(self.binary_operator(l, o, r)?),
            Node::UnaryOperator { operator: o, child: c, .. } => Ok(self.unary(o, c)?),
            Node::Logical {left: l, operator: o, right: r, ..} => Ok(self.logical(l, o, r)?),
            Node::Literal { value: lit, .. } => Ok(self.literal(lit)),
            Node::Block(nodes) => {
                for n in nodes {
                    self.traverse(n)?;
                }
                Ok(Object::None)
            },
            Node::Declare {name, value, ..} => Ok(self.declare(name, value)?),
            Node::Assign {id, name, value} => Ok(self.assign(id, name, value)?),
            Node::Print(expr) => Ok(self.print(expr)?),
            Node::Variable { id, name } => Ok(self.lookup(name.value, id)?),
            Node::If { condition, body, else_block, ..} => Ok(self.if_block(condition, body, else_block)?),
            _=> {println!("Not implemented: {:?}", node); return Ok(Object::None)}
        }
    }

    fn if_block(&mut self, condition: Box<Node>, body: Box<Node>, else_block: Option<Box<Node>>) -> Result<Object, String> {
        let c = self.traverse(*condition)?;
        if self.is_truthy(&c) {
            let b = self.traverse(*body)?;
            return Ok(b)
        } else if else_block != None {
            return Ok(self.traverse(*else_block.unwrap())?)
        } else {
            return Ok(Object::None)
        }
    }

    fn print(&mut self, expr: Box<Node>) -> Result<Object, String> {
        let value = match self.traverse(*expr)? {
            Object::None => "none".to_string(),
            Object::String(s) => s,
            Object::Number(n) => {
                let number = n.to_string();
                if number.ends_with(".0") {
                    number.strip_suffix(".0").unwrap().to_string()
                } else {
                    number
                }
            },
            Object::Bool(b) => b.to_string()
        };

        println!("{}", value);

        Ok(Object::None)
    }

    fn declare(&mut self, name: Token, value: Box<Node>) -> Result<Object, String> {
        let v = self.traverse(*value)?;
        let enviro = self.current_environment();
        enviro.assign(name.value, v);

        Ok(Object::None)
    }

    #[allow(mutable_borrow_reservation_conflict)]
    fn assign(&mut self, id: usize, name: Token, value: Box<Node>) -> Result<Object, String> {
        let v = self.traverse(*value)?;
        let distance = self.depths.get(&id).unwrap();
        
        self.ancestor(distance.clone()).assign(name.value, v);
        Ok(Object::None)
    }

    fn logical(&mut self, left: Box<Node>, operator: Token, right: Box<Node>) -> Result<Object, String> {
        let l = self.traverse(*left)?;
        let r = self.traverse(*right)?;
        
        if operator._type == TokenType::Or {
            if self.is_truthy(&l) {
                return Ok(l)
            }
        } else if !self.is_truthy(&l) {
            return Ok(l)
        }

        Ok(r)

    }

    fn binary_operator(&mut self, left: Box<Node>, operator: Token, right: Box<Node>) -> Result<Object, String> {
        let l = self.traverse(*left)?;
        let r = self.traverse(*right)?;

        return match operator._type {
            TokenType::Plus => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val + right_val)),
                (Object::String(left_val), Object::String(right_val)) => Ok(Object::String(left_val + &right_val)),
                _ => Err(String::from("Left and right values must both be numbers or strings for additon."))
            },
            TokenType::Minus => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val - right_val)),
                _=> Err(String::from("Left and right values must both be numbers for subtraction."))
            },
            TokenType::Multiply => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val * right_val)),
                _=> Err(String::from("Left and right values must both be numbers for multiplication."))
            },
            TokenType::Divide => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val / right_val)),
                _=> Err(String::from("Left and right values must both be numbers for division."))
            },
            TokenType::Greater => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val > right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            TokenType::Less => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val < right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            TokenType::GreraterEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val >= right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            TokenType::LessEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val <= right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            TokenType::Equal => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val == right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            TokenType::NotEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val != right_val)),
                _=> Err(String::from("Left and right values must both be numbers for comparions."))
            },
            _ => Err(format!("Operator not implemented: {:?}", operator))
        }

    }

    fn unary(&mut self, operator: Token, c: Box<Node>) -> Result<Object, String> {
        let child = self.traverse(*c)?;

        return match operator._type {
            TokenType::Minus => match child {
                Object::Number(v) => Ok(Object::Number(-(v))),
                _ => Err(String::from("Value must be number when negating (-)."))
            },
            TokenType::Not => {
                let truthy = self.is_truthy(&child);
                return Ok(Object::Bool(!truthy))
            },
            _ => return Err(format!("Unrecognized unary operator: {:?}", operator))
        }

        // return Ok(Some(Object::Bool(true)))
    }

    fn literal(&self, node: Literal) -> Object {
        return match node {
            Literal::Number(v) => Object::Number(v),
            Literal::Bool(v) => Object::Bool(v),
            Literal::String(v) => Object::String(v),
            Literal::None => Object::None
        }
    }
}
