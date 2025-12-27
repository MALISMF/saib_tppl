use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use crate::ast::Node;

/// Синтаксический анализатор (парсер) для языка Pascal
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    /// Создает новый парсер
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            current_token: None,
        }
    }

    /// Выброс ошибки синтаксиса
    fn error(&self, message: &str) -> ! {
        panic!("{} Got {:?}", message, self.current_token);
    }

    /// Потребление лексемы ожидаемого типа
    fn eat(&mut self, token_type: TokenType) {
        match &self.current_token {
            Some(token) if token.token_type == token_type => {
                self.current_token = Some(self.lexer.next_token());
            }
            Some(token) => {
                panic!(
                    "Expected {:?}, got {:?}",
                    token_type, token.token_type
                );
            }
            None => {
                panic!("Expected {:?}, got None", token_type);
            }
        }
    }

    /// Парсинг программы на Pascal
    pub fn parse(&mut self, text: &str) -> Node {
        self.lexer.set_text(text);
        self.current_token = Some(self.lexer.next_token());

        // program ::= complex_statement dot
        let compound = self.complex_statement(); // Парсим составной оператор
        self.eat(TokenType::Dot); // Ожидаем точку в конце программы

        Node::program(compound)
    }

    /// complex_statement ::= BEGIN statement_list END
    fn complex_statement(&mut self) -> Node {
        self.eat(TokenType::Begin); // Потребляем BEGIN
        let statements = self.statement_list(); // Парсим список операторов
        self.eat(TokenType::End); // Потребляем END
        Node::compound_statement(statements)
    }

    /// statement_list ::= statement | statement SEMI statement_list
    fn statement_list(&mut self) -> Vec<Option<Box<Node>>> {
        let mut statements = Vec::new();
        
        // Парсим первый оператор (может быть пустым)
        let first_stmt = self.statement();
        if let Some(stmt) = first_stmt {
            statements.push(Some(Box::new(stmt)));
        } else {
            statements.push(None);
        }

        // Обрабатываем несколько операторов, разделенных точкой с запятой
        while let Some(ref token) = self.current_token {
            if token.token_type == TokenType::Semi {
                self.eat(TokenType::Semi); // Потребляем точку с запятой
                // Проверяем, есть ли еще оператор (не END)
                if let Some(ref next_token) = self.current_token {
                    if next_token.token_type != TokenType::End {
                        let stmt = self.statement();
                        if let Some(s) = stmt {
                            statements.push(Some(Box::new(s)));
                        } else {
                            statements.push(None);
                        }
                    } else {
                        // Завершающая точка с запятой перед END
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        statements
    }

    /// statement ::= compound_statement | assignment | empty
    fn statement(&mut self) -> Option<Node> {
        match &self.current_token {
            Some(token) if token.token_type == TokenType::Begin => {
                Some(self.complex_statement()) // Вложенный составной оператор
            }
            Some(token) if token.token_type == TokenType::Id => {
                Some(self.assignment()) // Присваивание
            }
            Some(token) if matches!(
                token.token_type,
                TokenType::End | TokenType::Semi | TokenType::Dot | TokenType::Eol
            ) => {
                // Пустой оператор
                None
            }
            _ => {
                self.error("Expected statement");
            }
        }
    }

    /// assignment ::= variable ASSIGN expr
    fn assignment(&mut self) -> Node {
        let var = self.variable(); // Парсим переменную
        self.eat(TokenType::Assign); // Потребляем :=
        let expr = self.expr(); // Парсим выражение
        Node::assignment(var, expr)
    }

    /// variable ::= ID
    fn variable(&mut self) -> Node {
        let token = self.current_token.clone().expect("Expected ID token");
        self.eat(TokenType::Id); // Потребляем идентификатор
        Node::variable(token)
    }

    /// expr ::= term (('+' | '-') term)*
    fn expr(&mut self) -> Node {
        let mut result = self.term(); // Парсим первый терм

        // Обрабатываем операции сложения и вычитания (левоассоциативные)
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    let op = token.clone();
                    if token.token_type == TokenType::Plus {
                        self.eat(TokenType::Plus);
                    } else {
                        self.eat(TokenType::Minus);
                    }
                    result = Node::bin_op(result, op, self.term()); // Создаем узел бинарной операции
                }
                _ => break,
            }
        }

        result
    }

    /// term ::= factor (('*' | '/') factor)*
    fn term(&mut self) -> Node {
        let mut result = self.factor(); // Парсим первый множитель

        // Обрабатываем операции умножения и деления (левоассоциативные)
        while let Some(ref token) = self.current_token {
            match token.token_type {
                TokenType::Mul | TokenType::Div => {
                    let op = token.clone();
                    if token.token_type == TokenType::Mul {
                        self.eat(TokenType::Mul);
                    } else {
                        self.eat(TokenType::Div);
                    }
                    result = Node::bin_op(result, op, self.factor()); // Создаем узел бинарной операции
                }
                _ => break,
            }
        }

        result
    }

    /// factor ::= ('+' | '-') factor | INTEGER | LPAREN expr RPAREN | variable
    fn factor(&mut self) -> Node {
        let token = self.current_token.clone().expect("Expected factor");

        // Унарный плюс или минус
        match token.token_type {
            TokenType::Plus => {
                self.eat(TokenType::Plus);
                Node::unary_op(token, self.factor())
            }
            TokenType::Minus => {
                self.eat(TokenType::Minus);
                Node::unary_op(token, self.factor())
            }
            TokenType::Integer => {
                self.eat(TokenType::Integer);
                Node::number(token)
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen);
                let result = self.expr(); // Парсим выражение внутри скобок
                self.eat(TokenType::RParen);
                result
            }
            TokenType::Id => {
                self.variable()
            }
            _ => {
                self.error("Unexpected factor");
            }
        }
    }
}

