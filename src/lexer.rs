use crate::token::*;
use crate::error::Error;
use line_col::LineColLookup;

pub struct Lexer<'a> {
    content: Vec::<char>,
    index: usize,
    chr: Option<char>,
    lc_lookup: LineColLookup<'a>
}

impl <'a>Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        let content: Vec::<char> = source.chars().collect();
        let index = 0;
        let chr = if content.len() != 0 {
            content[index]
        } else {' '}; // if the file is empty (just in case)
        Self {
            content,
            index,
            chr: Some(chr),
            lc_lookup: LineColLookup::new(&source)
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

    fn get_word(&mut self, line: usize, column: usize) -> Token {
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
            value: word,
            line,
            column
        }
    }

    fn get_number(&mut self, line: usize, column: usize) -> Result<Token, Error> {
        let mut number = String::new();
        while self.chr != None && (
            self.unwrap().is_numeric() ||
            self.unwrap() == '.'
        ) {
            number.push(self.unwrap());
            self.increment();
        }

        if number.matches(".").count() > 1 {
            return Err(Error::Syntax(format!("Invalid float. [{}:{}]", line, column)))
        }
        self.index -= 1;
        self.chr = Some(self.content[self.index]);
        Ok(Token {
            _type: TokenType::Number(number.parse::<f64>().unwrap()),
            value: number,
            line,
            column
        })
    }

    fn get_str(&mut self, line: usize, column: usize) -> Result<Token, Error> {
        let mut string = String::new();
        while self.chr != None && (
            self.unwrap() != '"'
        ) {
            string.push(self.unwrap());
            self.increment();
        }
        if self.chr == None {
            return Err(Error::Syntax(format!("Unterminated string. [{}:{}]", line, column)))
        }
        Ok(Token { _type: TokenType::String(string.clone()), value: string, line, column})
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        while self.chr != None {
            let chr = self.chr.unwrap();
            let lc = self.lc_lookup.get(self.index);
            match chr { // get ready for a big boy match statement
                chr if chr.is_whitespace() => {}, // skip to increment
    
                chr if chr.is_alphabetic() => tokens.push(self.get_word(lc.0, lc.1)),

                chr if chr.is_numeric() => {
                    tokens.push(self.get_number(lc.0, lc.1)?);
                },

                '"' => {
                    self.increment();
                    tokens.push(self.get_str(lc.0, lc.1)?);
                },
                '*' => tokens.push(Token {_type: TokenType::Multiply, value: "*".to_string(), line: lc.0, column: lc.1 }),
                '/' => tokens.push(Token {_type: TokenType::Divide, value: "/".to_string(), line: lc.0, column: lc.1 }),
                '(' => tokens.push(Token {_type: TokenType::ParOpen, value: "(".to_string(), line: lc.0, column: lc.1 }),
                ')' => tokens.push(Token {_type: TokenType::ParClose, value: ")".to_string(), line: lc.0, column: lc.1 }),
                '{' => tokens.push(Token {_type: TokenType::BrackOpen, value: "{".to_string(), line: lc.0, column: lc.1 }),
                '}' => tokens.push(Token {_type: TokenType::BrackClose, value: "}".to_string(), line: lc.0, column: lc.1 }),
                '+' => tokens.push(Token {_type: TokenType::Plus, value: "+".to_string(), line: lc.0, column: lc.1 }),
                '-' => tokens.push(Token {_type: TokenType::Minus, value: "-".to_string(), line: lc.0, column: lc.1 }),
                ';' => tokens.push(Token {_type: TokenType::Separate, value: ";".to_string(), line: lc.0, column: lc.1 }),
                ',' => tokens.push(Token {_type: TokenType::Comma, value: ",".to_string(), line: lc.0, column: lc.1 }),
                '>' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::GreraterEqual, value: ">=".to_string(), line: lc.0, column: lc.1 });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Greater, value: ">".to_string(), line: lc.0, column: lc.1 });
                    } 
                },

                '<' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::LessEqual, value: "<=".to_string(), line: lc.0, column: lc.1 });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Less, value: "<".to_string(), line: lc.0, column: lc.1 });
                    }
                },

                '=' => {

                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::Equal, value: "==".to_string(), line: lc.0, column: lc.1 });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Assign, value: "=".to_string(), line: lc.0, column: lc.1 });
                    }
                },

                '!' => {
                    if self.is_peek_equal() {
                        tokens.push(Token {_type: TokenType::NotEqual, value: "!=".to_string(), line: lc.0, column: lc.1 });
                        self.increment();
                    } else {
                        tokens.push(Token {_type: TokenType::Not, value: "!".to_string(), line: lc.0, column: lc.1 });
                    }
                },

                _ => return Err(Error::Syntax(format!("Unkown character '{}' [{}:{}]", chr, lc.0, lc.1)))
            }
            self.increment();
        }
        let lc = self.lc_lookup.get(self.index);
        tokens.push(Token {_type: TokenType::Eof, value: "<eof>".to_string(), line: lc.0, column: lc.1 });
        Ok(tokens)
    }
}
