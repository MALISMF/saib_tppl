use cow::Cow;

#[test]
fn test_increment() {
    let mut cow = Cow::new("MoO".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 1);
}

#[test]
fn test_multiple_increments() {
    let mut cow = Cow::new("MoO MoO MoO MoO MoO".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 5);
}

#[test]
fn test_decrement() {
    let mut cow = Cow::new("MoO MoO MOo".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 1);
}

#[test]
fn test_move_right() {
    let mut cow = Cow::new("MoO moO".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.current, 1);
    assert_eq!(cow.get_val(), 0);
}

#[test]
fn test_move_left() {
    let mut cow = Cow::new("moO mOo".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.current, -1);
}

#[test]
fn test_clear_cell() {
    let mut cow = Cow::new("MoO MoO MoO OOO".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 0);
}

#[test]
fn test_register_store() {
    let mut cow = Cow::new("MoO MoO MoO MMM".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.register, Some(3));
}

#[test]
fn test_register_restore() {
    let mut cow = Cow::new("MoO MoO MoO MMM MOo MOo MMM".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 3);
    assert_eq!(cow.register, None);
}

#[test]
fn test_register_swap() {
    let code = "MoO MoO MoO MMM moO MoO MoO MoO MoO MMM MOo MOo MOo MOo MOo MOo".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 3);
}

#[test]
fn test_simple_loop() {
    let code = "MoO MOO MOo moo".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 0);
}

#[test]
fn test_loop_multiple_iterations() {
    let code = "MoO MoO MoO MOO moO MoO moo".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 0);
}

#[test]
fn test_nested_loops() {
    let code = "MoO MOO MoO MOO MOo moo moo".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 0);
}

#[test]
fn test_error_moo_without_MOO() {
    let result = Cow::new("moo".to_string());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("moo без MOO"));
}

#[test]
fn test_error_MOO_without_moo() {
    let result = Cow::new("MOO".to_string());
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("MOO без moo"));
}

#[test]
fn test_error_unmatched_loops() {
    let result = Cow::new("MOO MoO MOO moo".to_string());
    assert!(result.is_err());
}

#[test]
fn test_multiple_cells() {
    let code = "MoO MoO moO MoO MoO MoO mOo MoO MoO".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.current, 0);
    assert_eq!(cow.cells.get(&1).unwrap_or(&0), &3);
}

#[test]
fn test_cell_persistence() {
    let code = "MoO MoO moO MoO mOo MoO MoO".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.cells.get(&0).unwrap(), &2);
    assert_eq!(cow.cells.get(&1).unwrap(), &1);
}

#[test]
fn test_empty_code() {
    let mut cow = Cow::new("".to_string()).unwrap();
    let result = cow.run();
    assert!(result.is_ok());
}

#[test]
fn test_unknown_command() {
    let mut cow = Cow::new("XYZ MoO".to_string()).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), 1);
}

#[test]
fn test_negative_values() {
    let code = "MOo MOo MOo".to_string();
    let mut cow = Cow::new(code).unwrap();
    cow.run().unwrap();
    assert_eq!(cow.get_val(), -3);
}