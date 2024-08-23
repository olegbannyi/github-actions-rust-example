#[test]
fn test_add() {
    assert_eq!(github_actions_rust_example::add(1, 2), 3);
}

#[test]
fn test_subtract() {
    assert_eq!(github_actions_rust_example::subtract(2, 1), 1);
}

#[test]
fn test_multiply() {
    assert_eq!(github_actions_rust_example::multiply(2, 3), 6);
}

#[test]
fn test_divide() {
    assert_eq!(github_actions_rust_example::divide(6, 3), 2);
}
