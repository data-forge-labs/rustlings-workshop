/// Add two numbers (safe function)
pub fn safe_add(a: i32, b: i32) -> i32 { a + b }

/// Dereference a raw pointer (requires unsafe)
/// Returns the value at the pointer, or 0 if null
pub unsafe fn unsafe_dereference(ptr: *const i32) -> i32 {
    if ptr.is_null() {
        0
    } else {
        unsafe { *ptr }
    }
}

/// Create a mutable raw pointer and write to it (unsafe)
pub unsafe fn unsafe_write(ptr: *mut i32, val: i32) {
    unsafe { *ptr = val };
}

/// Safe wrapper that demonstrates split_at_mut (safe interior mutability pattern)
pub fn safe_split_sum(slice: &mut [i32]) -> (i32, i32) {
    let mid = slice.len() / 2;
    let (left, right) = slice.split_at_mut(mid);
    let sum_left: i32 = left.iter().sum();
    let sum_right: i32 = right.iter().sum();
    (sum_left, sum_right)
}

/// Demonstrate that safe Rust prevents buffer overflow
pub fn safe_index(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}

/// Return which safety concepts this project covers
pub fn safety_concepts() -> Vec<&'static str> {
    vec![
        "safe functions",
        "unsafe functions",
        "raw pointers",
        "null pointer safety",
        "split_at_mut",
        "bounds checking",
    ]
}

#[cfg(test)]
mod tests {
    mod step_01_safe_functions {
        use super::super::*;

        #[test]
        fn test_safe_add_normal() {
            assert_eq!(safe_add(3, 5), 8);
        }

        #[test]
        fn test_safe_add_negative() {
            assert_eq!(safe_add(-4, 7), 3);
        }

        #[test]
        fn test_safe_add_zero() {
            assert_eq!(safe_add(0, 0), 0);
        }

        #[test]
        fn test_safe_index_valid() {
            let v = [10, 20, 30];
            assert_eq!(safe_index(&v, 1), Some(20));
        }

        #[test]
        fn test_safe_index_out_of_bounds() {
            let v = [10, 20, 30];
            assert_eq!(safe_index(&v, 10), None);
        }
    }

    mod step_02_unsafe_functions {
        use super::super::*;

        #[test]
        fn test_unsafe_dereference_valid() {
            let val = 42;
            let ptr: *const i32 = &val;
            let result = unsafe { unsafe_dereference(ptr) };
            assert_eq!(result, 42);
        }

        #[test]
        fn test_unsafe_dereference_null() {
            let ptr = std::ptr::null::<i32>();
            let result = unsafe { unsafe_dereference(ptr) };
            assert_eq!(result, 0);
        }

        #[test]
        fn test_unsafe_write() {
            let mut val = 0;
            let ptr: *mut i32 = &mut val;
            unsafe { unsafe_write(ptr, 99) };
            assert_eq!(val, 99);
        }
    }

    mod step_03_memory_safety {
        use super::super::*;

        #[test]
        fn test_safe_split_sum_even() {
            let mut arr = [1, 2, 3, 4];
            let (a, b) = safe_split_sum(&mut arr);
            assert_eq!(a, 3);
            assert_eq!(b, 7);
        }

        #[test]
        fn test_safe_split_sum_odd() {
            let mut arr = [10, 20, 30];
            let (a, b) = safe_split_sum(&mut arr);
            assert_eq!(a, 10);
            assert_eq!(b, 50);
        }

        #[test]
        fn test_safe_split_sum_empty() {
            let mut arr: [i32; 0] = [];
            let (a, b) = safe_split_sum(&mut arr);
            assert_eq!(a, 0);
            assert_eq!(b, 0);
        }
    }

    mod step_04_concepts {
        use super::super::*;

        #[test]
        fn test_safety_concepts_non_empty() {
            let concepts = safety_concepts();
            assert!(!concepts.is_empty(), "must list at least one concept");
        }

        #[test]
        fn test_safety_concepts_contains_keywords() {
            let concepts = safety_concepts();
            let joined = concepts.join(" ");
            assert!(
                joined.contains("unsafe") || joined.contains("safe") || joined.contains("raw pointer"),
                "concepts should mention key safety topics: got {:?}",
                concepts,
            );
        }
    }
}
