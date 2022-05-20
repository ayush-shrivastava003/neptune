use crate::{interpreter::Interpreter, syntax_tree::Node, token::*};
use std::{collections::HashMap};

pub struct SemanticAnalyzer<'b> {
    interpreter: &'b mut Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl <'b>SemanticAnalyzer<'b> {
    pub fn new(interpreter: &'b mut Interpreter) -> Self {
        Self { interpreter, scopes: vec![] }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::<String, bool>::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, false);
    }

    fn define(&mut self, name: String) {
        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, true);
    }

    #[allow(unused_variables)]
    fn resolve_local(&mut self, expr: &usize, name: String) {
        let mut counter = 0;
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(&name) {
                let position = self.scopes.iter().position(|s| matches!(s, scope));
                let distance = self.scopes.len() - 1 - position.unwrap();
                self.interpreter.resolve(*expr, distance);
                return
            }
            counter += 1;
        }
    }

    pub fn resolve(&mut self, expr: &Node) {
        match expr {
            Node::Block(vec) => {
                self.begin_scope();
                for node in vec.iter() {
                    self.resolve(node)
                }
                self.end_scope();
            },
            Node::Assign { id, name, value} => self.assign(id, name, value),
            Node::BinaryOperator {left, right, .. } => self.binary(left, right),
            Node::Logical { left, right, .. } => self.binary(left, right),
            Node::UnaryOperator {child, ..} => self.unary(child),
            Node::Declare {name, value, .. } => self.resolve_declare(name, value),
            Node::If { condition, body, else_block, .. } => self.if_block(condition, body, else_block),
            Node::While { condition, body, .. } => self.while_block(condition, body),
            Node::Variable {id, name} => self.variable(id, name),
            Node::Print(expr) => self.resolve(expr),
            _ => return // literal
        }
    }

    fn resolve_declare(&mut self, name: &Token, value: &Box<Node>) {
        self.declare(name.value.clone());
        self.resolve(value);
        self.define(name.value.clone())
    }

    fn assign(&mut self, id: &usize, name: &Token, value: &Box<Node>) {
        self.resolve_local(id, name.value.clone());
        self.resolve(value)
    }

    fn binary(&mut self, left: &Box<Node>, right: &Box<Node>) {
        self.resolve(left);
        self.resolve(right);
    }

    fn unary(&mut self, child: &Box<Node>) {
        self.resolve(child)
    }

    fn if_block(&mut self, condition: &Box<Node>, if_block: &Box<Node>, else_block: &Option<Box<Node>>) {
        self.resolve(condition);
        self.resolve(if_block);
        if let Some(block) = else_block {
            self.resolve(block)
        }
    }

    fn while_block(&mut self, condition: &Box<Node>, body: &Box<Node>) {
        self.resolve(condition);
        self.resolve(body);
    }

    fn variable(&mut self, id: &usize, name: &Token) {
        let scope = self.scopes.last_mut().unwrap();
        let is_defined = scope.get(&name.value);
        
        if is_defined != None && is_defined.unwrap() == &false {
            panic!("'{:?}' cannot be read in its own declaration.", name)
        }

        self.resolve_local(id, name.value.clone());
    }
}