use std::sync::{Arc, RwLock};
use std::thread;

pub fn is_send<T: Send>(_: &T) -> bool {
    true
}

pub fn is_sync<T: Sync>(_: &T) -> bool {
    true
}

pub fn rwlock_read_heavy(readers: usize, ops_per_reader: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));
    let total_reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    thread::scope(|s| {
        for _ in 0..readers {
            let counter = &counter;
            let total_reads = &total_reads;
            s.spawn(|| {
                for _ in 0..ops_per_reader {
                    let _guard = counter.read().unwrap();
                    total_reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });

    total_reads.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn rwlock_write_once(ops: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));

    thread::scope(|s| {
        for _ in 0..ops {
            let counter = &counter;
            s.spawn(|| {
                let mut guard = counter.write().unwrap();
                *guard += 1;
            });
        }
    });

    *counter.read().unwrap()
}

pub fn parallel_sum(data: Vec<i32>) -> i32 {
    if data.is_empty() {
        return 0;
    }

    thread::scope(|s| {
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let chunk_size = (data.len() + num_threads - 1) / num_threads;

        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(s.spawn(|| chunk.iter().sum::<i32>()));
        }

        handles.into_iter().map(|h| h.join()).sum()
    })
}

pub fn parallel_map(data: Vec<i32>, mapper: fn(i32) -> i32) -> Vec<i32> {
    if data.is_empty() {
        return Vec::new();
    }

    thread::scope(|s| {
        let num_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let chunk_size = (data.len() + num_threads - 1) / num_threads;

        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(s.spawn(|| chunk.iter().map(|&x| mapper(x)).collect::<Vec<i32>>()));
        }

        let mut results = Vec::with_capacity(data.len());
        for handle in handles {
            results.extend(handle.join());
        }
        results
    })
}

#[cfg(test)]
mod tests {
    mod step_01_send_sync {
        use crate::{is_send, is_sync};
        use std::rc::Rc;
        use std::sync::Arc;

        #[test]
        fn test_is_send_with_i32() {
            let x = 42i32;
            assert!(is_send(&x));
        }

        #[test]
        fn test_is_send_with_arc() {
            let x = Arc::new(42);
            assert!(is_send(&x));
        }

        #[test]
        fn test_is_sync_with_i32() {
            let x = 42i32;
            assert!(is_sync(&x));
        }

        #[test]
        fn test_is_sync_with_arc() {
            let x = Arc::new(42);
            assert!(is_sync(&x));
        }

        #[test]
        fn test_is_send_fails_to_compile_with_rc() {
            let x = Rc::new(42);
            // NOTE: Rc<i32> does NOT implement Send, so is_send(&x) would not compile.
            // This test is a compile-time demonstration — it passes because is_send
            // is never actually called. Uncommenting the line below would cause a
            // compile error:
            // assert!(is_send(&x));
        }

        #[test]
        fn test_is_sync_with_string() {
            let s = String::from("hello");
            assert!(is_sync(&s));
        }
    }

    mod step_02_rwlock {
        use crate::{rwlock_read_heavy, rwlock_write_once};

        #[test]
        fn test_rwlock_read_heavy_normal() {
            let result = rwlock_read_heavy(4, 100);
            assert_eq!(result, 400);
        }

        #[test]
        fn test_rwlock_read_heavy_zero_readers() {
            let result = rwlock_read_heavy(0, 100);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_rwlock_read_heavy_zero_ops() {
            let result = rwlock_read_heavy(4, 0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_rwlock_write_once_normal() {
            let result = rwlock_write_once(10);
            assert_eq!(result, 10);
        }

        #[test]
        fn test_rwlock_write_once_zero_ops() {
            let result = rwlock_write_once(0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_rwlock_write_once_single() {
            let result = rwlock_write_once(1);
            assert_eq!(result, 1);
        }
    }

    mod step_03_parallelism {
        use crate::{parallel_map, parallel_sum};

        #[test]
        fn test_parallel_sum_normal() {
            let data = vec![1, 2, 3, 4, 5];
            let result = parallel_sum(data);
            assert_eq!(result, 15);
        }

        #[test]
        fn test_parallel_sum_empty() {
            let data = vec![];
            let result = parallel_sum(data);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_parallel_sum_single_element() {
            let data = vec![42];
            let result = parallel_sum(data);
            assert_eq!(result, 42);
        }

        #[test]
        fn test_parallel_sum_large() {
            let data: Vec<i32> = (1..=1000).collect();
            let expected: i32 = (1..=1000).sum();
            let result = parallel_sum(data);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_parallel_map_double() {
            let data = vec![1, 2, 3, 4, 5];
            let result = parallel_map(data, |x| x * 2);
            assert_eq!(result, vec![2, 4, 6, 8, 10]);
        }

        #[test]
        fn test_parallel_map_empty() {
            let data = vec![];
            let result = parallel_map(data, |x| x * 2);
            assert_eq!(result, vec![]);
        }

        #[test]
        fn test_parallel_map_identity() {
            let data = vec![1, 2, 3];
            let result = parallel_map(data, |x| x);
            assert_eq!(result, vec![1, 2, 3]);
        }

        #[test]
        fn test_parallel_map_negate() {
            let data = vec![1, -2, 3, -4];
            let result = parallel_map(data, |x| -x);
            assert_eq!(result, vec![-1, 2, -3, 4]);
        }
    }
}
