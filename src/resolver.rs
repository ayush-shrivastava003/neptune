use crate::{ast::*, token::Token, interpreter::Interpreter, error::Error};
use std::collections::HashMap;

pub struct Resolver<'a> {
    scopes: Vec<HashMap<String, bool>>,
    interpreter: &'a mut Interpreter,
    is_fn: bool
}

impl <'a>Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self { scopes: vec![], interpreter, is_fn: false }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        if !self.scopes.is_empty() {
            let scope = self.scopes.last_mut().unwrap();
            scope.insert(name, false);
        }
    }

    fn define(&mut self, name: String) {
        if !self.scopes.is_empty() {
            let scope = self.scopes.last_mut().unwrap();
            scope.insert(name, true);
        }
    }

    fn resolve_local(&mut self, expr: &usize, name: String) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name) {
                // println!("{:?}; {}", self.scopes, i);
                self.interpreter.depths.insert(*expr, i);
                return
            }
        }

        // let mut i = self.scopes.len() - 1;
        // while i >= 0 {
        //     if self.scopes[i].contains_key(&name) {
        //         self.interpreter.depths.insert(*expr, self.scopes.len() - 1 - i);
        //         return
        //     }
        //     i = i - 1;
        // }
    }

    pub fn resolve_block(&mut self, nodes: &Vec<Node>) -> Result<(), Error> {
        for n in nodes {
            self.resolve(n)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Node) -> Result<(), Error> {
        match expr {
            Node::Block(vec) => {
                // println!("\x1b[32mMake (Block (resolver.rs)).\x1b[0m");
                // println!("Made block for: {:#?}", vec);
                self.begin_scope();
                for node in vec {
                    self.resolve(node)?
                }
                // println!("\x1b[31mPurge (Block (resolver.rs)).\x1b[0m");
                self.end_scope();
                Ok(())
            },
            Node::Assign { id, name, value} => Ok(self.assign(id, name, value)?),
            Node::BinaryOperator {left, right, .. } => Ok(self.binary(left, right)?),
            Node::Logical { left, right, .. } => Ok(self.binary(left, right)?),
            Node::UnaryOperator {child, ..} => Ok(self.unary(child)?),
            Node::Declare {name, value, .. } => Ok(self.resolve_declare(name, value)?),
            Node::If { condition, body, else_block, .. } => Ok(self.if_block(condition, body, else_block)?),
            Node::While { condition, body, .. } => Ok(self.while_block(condition, body)?),
            Node::Variable {id, name} => Ok(self.variable(id, name)?),
            Node::Print(expr) => self.resolve(expr),
            Node::DeclareFn { id, name, args, body } => Ok(self.declare_fn(id, name, args, body)?),
            Node::Return { value, .. } => self.return_statement(value),
            Node::FnCall { name, args, .. } => self.call(name, args),
            _ => Ok(()) // literal
        }
    }

    fn call(&mut self, name: &Box<Node>, args: &Vec<Node>) -> Result<(), Error> {
        self.resolve(name)?;

        for arg in args {
            self.resolve(arg)?;
        }

        Ok(())
    }

    fn declare_fn(&mut self, id: &usize, name: &Token, args: &Vec<Token>, body: &Box<Node>) -> Result<(), Error> {
        let was = self.is_fn;
        self.is_fn = true;
        self.declare(name.value.clone());
        self.define(name.value.clone());

        // println!("\x1b[32mMake (Function (resolver.rs)).\x1b[0m");
        self.begin_scope();
        for arg in args {
            self.declare(arg.value.clone());
            self.define(arg.value.clone());
        }
        self.resolve(body)?;
        self.resolve_local(id, name.value.clone());
        // println!("\x1b[31mPurge (Function (resolver.rs)).\x1b[0m");
        self.end_scope();
        self.is_fn = was;
        Ok(())
    }

    fn return_statement(&mut self, value: &Box<Node>) -> Result<(), Error> {
        if !self.is_fn {
            Err(Error::Syntax(format!("Cannot return outside of a function declaration.")))
        } else {
            Ok(self.resolve(value)?)
        }
    }

    fn assign(&mut self, id: &usize, name: &Token, value: &Box<Node>) -> Result<(), Error> {
        self.resolve(value)?;
        self.resolve_local(id, name.value.clone());
        Ok(())
    }

    fn resolve_declare(&mut self, name: &Token, value: &Box<Node>) -> Result<(), Error> {
        self.declare(name.value.clone());
        self.resolve(value)?;
        self.define(name.value.clone());
        Ok(())
    }

    fn variable(&mut self, id: &usize, name: &Token) -> Result<(), Error> {
        if !self.scopes.is_empty() {
            let scope = self.scopes.last().unwrap();
            let is_defined = scope.get(&name.value);
            if is_defined != None && is_defined.unwrap() == &false {
                return Err(Error::Syntax(format!("'{:?}' cannot be read in its own declaration. [{}:{}]", name, name.line, name.column)))
            }
        }

        self.resolve_local(id, name.value.clone());
        Ok(())
    }

    fn while_block(&mut self, condition: &Box<Node>, body: &Box<Node>) -> Result<(), Error> {
        self.resolve(condition)?;
        self.resolve(body)?;
        Ok(())
    }

    fn if_block(&mut self, condition: &Box<Node>, body: &Box<Node>, else_block: &Option<Box<Node>>) -> Result<(), Error> {
        self.resolve(condition)?;
        self.resolve(body)?;
        if let Some(e) = else_block {
            self.resolve(e)?;
        }
        Ok(())
    }

    fn binary(&mut self, left: &Box<Node>, right: &Box<Node>) -> Result<(), Error> {
        self.resolve(left)?;
        self.resolve(right)?;
        Ok(())
    }

    fn unary(&mut self, child: &Box<Node>) -> Result<(), Error> {
        self.resolve(child)?;
        Ok(())
    }
}