#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
    Comma
}
