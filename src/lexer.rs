use crate::token::*;

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
        let _type = match word.as_ref() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "fn" => TokenType::FuncDeclare,
            "let" => TokenType::Declare,
            "return" => TokenType::Return,
            "print" => TokenType::Print,
            "or" => TokenType::Or,
            "and" => TokenType::And,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false), 
            "none" => TokenType::None,          
            _ => TokenType::Name(word.clone())
        };

        self.index -= 1;

        Token {
            _type,
            value: word
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
        Ok(Token {
            _type: TokenType::Number(number.parse::<f64>().unwrap()),
            value: number
        })
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
        Ok(Token { _type: TokenType::String(string.clone()), value: string})
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
                '*' => tokens.push(Token {_type: TokenType::Multiply, value: "*".to_string() }),
                '/' => tokens.push(Token {_type: TokenType::Divide, value: "/".to_string() }),
                '(' => tokens.push(Token {_type: TokenType::ParOpen, value: "(".to_string() }),
                ')' => tokens.push(Token {_type: TokenType::ParClose, value: ")".to_string() }),
                '{' => tokens.push(Token {_type: TokenType::BrackOpen, value: "{".to_string() }),
                '}' => tokens.push(Token {_type: TokenType::BrackClose, value: "}".to_string() }),
                '+' => tokens.push(Token {_type: TokenType::Plus, value: "+".to_string() }),
                '-' => tokens.push(Token {_type: TokenType::Minus, value: "-".to_string() }),
                ';' => tokens.push(Token {_type: TokenType::Separate, value: ";".to_string() }),
                ',' => tokens.push(Token {_type: TokenType::Comma, value: ",".to_string() }),
                '>' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::GreraterEqual, value: ">=".to_string() });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Greater, value: ">".to_string() });
                    } 
                },

                '<' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::LessEqual, value: "<=".to_string() });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Less, value: "<".to_string() });
                    }
                },

                '=' => {

                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::Equal, value: "==".to_string() });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Assign, value: "=".to_string() });
                    }
                },

                '!' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::NotEqual, value: "!=".to_string() });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Not, value: "!".to_string() });
                    }
                },

                _ => return Err(format!("Unkown character '{}'", chr)) 
            }
            self.increment();
        }
        tokens.push(Token {_type: TokenType::Eof, value: "<eof>".to_string()});
        Ok(tokens)
    }
}