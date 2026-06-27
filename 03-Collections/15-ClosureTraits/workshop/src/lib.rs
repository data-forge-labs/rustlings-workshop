// Step 1: The three closure traits
// ================================

/// Accepts a closure that can only be called once (FnOnce).
/// The closure takes ownership of any captured values.
pub fn apply_fn_once<F: FnOnce() -> String>(f: F) -> String {
    f()
}

pub fn apply_fn_mut<F: FnMut(i32) -> i32>(f: &mut F, val: i32) -> i32 {
    f(val)
}

pub fn apply_fn<F: Fn(i32) -> bool>(f: &F, val: i32) -> bool {
    f(val)
}

// Step 2: Closures returning closures
// =====================================

/// Returns a closure that adds `n` to its argument.
/// The returned closure captures `n` by value.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Step 3: FnMut with sort_by
// ===========================

/// Sort a slice of (name, score) tuples by score descending using sort_by with a closure.
pub fn sort_by_score(data: &mut Vec<(&str, i32)>) {
    data.sort_by(|a, b| b.1.cmp(&a.1));
}

// Step 4: Custom Iterator — Counter
// ==================================

/// A counter from 1 to max (inclusive).
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Step 5: Custom Iterator — Fibonacci
// =====================================

/// Yields Fibonacci numbers: 0, 1, 1, 2, 3, 5, 8, ...
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(result)
    }
}

// Step 6: Chaining closures + iterators in a data pipeline
// ==========================================================

/// Given a list of numbers, return only even numbers, square them, and sum the results.
/// Demonstrates chaining closures with iterator adapters.
pub fn run_pipeline(data: &[i32]) -> Vec<i32> {
    data.iter().copied().filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ====== Step 1: Closure traits ======

    mod step_01_closure_traits {
        use super::*;

        #[test]
        fn test_apply_fn_once() {
            let result = apply_fn_once(|| String::from("hello"));
            assert_eq!(result, "hello");
        }

        #[test]
        fn test_apply_fn_once_with_capture() {
            let name = String::from("world");
            let result = apply_fn_once(|| format!("hello, {}", name));
            assert_eq!(result, "hello, world");
        }

        #[test]
        fn test_apply_fn_mut() {
            let mut counter = 0;
            let result = apply_fn_mut(&mut |x| { counter += 1; x * 2 }, 5);
            assert_eq!(result, 10);
            assert_eq!(counter, 1);
        }

        #[test]
        fn test_apply_fn_mut_multiple() {
            let mut counter = 0;
            let mut f = |x| { counter += 1; x + counter };
            assert_eq!(apply_fn_mut(&mut f, 10), 11);
            assert_eq!(apply_fn_mut(&mut f, 10), 12);
        }

        #[test]
        fn test_apply_fn() {
            let f = |x| x > 10;
            assert!(apply_fn(&f, 15));
            assert!(!apply_fn(&f, 5));
        }

        #[test]
        fn test_apply_fn_borrows() {
            let threshold = 100;
            let f = |x| x > threshold;
            assert!(apply_fn(&f, 200));
            assert!(!apply_fn(&f, 50));
        }
    }

    // ====== Step 2: Closures returning closures ======

    mod step_02_closure_returning_closure {
        use super::*;

        #[test]
        fn test_make_adder() {
            let add_five = make_adder(5);
            assert_eq!(add_five(3), 8);
            assert_eq!(add_five(10), 15);
        }

        #[test]
        fn test_make_adder_different_values() {
            let add_zero = make_adder(0);
            let add_ten = make_adder(10);
            assert_eq!(add_zero(42), 42);
            assert_eq!(add_ten(42), 52);
        }
    }

    // ====== Step 3: FnMut with sort_by ======

    mod step_03_fnmut_sort {
        use super::*;

        #[test]
        fn test_sort_by_score() {
            let mut data = vec![("Alice", 90), ("Bob", 70), ("Carol", 85)];
            sort_by_score(&mut data);
            assert_eq!(data, vec![("Alice", 90), ("Carol", 85), ("Bob", 70)]);
        }

        #[test]
        fn test_sort_by_score_already_sorted() {
            let mut data = vec![("A", 100), ("B", 50), ("C", 25)];
            sort_by_score(&mut data);
            assert_eq!(data, vec![("A", 100), ("B", 50), ("C", 25)]);
        }
    }

    // ====== Step 4: Custom Iterator — Counter ======

    mod step_04_custom_iterator_counter {
        use super::*;

        #[test]
        fn test_counter_len() {
            assert_eq!(Counter::new(5).count(), 5);
        }

        #[test]
        fn test_counter_values() {
            let vals: Vec<u32> = Counter::new(3).collect();
            assert_eq!(vals, vec![1, 2, 3]);
        }

        #[test]
        fn test_counter_with_map() {
            let doubled: Vec<u32> = Counter::new(4).map(|x| x * 2).collect();
            assert_eq!(doubled, vec![2, 4, 6, 8]);
        }
    }

    // ====== Step 5: Custom Iterator — Fibonacci ======

    mod step_05_custom_iterator_fibonacci {
        use super::*;

        #[test]
        fn test_fibonacci_first_5() {
            let fib: Vec<u64> = Fibonacci::new().take(5).collect();
            assert_eq!(fib, vec![0, 1, 1, 2, 3]);
        }

        #[test]
        fn test_fibonacci_first_10() {
            let fib: Vec<u64> = Fibonacci::new().take(10).collect();
            assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        }

        #[test]
        fn test_fibonacci_fold() {
            let sum: u64 = Fibonacci::new().take(7).sum();
            assert_eq!(sum, 20); // 0+1+1+2+3+5+8 = 20
        }
    }

    // ====== Step 6: Data pipeline ======

    mod step_06_data_pipeline {
        use super::*;

        #[test]
        fn test_pipeline_filters_squares_sums() {
            let result = run_pipeline(&[1, 2, 3, 4, 5]);
            assert_eq!(result, vec![4, 16]); // 2²=4, 4²=16
        }

        #[test]
        fn test_pipeline_no_evens() {
            let result = run_pipeline(&[1, 3, 5, 7]);
            assert_eq!(result, vec![]);
        }
    }
}
