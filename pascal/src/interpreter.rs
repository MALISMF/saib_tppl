use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::Node;
use crate::token::TokenType;

/// Интерпретатор для упрощенной версии языка Pascal
pub struct PascalInterpreter {
    variables: HashMap<String, f64>,
}

impl PascalInterpreter {
    /// Создает новый интерпретатор
    pub fn new() -> Self {
        PascalInterpreter {
            variables: HashMap::new(),
        }
    }

    /// Интерпретирует программу на Pascal и возвращает словарь всех переменных
    pub fn interpret(&mut self, text: &str) -> HashMap<String, f64> {
        self.variables.clear(); // Сброс переменных для каждой интерпретации
        let lexer = Lexer::new();
        let mut parser = Parser::new(lexer);
        let tree = parser.parse(text); // Парсинг программы в AST
        self.visit(&tree); // Выполнение программы
        self.variables.clone() // Возврат копии словаря переменных
    }

    /// Диспетчер паттерна Visitor - вызывает соответствующий метод для узла
    fn visit(&mut self, node: &Node) -> f64 {
        match node {
            Node::Program { compound_statement } => {
                self.visit(compound_statement) // Обрабатываем составной оператор
            }
            Node::CompoundStatement { statements } => {
                // Обработка составного оператора BEGIN ... END - выполняем все операторы
                for statement in statements {
                    if let Some(ref stmt) = statement {
                        self.visit(stmt);
                    }
                }
                0.0 // Возвращаем 0 для составного оператора
            }
            Node::Assignment { var, expr } => {
                // Обработка присваивания - вычисляем значение и сохраняем в переменную
                let var_name = match **var {
                    Node::Variable { ref name, .. } => name.clone(),
                    _ => panic!("Expected variable in assignment"),
                };
                let value = self.visit(expr); // Вычисляем значение выражения
                self.variables.insert(var_name.clone(), value); // Сохраняем значение переменной
                value
            }
            Node::Number { value, .. } => {
                // Обработка числового литерала - возвращаем его значение
                *value as f64
            }
            Node::Variable { name, .. } => {
                // Обработка переменной - возвращаем её значение из словаря
                self.variables
                    .get(name)
                    .copied()
                    .unwrap_or_else(|| panic!("Variable '{}' is not defined", name))
            }
            Node::BinOp { left, op, right } => {
                // Обработка бинарной операции - вычисляем левую и правую части, применяем операцию
                let left_val = self.visit(left); // Вычисляем левый операнд
                let right_val = self.visit(right); // Вычисляем правый операнд

                // Выполняем операцию в зависимости от типа оператора
                match op.token_type {
                    TokenType::Plus => left_val + right_val,
                    TokenType::Minus => left_val - right_val,
                    TokenType::Mul => left_val * right_val,
                    TokenType::Div => {
                        if right_val == 0.0 {
                            panic!("Division by zero");
                        }
                        left_val / right_val
                    }
                    _ => panic!("Unknown operator: {:?}", op.token_type),
                }
            }
            Node::UnaryOp { op, expr } => {
                // Обработка унарной операции (+ или -)
                let value = self.visit(expr); // Вычисляем значение выражения

                match op.token_type {
                    TokenType::Plus => value,
                    TokenType::Minus => -value,
                    _ => panic!("Unknown unary operator: {:?}", op.token_type),
                }
            }
        }
    }
}

impl Default for PascalInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

