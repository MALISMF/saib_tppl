/// Интерпретатор для упрощенной версии языка Pascal
pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;

pub use interpreter::PascalInterpreter;


