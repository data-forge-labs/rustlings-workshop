/// Add two numbers (for basic unit test demo)
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divide, returning Result (for error-handling test demo)
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Find item in slice (for testing Option return)
pub fn find_item<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    slice.iter().position(|x| x == target)
}

/// Fibonacci (for property-based testing demo)
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = b;
                b = a + b;
                a = temp;
            }
            b
        }
    }
}

/// Validate email format (for parameterized testing demo)
pub fn validate_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    let at_pos = match email.find('@') {
        Some(p) => p,
        None => return false,
    };
    let local = &email[..at_pos];
    let domain = &email[at_pos + 1..];
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

/// Describe different test types and their Python equivalents
pub fn test_types() -> Vec<(&'static str, &'static str)> {
    vec![
        ("#[test]", "pytest"),
        ("#[should_panic]", "pytest.raises"),
        ("assert_eq!", "assert =="),
        ("Result<(), E> in test", "pytest + raises"),
    ]
}

#[cfg(test)]
mod tests {
    mod step_01_basic_tests {
        use crate::add;
        use crate::divide;

        #[test]
        fn test_add_normal() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_add_negative() {
            assert_eq!(add(-1, 1), 0);
        }

        #[test]
        fn test_add_zero() {
            assert_eq!(add(0, 0), 0);
        }

        #[test]
        #[should_panic(expected = "overflow")]
        fn test_add_overflow() {
            // i32::MAX + 1 should panic in debug mode
            let _ = add(i32::MAX, 1);
        }

        #[test]
        fn test_divide_normal() {
            let result = divide(10.0, 2.0).unwrap();
            assert_eq!(result, 5.0);
        }

        #[test]
        fn test_divide_by_zero() {
            let result = divide(1.0, 0.0);
            assert!(result.is_err());
        }
    }

    mod step_02_option_result_tests {
        use crate::{divide, find_item};

        #[test]
        fn test_find_item_found() {
            let v = vec![1, 2, 3, 4, 5];
            assert_eq!(find_item(&v, &3), Some(2));
        }

        #[test]
        fn test_find_item_not_found() {
            let v = vec![1, 2, 3];
            assert_eq!(find_item(&v, &99), None);
        }

        #[test]
        fn test_find_item_empty_slice() {
            let v: Vec<i32> = vec![];
            assert_eq!(find_item(&v, &1), None);
        }

        #[test]
        fn test_divide_roundtrip() {
            let result = divide(divide(100.0, 5.0).unwrap(), 2.0);
            assert!(result.is_ok());
            assert!((result.unwrap() - 10.0).abs() < 1e-10);
        }
    }

    mod step_03_property_tests {
        use crate::{fibonacci, validate_email};

        #[test]
        fn test_fibonacci_0() {
            assert_eq!(fibonacci(0), 0);
        }

        #[test]
        fn test_fibonacci_1() {
            assert_eq!(fibonacci(1), 1);
        }

        #[test]
        fn test_fibonacci_2() {
            assert_eq!(fibonacci(2), 1);
        }

        #[test]
        fn test_fibonacci_10() {
            assert_eq!(fibonacci(10), 55);
        }

        #[test]
        fn test_validate_email_valid() {
            assert!(validate_email("user@example.com"));
            assert!(validate_email("a.b@c.co"));
        }

        #[test]
        fn test_validate_email_invalid() {
            assert!(!validate_email("not-an-email"));
            assert!(!validate_email("@missing-local.com"));
            assert!(!validate_email("missing-at"));
        }

        #[test]
        fn test_validate_email_empty() {
            assert!(!validate_email(""));
        }
    }

    mod step_04_test_types {
        use crate::test_types;

        #[test]
        fn test_test_types_non_empty() {
            let types = test_types();
            assert!(!types.is_empty());
        }

        #[test]
        fn test_test_types_maps_pytest() {
            let types = test_types();
            let found = types.iter().any(|(rust, py)| {
                rust.contains("test") && py.contains("pytest")
            });
            assert!(
                found,
                "Should map Rust #[test] to Python pytest"
            );
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_on_oob() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    #[test]
    fn test_divide_result() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        assert_eq!(result, 5.0);
        Ok(())
    }
}
