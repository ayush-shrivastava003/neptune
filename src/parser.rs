use crate::token::*;
use crate::syntax_tree::*;

pub struct Parser<'a> {
    tokens:  &'a Vec<Token>,
    token_index: usize,
    current_id: usize
}

impl <'a>Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {tokens, token_index: 0, current_id: 0}
    }

    fn current(&self) -> &Token {
        let token = self.tokens.get(self.token_index);
        &token.unwrap()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.token_index + 1)
    }

    fn next(&mut self) {
        if !matches!(self.current()._type, TokenType::Eof) {
            self.token_index += 1;
        }
    }

    fn new_id(&mut self) -> usize {
        let id = self.current_id;
        self.current_id += 1;
        id + 1
    }

    #[allow(unused_variables)] // for some reason my IDE complains about unused variables
    fn eat(&mut self, expected: &TokenType, msg: &str) {
        if &self.current()._type == expected || matches!(&self.current()._type, &TokenType::Name(_)) {
            self.next();
        } else {
            panic!("{} \x1b[32m{:?}\x1b[0m",  msg, &self.current())
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let mut nodes = Vec::<Node>::new();

        while self.current()._type != TokenType::Eof {
            nodes.push(self.statement()?);
        }

        Ok(Node::Block(nodes))
    }

    fn code_block(&mut self) -> Result<Node, String> {
        let mut nodes = Vec::<Node>::new();
        self.eat(&TokenType::BrackOpen, "Expected open bracket to code block.");

        while self.current()._type != TokenType::BrackClose {
            nodes.push(self.statement()?);
        }

        self.eat(&TokenType::BrackClose, "Expected closing bracket to code block.");
        Ok(Node::Block(nodes))
    }

    fn statement(&mut self) -> Result<Node, String> {
        return match self.current()._type {
            TokenType::Declare => self.declare_var(),
            TokenType::Name(_) => {
                let peek = self.peek();
                if peek != None && matches!(peek.unwrap()._type, TokenType::Assign) { // TODO: increment & decrement
                    self.assign()
                } else {
                    self.get_expression()
                }
            },
            // &Token::FuncDeclare => self.declare_func(),
            // &Token::Return => self.return(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::For => todo!(),
            TokenType::Print => self.print(),
            TokenType::BrackOpen => self.code_block(),
            _ => self.get_expression()
        }
    }

    fn print(&mut self) -> Result<Node, String> {
        self.eat(&TokenType::Print, "");
        let value = self.get_expression()?;
        self.eat(&TokenType::Separate, "Expected a separator after the statement.");

        Ok(Node::Print(
            Box::new(
                value
            )
        ))
    }

    pub fn declare_var(&mut self) -> Result<Node, String> {
        self.eat(&TokenType::Declare, "Expected 'let' in front of variable declaration.");
        let name = self.current().clone();

        self.eat(&TokenType::Name("".to_string()), "Expected a name after 'let' keyword.");
        self.eat(&TokenType::Assign, format!("Expected '=' to assign '{:?}' to a value", name).as_str());

        let value = self.get_expression()?;
        self.eat(&TokenType::Separate, "Expected a separator after the statement.");

        Ok(Node::Declare { name, value: Box::new(value), id: self.new_id() })
    }

    pub fn assign(&mut self) -> Result<Node, String> {
        let name = self.current().clone();
        self.eat(&TokenType::Name("".to_string()), "Expected a name when assigning a value.");
        self.eat(&TokenType::Assign, format!("Expected '=' to assign '{:?}' to a value", name).as_str());
        let value = self.get_expression()?;
        self.eat(&TokenType::Separate, "Expected a separator after the statement.");

        Ok(Node::Assign {name, value: Box::new(value), id: self.new_id() })
    }

    pub fn if_statement(&mut self) -> Result<Node, String> {
        self.eat(&TokenType::If, "Expected 'if' to starting if statement.");
        self.eat(&TokenType::ParOpen, "Expected open parenthesis to if statement.");

        let condition = self.get_expression()?;
        self.eat(&TokenType::ParClose, "Expected closing parenthesis to if statement.");

        let body = self.statement()?;
        let mut else_block: Option<Box<Node>> = None;

        if self.current()._type == TokenType::Else {
            self.eat(&TokenType::Else, "Expected else to if statement.");
            else_block = Some(Box::new(self.statement()?));
        }

        Ok(Node::If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_block,
            id: self.new_id()
        })

    }

    fn while_statement(&mut self) -> Result<Node, String> {
        self.eat(&TokenType::While, "Expected 'while' to start of while loop.");
        self.eat(&TokenType::ParOpen, "Expected '(' to condition of while loop.");
        let condition = Box::new(self.get_expression()?);
        self.eat(&TokenType::ParClose, "Expected ')' after condition of while loop.");
        let body = Box::new(self.statement()?);
        Ok(Node::While {condition, body, id: self.new_id() })
    }

    pub fn get_expression(&mut self) -> Result<Node, String> {
        Ok(self.or_statement()?)
    }

    fn or_statement(&mut self) -> Result<Node, String> {
        let mut expr = self.and_statement()?;

        while self.current()._type == TokenType::Or {
            let operator = self.current().clone();
            self.next();
            let right = self.and_statement()?;

            expr = Node::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                id: self.new_id()
            }
        }

        Ok(expr)
    }

    fn and_statement(&mut self) -> Result<Node, String> {
        let mut expr = self.equality()?;

        while self.current()._type == TokenType::And {
            let operator = self.current().clone();
            self.next();
            let right = self.equality()?;

            expr = Node::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                id: self.new_id()
            }
        }

        Ok(expr)
    }

    pub fn equality(&mut self) -> Result<Node, String> {
        let mut expr = self.comparison()?;

        while matches!(self.current()._type, TokenType::NotEqual | TokenType::Equal) {
            let operator = self.current().clone();
            self.next();
            let right = self.comparison()?;

            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
                id: self.new_id()
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Node, String> {
        let mut expr = self.term()?;

        while matches!(
            self.current()._type, 
            TokenType::Greater |
            TokenType::Less |
            TokenType::GreraterEqual |
            TokenType::LessEqual
        ) {
                let operator = self.current().clone();
                self.next();
                let right = self.term()?;
                expr = Node::BinaryOperator {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                    id: self.new_id()
                };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Node, String> {
        let mut expr = self.factor()?;

        while matches!(
            self.current()._type,
            TokenType::Plus |
            TokenType::Minus
        ) {
            let operator = self.current().clone();
            self.next();
            let right = self.factor()?;
            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
                id: self.new_id()
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Node, String> {
        let mut expr = self.unary()?;

        while matches!(
            self.current()._type,
            TokenType::Multiply |
            TokenType::Divide
        ) {
            let operator = self.current().clone();
            self.next();
            let right = self.unary()?;
            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
                id: self.new_id()
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Node, String> {
        if matches!(
            self.current()._type,
            TokenType::Not |
            TokenType::Minus
        ) {
            let operator = self.current().clone();
            self.next();
            let child = self.unary()?;
            return Ok(Node::UnaryOperator { operator: operator, child: Box::new(child), id: self.new_id() })
        } else {
            return Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Node, String> {
        let id = self.new_id();
        let expr = match &self.current()._type {
            TokenType::Number(value) => Node::Literal {
                value: Literal::Number(*value),
                id
            },
            TokenType::Bool(value) => Node::Literal {
                value: Literal::Bool(*value),
                id
            },
            TokenType::String(value) => Node::Literal {
                id,
                value: Literal::String(value.to_string())
            },
            TokenType::ParOpen => {
                self.next();
                let node = self.get_expression()?;
                // self.eat(&Token::ParClose, "Expected clsoing parenthesis to expression.");
                node
            },
            TokenType::None => Node::Literal {value: Literal::None, id: self.new_id() },
            TokenType::Name(_) => Node::Variable { id: self.new_id(), name: self.current().clone() },
            _ => return Err(format!("Couldn't identify this token: {:?}", self.current()))
        };
        self.next();
        Ok(expr)
    }
}