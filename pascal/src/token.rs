/// Token types for Pascal interpreter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    Begin,
    End,
    
    // Identifiers and literals
    Id,
    Integer,
    Real,
    
    // Operators
    Assign,  // :=
    Plus,    // +
    Minus,   // -
    Mult,    // *
    Div,     // /
    
    // Delimiters
    LParen,   // (
    RParen,   // )
    Semi,     // ;
    Dot,      // .
    
    // Special
    Eol,
    Eof,
}

/// Represents a single token
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, col: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            col,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.token_type, self.value)
    }
}

