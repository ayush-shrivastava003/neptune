use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    content: Vec::<char>,
    index: usize,
    chr: Option<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let chars: Vec::<char> = source.chars().collect();
        let index = 0;
        let chr = if chars.len() != 0 {
            chars[index]
        } else {' '}; // if the file is empty (just in case)
        Self {
            content: chars,
            index: index,
            chr: Some(chr),
        }
    }

    fn unwrap(&self) -> char { // gets character that is hidden behind an option.
        self.chr.unwrap()
    }

    fn increment(&mut self) {
        self.index += 1;
        if self.index < self.content.len() {
            let chr = self.content[self.index];
            self.chr = Some(chr);
        } else {
            self.chr = None;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index + 1 < self.content.len() {
            Some(self.content[self.index + 1])
        } else {
            None
        }
    }

    fn is_peek_equal(&self) -> bool { // checks if the next character is an equal sign.
        let peek_chr = self.peek();
        peek_chr != None && peek_chr.unwrap() == '='
    }

    fn get_word(&mut self) -> Token {
        let mut word = String::new();
        while self.chr != None && ( // ik this formatting is disgusting
            self.unwrap().is_alphanumeric() || 
            self.unwrap() == '_'
        ) {
            word.push(self.unwrap());
            self.increment();
        }
        match word.as_ref() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "fn" => Token::FuncDeclare,
            "let" => Token::Declare,
            "return" => Token::Return,
            "print" => Token::Print,
            "or" => Token::Or,
            "and" => Token::And,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),            
            _ => Token::Name(word)
        }
    }

    fn get_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();
        while self.chr != None && (
            self.unwrap().is_numeric() ||
            self.unwrap() == '.'
        ) {
            number.push(self.unwrap());
            self.increment();
        }

        if number.matches(".").count() > 1 {
            return Err(String::from("Invalid float."))
        }
        self.index -= 1;
        self.chr = Some(self.content[self.index]);
        Ok(Token::Number(number.parse::<f64>().unwrap()))
    }

    fn get_str(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        while self.chr != None && (
            self.unwrap() != '"'
        ) {
            string.push(self.unwrap());
            self.increment();
        }
        if self.chr == None {
            return Err(String::from("Unterminated string."))
        }
        Ok(Token::String(string))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        while self.chr != None {
            let chr = self.chr.unwrap();
            match chr { // get ready for a big boy match statement
                chr if chr.is_whitespace() => {}, // skip to increment
    
                chr if chr.is_alphabetic() => tokens.push(self.get_word()),

                chr if chr.is_numeric() => {
                    tokens.push(self.get_number()?);
                },

                '"' => {
                    self.increment();
                    tokens.push(self.get_str()?);
                },
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '(' => tokens.push(Token::ParOpen),
                ')' => tokens.push(Token::ParClose),
                '{' => tokens.push(Token::BrackOpen),
                '}' => tokens.push(Token::BrackClose),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                ';' => tokens.push(Token::Separate),
                ',' => tokens.push(Token::Comma),
                '>' => {
                    if self.is_peek_equal() {
                        tokens.push(Token::GreraterEqual);
                        self.increment();
                    } else {
                        tokens.push(Token::Greater);
                    } 
                },

                '<' => {
                    if self.is_peek_equal() {
                        tokens.push(Token::LessEqual);
                        self.increment();
                    } else {
                        tokens.push(Token::Less);
                    }
                },

                '=' => {

                    if self.is_peek_equal() {
                        tokens.push(Token::Equal);
                        self.increment();
                    } else {
                        tokens.push(Token::Assign);
                    }
                },

                '!' => {
                    if self.is_peek_equal() {
                        tokens.push(Token::NotEqual);
                        self.increment();
                    } else {
                        tokens.push(Token::Not);
                    }
                },

                _ => return Err(format!("Unkown character '{}'", chr)) 
            }
            self.increment();
        }
        Ok(tokens)
    }
}