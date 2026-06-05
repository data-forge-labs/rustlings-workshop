use rust_testing;

#[test]
fn test_add_integration() {
    assert_eq!(rust_testing::add(100, 200), 300);
}

#[test]
fn test_fibonacci_integration() {
    assert_eq!(rust_testing::fibonacci(10), 55);
}

#[test]
fn test_validate_email_integration() {
    assert!(rust_testing::validate_email("test@example.com"));
    assert!(!rust_testing::validate_email("bad"));
}
