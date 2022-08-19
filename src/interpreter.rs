use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::{
    ast::*,
    token::*,
    error::Error,
    function::Function
};

#[derive(Clone)]
pub enum Object { // wrapper for multiple data types
    Number(f64),
    Bool(bool),
    String(String),
    None,
    Function(Function)
}

pub struct Interpreter {
    pub environments: Vec<HashMap<String, Object>>,
    pub depths: HashMap<usize, usize>,
    globals: HashMap<String, Object>
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals: HashMap<String, Object> = HashMap::new();
        globals.insert("print".to_string(), Object::Function(Function::Native {
            arg_len: 1,
            body: Box::new(|args| {
                println!("{}", args[0]);
                Ok(Object::None)
            }),
            name: "print".to_string()
        }));
        globals.insert("time".to_string(), Object::Function(Function::Native {
            arg_len: 0,
            body: Box::new(|_args| {
                Ok(Object::Number(
                    SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("failure!").as_millis() as f64
                ))
            }),
            name: "time".to_string()
        }));
        Self { environments: vec![], depths: HashMap::new(), globals }
    }

    pub fn is_truthy(&self, obj: &Object) -> bool {
        match obj {
            &Object::Bool(v) => v,
            &Object::None => false,
            _ => true
          }
    }

    fn lookup(&mut self, name: &Token, id: &usize) -> Result<Object, Error> {
        let distance = self.depths.get(id);
        // println!("{:?} ({}) is at a depth of {:?}", name, id, distance);
        if let None = distance {
            let obj = self.globals.get(&name.value);
            if let None = obj {
                return Err(Error::Runtime(format!("Unkown variable '{}' [{}:{}]", name.value, name.line, name.column)))
            } else {
                return Ok(obj.unwrap().clone());
            }
        }

        for (i, enviro) in self.environments.iter().rev().enumerate() {
            if i == distance.unwrap().clone() {
                let obj = enviro.get(&name.value);
                return Ok(obj.unwrap().clone())
            }
        }

        Ok(Object::None)
    }

    pub fn run(&mut self, nodes: &Vec<Node>) -> Result<Object, Error> {
        let f = self.traverse_block(nodes);
        match f {
            Err(Error::Runtime(v)) => Err(Error::Runtime(v)),
            Ok(v) => Ok(v),
            _ => Ok(Object::None)
        }
    }

    pub fn traverse(&mut self, node: &Node) -> Result<Object, Error> {
        match node {
            Node::Block(nodes) => {
                self.environments.push(HashMap::new());
                // println!("{:?}", self.environments);
                // println!("\x1b[32mMake (block).\x1b[0m");
                // println!("after len: {}", self.environments.len());
                // println!("Made block for: {:#?}", nodes);
                Ok(self.traverse_block(nodes)?)
            },
            Node::BinaryOperator {left: l, operator: o, right: r, ..} => Ok(self.binary_operator(l, o, r)?),
            Node::UnaryOperator { operator: o, child: c, .. } => Ok(self.unary(o, c)?),
            Node::Logical {left: l, operator: o, right: r, ..} => Ok(self.logical(l, o, r)?),
            Node::Literal { value: lit, .. } => Ok(self.literal(lit)),
            Node::Declare {name, value, .. } => Ok(self.declare(name, value)?),
            Node::Assign {id, name, value} => Ok(self.assign(id, name, value)?),
            Node::Variable { id, name } => Ok(self.variable(name, id)?),
            Node::If { condition, body, else_block, ..} => Ok(self.if_block(condition, body, else_block)?),
            Node::While { condition, body, .. } => Ok(self.while_block(&condition, &body)?),
            Node::DeclareFn { name, args, body, .. } => Ok(self.declare_fn(name, args, body)?),
            Node::Return { value, .. } => Ok(self.return_statement(value)?),
            Node::FnCall { name, args, .. } => Ok(self.call(name, args)?),
            // _ => todo!()
        }
    }

    pub fn traverse_block(&mut self, stmts: &Vec<Node>) -> Result<Object, Error> {
        for stmts in stmts {
            match self.traverse(stmts) {
                Err(Error::Runtime(v)) => return Err(Error::Runtime(v)),
                Err(Error::Return(v)) => {
                    // println!("\x1b[31mPurge (block).\x1b[0m");
                    self.environments.pop();
                    // println!("after len: {}", self.environments.len());
                    return Err(Error::Return(v))
                },
                _ => continue
            }
        }

        // println!("\x1b[31mPurge (block).\x1b[0m");
        self.environments.pop();
        // println!("after len: {}", self.environments.len());
        Ok(Object::None)
    }

    fn call(&mut self, name: &Box<Node>, given_args: &Vec<Node>) -> Result<Object, Error> {
        let func = self.traverse(&**name)?;
        match func {
            Object::Function(mut f) => {
                match f {
                    Function::UserDefined { ref args, ..} => {
                        let mut evaled_args = vec![];
                
                        if given_args.len() != args.len() {
                            return Err(Error::Runtime(format!("Expected {} arguments, found {}.", args.len(), given_args.len())))
                        }
        
                        for arg in given_args {
                            evaled_args.push(self.traverse(arg)?)
                        }
        
                        Ok(f.call(self, evaled_args)?)
                    },
                    Function::Native { arg_len, .. } => {
                        let mut evaled_args = vec![];

                        if given_args.len() != arg_len {
                            return Err(Error::Runtime(format!("Expected {} arguments, found {}.", arg_len, given_args.len())))
                        }

                        for arg in given_args {
                            evaled_args.push(self.traverse(arg)?)
                        }

                        Ok(f.call(self, evaled_args)?)
                    }
                }
            }
            _ => return Err(Error::Runtime(format!("Can only call functions, not {}", func)))
        }
    }

    fn declare_fn(&mut self, name: &Token, args: &Vec<Token>, body: &Box<Node>) -> Result<Object, Error> {
        let function = Function::UserDefined { args: args.clone(), body: *body.clone(), name: name.clone() };
        if self.environments.len() == 0 {
            self.globals.insert(name.value.clone(), Object::Function(function));
        } else {
            self.environments.last_mut().unwrap().insert(name.value.clone(), Object::Function(function));
        }
        Ok(Object::None)
    }

    fn return_statement(&mut self, value: &Box<Node>) -> Result<Object, Error> {
        let v = self.traverse(value)?;
        Err(Error::Return(v))
    }
    
    pub fn variable(&mut self, name: &Token, id: &usize) -> Result<Object, Error> {
        self.lookup(&name, id)
    }
    
    pub fn declare(&mut self, name: &Token, value: &Box<Node>) -> Result<Object, Error> {
        let v = self.traverse(value)?;
        if self.environments.len() != 0 {
            self.environments.last_mut().unwrap().insert(name.value.clone(), v);
        } else {
            self.globals.insert(name.value.clone(), v);
        }
        Ok(Object::None)
    }
    
    fn assign(&mut self, id: &usize, name: &Token, value: &Box<Node>) -> Result<Object, Error> {
        let v = self.traverse(value)?;
        let distance = self.depths.get(&id);
        if let None = distance {
            self.globals.insert(name.value.clone(), v);
        } else {
            for (i, enviro) in self.environments.iter_mut().rev().enumerate() {
                if i == distance.unwrap().clone() {
                    enviro.insert(name.value.clone(), v);
                    break
                }
            }
        }
        Ok(Object::None)
    }
    
    fn while_block(&mut self, condition: &Box<Node>, body: &Box<Node>) -> Result<Object, Error> {
        let mut c = self.traverse(condition)?;
        while self.is_truthy(&c) {
            self.traverse(&**body)?;
            c = self.traverse(condition)?;
        }

        Ok(Object::None)
    }
    
    fn if_block(&mut self, condition: &Box<Node>, body: &Box<Node>, else_block: &Option<Box<Node>>) -> Result<Object, Error> {
        let c = self.traverse(condition)?;
        if self.is_truthy(&c) {
            let b = self.traverse(body)?;
            return Ok(b)
        } else if else_block != &None {
            return Ok(self.traverse(else_block.as_ref().unwrap())?)
        } else {
            return Ok(Object::None)
        }
    }

    fn logical(&mut self, left: &Box<Node>, operator: &Token, right: &Box<Node>) -> Result<Object, Error> {
        let l = self.traverse(left)?;
        let r = self.traverse(right)?;
        
        if operator._type == TokenType::Or {
            if self.is_truthy(&l) {
                return Ok(l)
            }
        } else if !self.is_truthy(&l) {
            return Ok(l)
        }

        Ok(r)

    }

    fn binary_operator(&mut self, left: &Box<Node>, operator: &Token, right: &Box<Node>) -> Result<Object, Error> {
        let l = self.traverse(left)?;
        let r = self.traverse(right)?;

        return match operator._type {
            TokenType::Plus => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val + right_val)),
                (Object::String(left_val), Object::String(right_val)) => Ok(Object::String(left_val + &right_val)),
                _ => Err(Error::Runtime(format!("Left and right values must both be numbers or strings for additon. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Minus => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val - right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for subtraction. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Multiply => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val * right_val)),
                (Object::String(left_val), Object::Number(right_val)) => Ok(Object::String(left_val.repeat(right_val as usize))),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for multiplication. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Divide => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Number(left_val / right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for division. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Greater => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val > right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Less => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val < right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::GreraterEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val >= right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::LessEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val <= right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Equal => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val == right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            TokenType::NotEqual => match (l, r) {
                (Object::Number(left_val), Object::Number(right_val)) => Ok(Object::Bool(left_val != right_val)),
                _=> Err(Error::Runtime(format!("Left and right values must both be numbers for comparions. [{}:{}]", operator.line, operator.column)))
            },
            _ => Err(Error::Runtime(format!("Operator not implemented: {:?}", operator)))
        }

    }

    fn unary(&mut self, operator: &Token, c: &Box<Node>) -> Result<Object, Error> {
        let child = self.traverse(c)?;

        return match operator._type {
            TokenType::Minus => match child {
                Object::Number(v) => Ok(Object::Number(-(v))),
                _ => Err(Error::Runtime(format!("Value must be number when negating (-). [{}:{}]", operator.line, operator.column)))
            },
            TokenType::Not => {
                let truthy = self.is_truthy(&child);
                return Ok(Object::Bool(!truthy))
            },
            _ => return Err(Error::Runtime(format!("Unrecognized unary operator: {:?} [{}:{}]", operator, operator.line, operator.column)))
        }
    }

    fn literal(&self, node: &Literal) -> Object {
        return match node {
            Literal::Number(v) => Object::Number(*v),
            Literal::Bool(v) => Object::Bool(*v),
            Literal::String(v) => Object::String(v.clone().to_string()),
            Literal::None => Object::None
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Bool(b) => write!(f, "{}", b),
            Object::Function(func) => write!(f, "{}", func),
            Object::Number(n) => {
                let number = n.to_string();
                if number.ends_with(".0") {
                    write!(f, "{}", number.strip_suffix(".0").unwrap().to_string())
                } else {
                    write!(f, "{}", number)
                }
            },
            Object::String(s) => write!(f, "{}", s),
            Object::None => write!(f, "none")
        }
    }
}