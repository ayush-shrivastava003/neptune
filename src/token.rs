#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    String(String),
    Name(String),
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    ParOpen,
    ParClose,
    Greater,
    Less,
    GreraterEqual,
    LessEqual,
    Equal,
    NotEqual,
    BrackOpen,
    BrackClose,

    // Control flow
    If,
    Else,
    Not,
    While,
    For,
    Or,
    And,

    // Variables and assignment
    Assign,
    // Increment,
    // Decrement,
    FuncDeclare,
    Declare,
    Return,
    Print,
    Bool(bool),
    Separate,
    Eof,
    Comma,
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize
}