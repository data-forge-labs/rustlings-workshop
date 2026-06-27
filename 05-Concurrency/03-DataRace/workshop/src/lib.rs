use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;

pub fn greet_thread() -> String {
    let handle = thread::spawn(|| {
        String::from("Hello from thread!")
    });
    handle.join().unwrap()
}

pub fn sum_in_parallel(data: Vec<i32>) -> i32 {
    if data.is_empty() {
        return 0;
    }
    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();
    let t1 = thread::spawn(move || left.into_iter().sum::<i32>());
    let t2 = thread::spawn(move || right.into_iter().sum::<i32>());
    t1.join().unwrap() + t2.join().unwrap()
}

pub fn move_closure_example(data: Vec<i32>) -> Vec<i32> {
    let handle = thread::spawn(move || {
        data.into_iter().map(|x| x * 2).collect()
    });
    handle.join().unwrap()
}

pub fn shared_counter_arc_mutex(ops: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = vec![];
    for _ in 0..ops {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
}

pub fn shared_counter_rwlock(ops: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));
    let mut handles = vec![];
    for _ in 0..ops {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut val = counter.write().unwrap();
            *val += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    *counter.read().unwrap()
}

pub fn condvar_coordinate() -> String {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    handle.join().unwrap();
    String::from("done")
}

#[cfg(test)]
mod tests {
    use crate::*;

    mod step_01_threads {
        use crate::*;

        #[test]
        fn test_greet_returns_string() {
            let result = greet_thread();
            assert!(!result.is_empty());
        }

        #[test]
        fn test_greet_contains_greeting() {
            let result = greet_thread();
            assert!(
                result.contains("Hello")
                    || result.contains("hello")
                    || result.contains("Hi")
                    || result.contains("hi")
            );
        }

        #[test]
        fn test_sum_positive_numbers() {
            assert_eq!(sum_in_parallel(vec![1, 2, 3, 4, 5]), 15);
        }

        #[test]
        fn test_sum_empty() {
            assert_eq!(sum_in_parallel(vec![]), 0);
        }

        #[test]
        fn test_sum_large() {
            let data: Vec<i32> = (1..=100).collect();
            assert_eq!(sum_in_parallel(data), 5050);
        }
    }

    mod step_02_move_closures {
        use crate::*;

        #[test]
        fn test_double_values() {
            let result = move_closure_example(vec![1, 2, 3]);
            assert_eq!(result, vec![2, 4, 6]);
        }

        #[test]
        fn test_empty() {
            let result = move_closure_example(vec![]);
            assert!(result.is_empty());
        }

        #[test]
        fn test_single_element() {
            let result = move_closure_example(vec![5]);
            assert_eq!(result, vec![10]);
        }
    }

    mod step_03_arc_mutex {
        use crate::*;

        #[test]
        fn test_counter_ten() {
            assert_eq!(shared_counter_arc_mutex(10), 10);
        }

        #[test]
        fn test_counter_zero() {
            assert_eq!(shared_counter_arc_mutex(0), 0);
        }

        #[test]
        fn test_counter_hundred() {
            assert_eq!(shared_counter_arc_mutex(100), 100);
        }
    }

    mod step_04_rwlock {
        use crate::*;

        #[test]
        fn test_rwlock_counter_ten() {
            assert_eq!(shared_counter_rwlock(10), 10);
        }

        #[test]
        fn test_rwlock_counter_zero() {
            assert_eq!(shared_counter_rwlock(0), 0);
        }

        #[test]
        fn test_rwlock_counter_hundred() {
            assert_eq!(shared_counter_rwlock(100), 100);
        }
    }

    mod step_05_condvar {
        use crate::*;

        #[test]
        fn test_condvar_returns_string() {
            let result = condvar_coordinate();
            assert!(!result.is_empty());
        }

        #[test]
        fn test_condvar_contains_status() {
            let result = condvar_coordinate();
            assert!(
                result.contains("done")
                    || result.contains("complete")
                    || result.contains("finished")
                    || result.contains("success")
                    || result.contains("Success")
            );
        }

        #[test]
        fn test_condvar_idempotent() {
            let r1 = condvar_coordinate();
            let r2 = condvar_coordinate();
            assert_eq!(r1, r2);
        }
    }
}
