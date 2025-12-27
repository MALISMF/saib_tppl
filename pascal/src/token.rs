/// Типы лексем (токенов) языка Pascal
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Ключевые слова
    Begin,  // BEGIN
    End,    // END
    
    // Идентификаторы и литералы
    Id,      // Идентификатор (имя переменной)
    Integer, // Целое число
    
    // Операторы
    Assign, // := (присваивание)
    Plus,   // + (сложение)
    Minus,  // - (вычитание)
    Mul,    // * (умножение)
    Div,    // / (деление)
    
    // Разделители
    LParen, // ( (левая скобка)
    RParen, // ) (правая скобка)
    Semi,   // ; (точка с запятой)
    Dot,    // . (точка)
    
    // Специальные
    Eol,    // Конец входного потока
}

/// Класс, представляющий лексему (токен)
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {:?})", self.token_type, self.value)
    }
}


