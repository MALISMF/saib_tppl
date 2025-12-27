use pascal_interpreter::PascalInterpreter;
use std::collections::HashMap;

/// Тесты для интерпретатора Pascal

#[test]
fn test_empty_program() {
    // Тест 1: Пустой блок BEGIN END
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result, HashMap::new());
}

#[test]
fn test_simple_expressions() {
    // Тест 2: Простые арифметические выражения
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 2 + 3 * (2 + 3);
    y := 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1));
END.
";
    let result = interpreter.interpret(code);
    // x = 2 + 3 * (2 + 3) = 2 + 3 * 5 = 2 + 15 = 17
    assert_eq!(result["x"], 17.0);
    // y = 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1)) = 1 - 2 + 3 * (2 + 2) = -1 + 3 * 4 = -1 + 12 = 11
    assert_eq!(result["y"], 11.0);
}

#[test]
fn test_nested_blocks() {
    // Тест 3: Вложенные блоки BEGIN END
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    y: = 2;
    BEGIN
        a := 3;
        a := a;
        b := 10 + a + 10 * y / 4;
        c := a - b
    END;
    x := 11;
END.
";
    let result = interpreter.interpret(code);
    // y = 2
    assert_eq!(result["y"], 2.0);
    // a = 3 (then a := a, so still 3)
    assert_eq!(result["a"], 3.0);
    // b = 10 + a + 10 * y / 4 = 10 + 3 + 10 * 2 / 4 = 10 + 3 + 20 / 4 = 10 + 3 + 5 = 18
    assert_eq!(result["b"], 18.0);
    // c = a - b = 3 - 18 = -15
    assert_eq!(result["c"], -15.0);
    // x = 11
    assert_eq!(result["x"], 11.0);
}

#[test]
fn test_single_assignment() {
    // Тест: одиночное присваивание переменной
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 42;
END.
";
    let result = interpreter.interpret(code);
    let mut expected = HashMap::new();
    expected.insert("x".to_string(), 42.0);
    assert_eq!(result, expected);
}

#[test]
fn test_multiple_assignments() {
    // Тест: несколько присваиваний подряд
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    a := 1;
    b := 2;
    c := 3;
END.
";
    let result = interpreter.interpret(code);
    let mut expected = HashMap::new();
    expected.insert("a".to_string(), 1.0);
    expected.insert("b".to_string(), 2.0);
    expected.insert("c".to_string(), 3.0);
    assert_eq!(result, expected);
}

#[test]
fn test_arithmetic_operations() {
    // Тест: все арифметические операции
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    add := 5 + 3;
    sub := 5 - 3;
    mul := 5 * 3;
    div := 10 / 2;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["add"], 8.0);
    assert_eq!(result["sub"], 2.0);
    assert_eq!(result["mul"], 15.0);
    assert_eq!(result["div"], 5.0);
}

#[test]
fn test_parentheses() {
    // Тест: выражение со скобками
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    result := (2 + 3) * 4;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["result"], 20.0);
}

#[test]
fn test_variable_in_expression() {
    // Тест: использование переменных в выражениях
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 5;
    y := x + 3;
    z := x * y;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["x"], 5.0);
    assert_eq!(result["y"], 8.0);
    assert_eq!(result["z"], 40.0);
}

#[test]
fn test_nested_expressions() {
    // Тест: глубоко вложенные выражения
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 2 + 3 * (4 + 5 * (6 + 7));
END.
";
    let result = interpreter.interpret(code);
    // x = 2 + 3 * (4 + 5 * 13) = 2 + 3 * (4 + 65) = 2 + 3 * 69 = 2 + 207 = 209
    assert_eq!(result["x"], 209.0);
}

#[test]
fn test_unary_operators() {
    // Тест: унарные операторы плюс и минус
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    pos := +5;
    neg := -5;
    complex := +10 - -5;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["pos"], 5.0);
    assert_eq!(result["neg"], -5.0);
    assert_eq!(result["complex"], 15.0);
}

#[test]
fn test_division() {
    // Тест: операции деления
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    div1 := 10 / 2;
    div2 := 15 / 4;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["div1"], 5.0);
    assert_eq!(result["div2"], 3.75);
}

#[test]
fn test_reassignment() {
    // Тест: повторное присваивание переменной
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 1;
    x := 2;
    x := x + 1;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["x"], 3.0);
}

#[test]
fn test_complex_nested_blocks() {
    // Тест: несколько уровней вложенных блоков
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    outer := 1;
    BEGIN
        inner1 := 2;
        BEGIN
            inner2 := 3;
        END;
    END;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["outer"], 1.0);
    assert_eq!(result["inner1"], 2.0);
    assert_eq!(result["inner2"], 3.0);
}

#[test]
fn test_empty_statements() {
    // Тест: программа с пустыми операторами (завершающие точки с запятой)
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 1;
    ;
    y := 2;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["x"], 1.0);
    assert_eq!(result["y"], 2.0);
}

#[test]
#[should_panic(expected = "Variable 'y' is not defined")]
fn test_undefined_variable_error() {
    // Тест: использование неопределенной переменной вызывает ошибку
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := y + 1;
END.
";
    interpreter.interpret(code);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero() {
    // Тест: деление на ноль вызывает ошибку
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    x := 10 / 0;
END.
";
    interpreter.interpret(code);
}

#[test]
fn test_case_insensitive_keywords() {
    // Тест: ключевые слова не чувствительны к регистру
    let mut interpreter = PascalInterpreter::new();
    let code = "
begin
    x := 1;
end.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["x"], 1.0);
}

#[test]
fn test_variable_names() {
    // Тест: различные допустимые имена переменных
    let mut interpreter = PascalInterpreter::new();
    let code = "
BEGIN
    var1 := 1;
    my_var := 2;
    x123 := 3;
END.
";
    let result = interpreter.interpret(code);
    assert_eq!(result["var1"], 1.0);
    assert_eq!(result["my_var"], 2.0);
    assert_eq!(result["x123"], 3.0);
}


