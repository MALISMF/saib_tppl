use crate::token::Token;

/// AST node types
#[derive(Debug, Clone)]
pub enum Node {
    /// Root node: BEGIN ... END.
    Program {
        statement: Box<Node>,
    },
    /// BEGIN ... END block
    CompoundStatement {
        statements: Vec<Box<Node>>,
    },
    /// Variable := Expression
    Assignment {
        var_name: String,
        expr: Box<Node>,
    },
    /// Binary operation: left op right
    BinOp {
        left: Box<Node>,
        op: Token,
        right: Box<Node>,
    },
    /// Unary operation: op operand
    UnaryOp {
        op: Token,
        operand: Box<Node>,
    },
    /// Numeric literal
    Number {
        token: Token,
        value: f64,
    },
    /// Variable reference
    Variable {
        name: String,
    },
    /// Empty statement
    Empty,
}

impl Node {
    pub fn number(token: Token) -> Self {
        let value = if matches!(token.token_type, crate::token::TokenType::Real) {
            token.value.parse::<f64>().unwrap()
        } else {
            token.value.parse::<i64>().unwrap() as f64
        };
        Node::Number { token, value }
    }
}

