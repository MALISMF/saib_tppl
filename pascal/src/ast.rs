use crate::token::Token;

/// Узлы AST (абстрактного синтаксического дерева)
#[derive(Debug, Clone)]
pub enum Node {
    /// Узел, представляющий целочисленный литерал
    Number {
        token: Token,
        value: i64,
    },
    /// Узел, представляющий переменную
    Variable {
        token: Token,
        name: String,
    },
    /// Узел, представляющий бинарную операцию (+, -, *, /)
    BinOp {
        left: Box<Node>,
        op: Token,
        right: Box<Node>,
    },
    /// Узел, представляющий унарную операцию (+ или -)
    UnaryOp {
        op: Token,
        expr: Box<Node>,
    },
    /// Узел, представляющий присваивание переменной
    Assignment {
        var: Box<Node>, // Variable
        expr: Box<Node>,
    },
    /// Узел, представляющий составной оператор BEGIN ... END
    CompoundStatement {
        statements: Vec<Option<Box<Node>>>,
    },
    /// Узел, представляющий полную программу на Pascal
    Program {
        compound_statement: Box<Node>, // CompoundStatement
    },
}

impl Node {
    /// Создает узел Number
    pub fn number(token: Token) -> Self {
        let value = token.value.parse::<i64>().expect("Invalid integer");
        Node::Number { token, value }
    }

    /// Создает узел Variable
    pub fn variable(token: Token) -> Self {
        let name = token.value.clone();
        Node::Variable { token, name }
    }

    /// Создает узел BinOp
    pub fn bin_op(left: Node, op: Token, right: Node) -> Self {
        Node::BinOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Создает узел UnaryOp
    pub fn unary_op(op: Token, expr: Node) -> Self {
        Node::UnaryOp {
            op,
            expr: Box::new(expr),
        }
    }

    /// Создает узел Assignment
    pub fn assignment(var: Node, expr: Node) -> Self {
        Node::Assignment {
            var: Box::new(var),
            expr: Box::new(expr),
        }
    }

    /// Создает узел CompoundStatement
    pub fn compound_statement(statements: Vec<Option<Box<Node>>>) -> Self {
        Node::CompoundStatement { statements }
    }

    /// Создает узел Program
    pub fn program(compound_statement: Node) -> Self {
        Node::Program {
            compound_statement: Box::new(compound_statement),
        }
    }
}

