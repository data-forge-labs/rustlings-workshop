use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_counter(ops: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ops {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    counter.load(Ordering::Relaxed)
}

pub fn atomic_flag_toggle() -> bool {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);
    let handle = thread::spawn(move || {
        flag_clone.store(true, Ordering::Relaxed);
    });
    handle.join().unwrap();
    flag.load(Ordering::Relaxed)
}

pub fn relaxed_ordering_demo() -> (usize, usize) {
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                a.fetch_add(1, Ordering::Relaxed);
                b.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    (a.load(Ordering::Relaxed), b.load(Ordering::Relaxed))
}

pub fn acquire_release_demo() -> (usize, usize) {
    let data = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let data_clone = Arc::clone(&data);
    let flag_clone = Arc::clone(&flag);
    let handle = thread::spawn(move || {
        data_clone.store(42, Ordering::Release);
        flag_clone.store(true, Ordering::Release);
    });
    while !flag.load(Ordering::Acquire) {
        std::hint::spin_loop();
    }
    let val = data.load(Ordering::Acquire);
    handle.join().unwrap();
    (val, flag.load(Ordering::Relaxed) as usize)
}

pub fn fetch_add_demo(values: Vec<usize>) -> usize {
    let sum = Arc::new(AtomicUsize::new(0));
    let num_threads = 4;
    let chunk_size = (values.len() + num_threads - 1) / num_threads;
    let mut handles = vec![];
    for chunk in values.chunks(chunk_size) {
        let sum = Arc::clone(&sum);
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            for v in chunk {
                sum.fetch_add(v, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    sum.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    mod step_01_atomic_types {
        use crate::{atomic_counter, atomic_flag_toggle};

        #[test]
        fn test_atomic_counter_normal() {
            let result = atomic_counter(100);
            assert_eq!(result, 400);
        }

        #[test]
        fn test_atomic_counter_zero_ops() {
            let result = atomic_counter(0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_atomic_flag_toggles_true() {
            let result = atomic_flag_toggle();
            assert!(result);
        }

        #[test]
        fn test_atomic_flag_toggle_repeated() {
            let r1 = atomic_flag_toggle();
            let r2 = atomic_flag_toggle();
            assert!(r1);
            assert!(r2);
        }
    }

    mod step_02_memory_ordering {
        use crate::{acquire_release_demo, relaxed_ordering_demo};

        #[test]
        fn test_relaxed_ordering_both_equal() {
            let (a, b) = relaxed_ordering_demo();
            assert_eq!(a, b);
            assert_eq!(a, 40);
        }

        #[test]
        fn test_relaxed_ordering_non_zero() {
            let (a, b) = relaxed_ordering_demo();
            assert!(a > 0);
            assert!(b > 0);
        }

        #[test]
        fn test_acquire_release_data_visible() {
            let (val, _) = acquire_release_demo();
            assert_eq!(val, 42);
        }

        #[test]
        fn test_acquire_release_flag_set() {
            let (_, flag) = acquire_release_demo();
            assert_eq!(flag, 1);
        }
    }

    mod step_03_atomic_operations {
        use crate::fetch_add_demo;

        #[test]
        fn test_fetch_add_sum() {
            let result = fetch_add_demo(vec![1, 2, 3, 4, 5]);
            assert_eq!(result, 15);
        }

        #[test]
        fn test_fetch_add_empty() {
            let result = fetch_add_demo(vec![]);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_fetch_add_large() {
            let values: Vec<usize> = (1..=1000).collect();
            let expected: usize = (1..=1000).sum();
            let result = fetch_add_demo(values);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_fetch_add_single_element() {
            let result = fetch_add_demo(vec![42]);
            assert_eq!(result, 42);
        }
    }
}
