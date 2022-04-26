use crate::token::Token;
use crate::syntax_tree::*;

pub struct Parser<'a> {
    tokens:  &'a Vec<Token>,
    token_index: usize  
}

impl <'a>Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {tokens, token_index: 0}
    }

    fn current(&self) -> &Token {
        let g = self.tokens.get(self.token_index);
        if g != None {
            return g.unwrap();
        } else {
            return &Token::Eof
        }
    }

    fn next(&mut self) {
        if !matches!(self.current(), &Token::Eof) {
            self.token_index += 1;
        }
    }

    // fn eat(&mut self, expected: &Token, msg: &str) {
    //     if matches!(self.current(), expected) {
    //         self.next();
    //     } else {
    //         panic!("mismatched tokens: wanted {:?}, got: {:?}", self.current(), expected)
    //     }
    // }

    pub fn parse(&mut self) -> Result<Node, String> {
        Ok(self.get_expression()?)
    }

    pub fn get_expression(&mut self) -> Result<Node, String> {
        Ok(self.or_statement()?)
    }

    fn or_statement(&mut self) -> Result<Node, String> {
        let mut expr = self.and_statement()?;

        while matches!(self.current(), &Token::Or) {
            let operator = self.current().clone();
            self.next();
            let right = self.and_statement()?;

            expr = Node::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn and_statement(&mut self) -> Result<Node, String> {
        let mut expr = self.equality()?;

        while matches!(self.current(), &Token::And) {
            let operator = self.current().clone();
            self.next();
            let right = self.equality()?;

            expr = Node::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    pub fn equality(&mut self) -> Result<Node, String> {
        let mut expr = self.comparison()?;

        while matches!(self.current(), &Token::NotEqual | &Token::Equal) {
            let operator = self.current().clone();
            self.next();
            let right = self.comparison()?;

            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Node, String> {
        let mut expr = self.term()?;

        while matches!(
            self.current(), 
            &Token::Greater |
            &Token::Less |
            &Token::GreraterEqual |
            &Token::LessEqual
        ) {
                let operator = self.current().clone();
                self.next();
                let right = self.term()?;
                expr = Node::BinaryOperator {
                    left: Box::new(expr),
                    operator: operator,
                    right: Box::new(right)
                };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Node, String> {
        let mut expr = self.factor()?;

        while matches!(
            self.current(),
            &Token::Plus |
            &Token::Minus
        ) {
            let operator = self.current().clone();
            self.next();
            let right = self.factor()?;
            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Node, String> {
        let mut expr = self.unary()?;

        while matches!(
            self.current(),
            &Token::Multiply |
            &Token::Divide
        ) {
            let operator = self.current().clone();
            self.next();
            let right = self.unary()?;
            expr = Node::BinaryOperator {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right)
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Node, String> {
        if matches!(
            self.current(),
            &Token::Not |
            &Token::Minus
        ) {
            let operator = self.current().clone();
            self.next();
            let child = self.unary()?;
            return Ok(Node::UnaryOperator { operator: operator, child: Box::new(child) })
        } else {
            return Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Node, String> {
        let expr = match self.current() {
            &Token::Number(value) => {Node::Literal(Literal::Number(value))},
            &Token::Bool(value) => Node::Literal(Literal::Bool(value)),
            Token::String(value) => Node::Literal(Literal::String(value.to_string())),
            &Token::ParOpen => {
                self.next();
                let node = self.get_expression()?;
                // self.eat(&Token::ParClose, "Expected clsoing parenthesis to expression.");
                node
            },
            _ => return Err(format!("Couldn't identify this token: {:?}", self.current()))
        };
        self.next();
        Ok(expr)
    }
}