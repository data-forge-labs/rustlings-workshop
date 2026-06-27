use std::cell::{Cell, RefCell};

pub fn cell_counter(ops: usize) -> usize {
    let counter = Cell::new(0usize);
    for _ in 0..ops {
        counter.set(counter.get() + 1);
    }
    counter.get()
}

pub fn cell_string(initial: &str, append: &str) -> String {
    let cell = Cell::new(String::from(initial));
    let mut val = cell.into_inner();
    val.push_str(append);
    val
}

pub fn refcell_demo(values: Vec<i32>) -> Vec<i32> {
    let r = RefCell::new(values);
    let mut borrowed = r.borrow_mut();
    for v in borrowed.iter_mut() {
        *v *= 2;
    }
    borrowed.clone()
}

pub fn refcell_borrow_error() -> Result<String, String> {
    let r = RefCell::new(String::from("hello"));
    let _borrow1 = r.borrow();
    match r.try_borrow_mut() {
        Ok(_) => Err(String::from("should have failed")),
        Err(e) => Err(format!("borrow error: {e}")),
    }
}

pub fn simulate_race_condition() -> usize {
    use std::cell::UnsafeCell;
    use std::sync::Arc;
    use std::thread;

    struct FakeCounter(UnsafeCell<usize>);
    unsafe impl Send for FakeCounter {}
    unsafe impl Sync for FakeCounter {}

    let counter = Arc::new(FakeCounter(UnsafeCell::new(0)));
    let mut handles = vec![];
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    let ptr = counter.0.get();
                    let val = std::ptr::read(ptr);
                    thread::yield_now();
                    std::ptr::write(ptr, val + 1);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    unsafe { *counter.0.get() }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_cell {
        use super::super::*;

        #[test]
        fn test_cell_counter_basic() {
            assert_eq!(cell_counter(5), 5);
        }

        #[test]
        fn test_cell_counter_zero() {
            assert_eq!(cell_counter(0), 0);
        }

        #[test]
        fn test_cell_counter_large() {
            assert_eq!(cell_counter(10_000), 10_000);
        }

        #[test]
        fn test_cell_string_basic() {
            assert_eq!(cell_string("hello", " world"), "hello world");
        }

        #[test]
        fn test_cell_string_empty_initial() {
            assert_eq!(cell_string("", "test"), "test");
        }

        #[test]
        fn test_cell_string_empty_append() {
            assert_eq!(cell_string("hello", ""), "hello");
        }
    }

    mod step_02_refcell {
        use super::super::*;

        #[test]
        fn test_refcell_demo_basic() {
            let result = refcell_demo(vec![1, 2, 3]);
            assert_eq!(result, vec![2, 4, 6]);
        }

        #[test]
        fn test_refcell_demo_empty() {
            let result: Vec<i32> = refcell_demo(vec![]);
            assert!(result.is_empty());
        }

        #[test]
        fn test_refcell_demo_single() {
            assert_eq!(refcell_demo(vec![5]), vec![10]);
        }

        #[test]
        fn test_refcell_borrow_error_violation() {
            let result = refcell_borrow_error();
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.contains("borrow"));
        }
    }

    mod step_03_race_conditions {
        use super::super::*;

        #[test]
        fn test_simulate_race_condition_lost_updates() {
            let result = simulate_race_condition();
            let expected = 8 * 1000;
            assert!(
                result < expected,
                "expected race condition to cause lost updates (got {result}, expected < {expected})"
            );
        }

        #[test]
        fn test_simulate_race_condition_non_zero() {
            let result = simulate_race_condition();
            assert!(result > 0, "at least some increments should succeed");
        }
    }
}
