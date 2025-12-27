use crate::token::{Token, TokenType};

/// Лексический анализатор (токенизатор) для языка Pascal
pub struct Lexer {
    text: Vec<char>,      // Исходный текст программы как вектор символов
    pos: usize,          // Текущая позиция в тексте
    current_char: Option<char>, // Текущий символ
}

impl Lexer {
    /// Создает новый лексический анализатор
    pub fn new() -> Self {
        Lexer {
            text: Vec::new(),
            pos: 0,
            current_char: None,
        }
    }

    /// Устанавливает текст для анализа
    pub fn set_text(&mut self, text: &str) {
        self.text = text.chars().collect();
        self.pos = 0;
        self.current_char = if !self.text.is_empty() {
            Some(self.text[self.pos])
        } else {
            None
        };
    }

    /// Переход к следующему символу
    fn forward(&mut self) {
        self.pos += 1;
        if self.pos >= self.text.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text[self.pos]);
        }
    }

    /// Пропуск пробельных символов
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.forward();
            } else {
                break;
            }
        }
    }

    /// Чтение целого числа
    fn read_integer(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                result.push(ch);
                self.forward();
            } else {
                break;
            }
        }
        result
    }

    /// Чтение идентификатора (имени переменной или ключевого слова)
    fn read_id(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.forward();
            } else {
                break;
            }
        }
        result
    }

    /// Просмотр символа впереди без перемещения позиции
    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.pos + offset;
        if pos >= self.text.len() {
            None
        } else {
            Some(self.text[pos])
        }
    }

    /// Получение следующей лексемы (токена) из входного текста
    pub fn next_token(&mut self) -> Token {
        loop {
            let ch = match self.current_char {
                Some(c) => c,
                None => {
                    return Token::new(TokenType::Eol, String::new());
                }
            };

            // Пропускаем пробельные символы
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            // Ключевые слова и идентификаторы
            if ch.is_alphabetic() || ch == '_' {
                let id_value = self.read_id();
                // Проверяем, является ли это ключевым словом
                match id_value.to_uppercase().as_str() {
                    "BEGIN" => return Token::new(TokenType::Begin, id_value),
                    "END" => return Token::new(TokenType::End, id_value),
                    _ => return Token::new(TokenType::Id, id_value),
                }
            }

            // Числа
            if ch.is_ascii_digit() {
                return Token::new(TokenType::Integer, self.read_integer());
            }

            // Операторы
            if ch == ':' {
                // Ищем '=' после ':' (возможно с пробелами между ними)
                let mut peek_pos = 1;
                while let Some(peek_ch) = self.peek(peek_pos) {
                    if peek_ch.is_whitespace() {
                        peek_pos += 1;
                    } else {
                        break;
                    }
                }
                if let Some('=') = self.peek(peek_pos) {
                    self.forward(); // Потребляем ':'
                    // Пропускаем пробелы
                    while let Some(c) = self.current_char {
                        if c.is_whitespace() {
                            self.forward();
                        } else {
                            break;
                        }
                    }
                    self.forward(); // Потребляем '='
                    return Token::new(TokenType::Assign, ":=".to_string());
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }

            if ch == '+' {
                self.forward();
                return Token::new(TokenType::Plus, ch.to_string());
            }

            if ch == '-' {
                self.forward();
                return Token::new(TokenType::Minus, ch.to_string());
            }

            if ch == '*' {
                self.forward();
                return Token::new(TokenType::Mul, ch.to_string());
            }

            if ch == '/' {
                self.forward();
                return Token::new(TokenType::Div, ch.to_string());
            }

            // Разделители
            if ch == '(' {
                self.forward();
                return Token::new(TokenType::LParen, ch.to_string());
            }

            if ch == ')' {
                self.forward();
                return Token::new(TokenType::RParen, ch.to_string());
            }

            if ch == ';' {
                self.forward();
                return Token::new(TokenType::Semi, ch.to_string());
            }

            if ch == '.' {
                self.forward();
                return Token::new(TokenType::Dot, ch.to_string());
            }

            panic!("Unexpected character: {:?}", ch);
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}


