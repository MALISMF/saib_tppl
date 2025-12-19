use pascal_interpreter::Interpreter;
use std::collections::HashMap;

#[test]
fn test_empty_program() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result, HashMap::new());
}

#[test]
fn test_arithmetic_expressions() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x:= 2 + 3 * (2 + 3);
        y:= 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1))
    END.
    "#;
    let result = interpreter.interpret(code);
    
    // x = 2 + 3 * (2 + 3) = 2 + 3 * 5 = 2 + 15 = 17
    assert_eq!(result.get("x"), Some(&17.0));
    
    // y = 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1))
    //   = 1 - 2 + 3 * (2 + 2)
    //   = 1 - 2 + 3 * 4
    //   = 1 - 2 + 12
    //   = 11
    assert_eq!(result.get("y"), Some(&11.0));
}

#[test]
fn test_nested_blocks() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        y: = 2;
        BEGIN
            a := 3;
            a := a;
            b := 10 + a + 10 * y / 4;
            c := a - b
        END;
        x := 11
    END.
    "#;
    let result = interpreter.interpret(code);
    
    // y = 2
    assert_eq!(result.get("y"), Some(&2.0));
    
    // a = 3, then a := a = 3
    assert_eq!(result.get("a"), Some(&3.0));
    
    // b = 10 + a + 10 * y / 4 = 10 + 3 + 10 * 2 / 4 = 10 + 3 + 5 = 18
    assert_eq!(result.get("b"), Some(&18.0));
    
    // c = a - b = 3 - 18 = -15
    assert_eq!(result.get("c"), Some(&-15.0));
    
    // x = 11
    assert_eq!(result.get("x"), Some(&11.0));
}

#[test]
fn test_simple_assignment() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 5;
        y := 10
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&5.0));
    assert_eq!(result.get("y"), Some(&10.0));
}

#[test]
fn test_addition() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 3 + 4
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&7.0));
}

#[test]
fn test_subtraction() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 10 - 3
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&7.0));
}

#[test]
fn test_multiplication() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 4 * 5
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&20.0));
}

#[test]
fn test_division() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 20 / 4
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&5.0));
}

#[test]
fn test_operator_precedence() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 2 + 3 * 4;
        y := (2 + 3) * 4
    END.
    "#;
    let result = interpreter.interpret(code);
    // x = 2 + 3 * 4 = 2 + 12 = 14
    assert_eq!(result.get("x"), Some(&14.0));
    // y = (2 + 3) * 4 = 5 * 4 = 20
    assert_eq!(result.get("y"), Some(&20.0));
}

#[test]
fn test_unary_operators() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := -5;
        y := +10;
        z := -(2 + 3)
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&-5.0));
    assert_eq!(result.get("y"), Some(&10.0));
    assert_eq!(result.get("z"), Some(&-5.0));
}

#[test]
fn test_variable_reuse() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 5;
        y := x + 3;
        z := x * y
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&5.0));
    assert_eq!(result.get("y"), Some(&8.0));
    assert_eq!(result.get("z"), Some(&40.0));
}

#[test]
fn test_multiple_statements() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        a := 1;
        b := 2;
        c := 3;
        d := a + b + c
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("a"), Some(&1.0));
    assert_eq!(result.get("b"), Some(&2.0));
    assert_eq!(result.get("c"), Some(&3.0));
    assert_eq!(result.get("d"), Some(&6.0));
}

#[test]
fn test_complex_nested_expressions() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := ((((2 + 3) * 4) - 5) / 3)
    END.
    "#;
    let result = interpreter.interpret(code);
    // x = ((((2 + 3) * 4) - 5) / 3)
    //   = (((5 * 4) - 5) / 3)
    //   = ((20 - 5) / 3)
    //   = (15 / 3)
    //   = 5
    assert_eq!(result.get("x"), Some(&5.0));
}

#[test]
fn test_mixed_operations() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 10;
        y := x + 5 * 2 - 3 / 3;
        z := y * 2
    END.
    "#;
    let result = interpreter.interpret(code);
    // y = 10 + 5 * 2 - 3 / 3 = 10 + 10 - 1 = 19
    assert_eq!(result.get("y"), Some(&19.0));
    // z = 19 * 2 = 38
    assert_eq!(result.get("z"), Some(&38.0));
}

#[test]
fn test_floating_point() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 3.5;
        y := 2.0;
        z := x + y
    END.
    "#;
    let result = interpreter.interpret(code);
    assert_eq!(result.get("x"), Some(&3.5));
    assert_eq!(result.get("y"), Some(&2.0));
    assert_eq!(result.get("z"), Some(&5.5));
}

#[test]
fn test_variable_shadowing_in_nested_blocks() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 1;
        BEGIN
            x := 2;
            y := 3
        END;
        z := x
    END.
    "#;
    let result = interpreter.interpret(code);
    // x should be overwritten to 2 in nested block
    assert_eq!(result.get("x"), Some(&2.0));
    assert_eq!(result.get("y"), Some(&3.0));
    // z should use the updated x value
    assert_eq!(result.get("z"), Some(&2.0));
}

#[test]
#[should_panic(expected = "Undefined variable")]
fn test_error_undefined_variable() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := y + 1
    END.
    "#;
    interpreter.interpret(code);
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_error_division_by_zero() {
    let mut interpreter = Interpreter::new();
    let code = r#"
    BEGIN
        x := 10 / 0
    END.
    "#;
    interpreter.interpret(code);
}

