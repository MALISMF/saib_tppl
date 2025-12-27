use pascal_interpreter::PascalInterpreter;

fn main() {
    // Тест 1: Пустой блок BEGIN END
    println!("Тест 1:");
    let mut interpreter = PascalInterpreter::new();
    let code1 = "
BEGIN
END.
";
    let result1 = interpreter.interpret(code1);
    println!("Результат: {:?}\n", result1);

    // Тест 2: Простые арифметические выражения
    println!("Тест 2:");
    let mut interpreter = PascalInterpreter::new();
    let code2 = "
BEGIN
    x := 2 + 3 * (2 + 3);
    y := 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1));
END.
";
    let result2 = interpreter.interpret(code2);
    println!("Результат: {:?}\n", result2);

    // Тест 3: Вложенные блоки BEGIN END
    println!("Тест 3:");
    let mut interpreter = PascalInterpreter::new();
    let code3 = "
BEGIN
    y := 2;
    BEGIN
        a := 3;
        a := a;
        b := 10 + a + 10 * y / 4;
        c := a - b
    END;
    x := 11;
END.
";
    let result3 = interpreter.interpret(code3);
    println!("Результат: {:?}", result3);
}
