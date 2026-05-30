// ============================================================
// 7-Threads — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// Spawn a thread that returns a greeting string.
/// README §1: Threads — spawn and join
pub fn spawn_and_join() -> String {
    todo!()
}

/// Sum numbers in a vector by splitting work across two threads.
/// README §1: Threads — data parallelism
pub fn sum_in_parallel(data: Vec<i32>) -> i32 {
    todo!()
}

/// Send messages through an mpsc channel.
/// README §5: Channels — message passing
pub fn channel_send_receive() -> Vec<String> {
    todo!()
}

/// Increment a shared counter using Arc<Mutex<usize>>.
/// README §11: Locks — Mutex
pub fn shared_counter(ops: usize) -> usize {
    todo!()
}

/// Demonstrate a shared counter using Arc<RwLock<usize>>.
/// README §12: RwLock
pub fn rwlock_counter(ops: usize) -> usize {
    todo!()
}

/// Run a closure on a scoped thread.
/// README §4: Scoped threads
pub fn scoped_worker(data: Vec<i32>) -> Vec<i32> {
    todo!()
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_threads {
        use super::*;

        #[test]
        fn test_spawn_and_join() {
            let result = spawn_and_join();
            assert!(!result.is_empty(), "Should return a greeting");
        }

        #[test]
        fn test_sum_in_parallel() {
            let data = vec![1, 2, 3, 4, 5, 6];
            let sum = sum_in_parallel(data);
            assert_eq!(sum, 21);
        }

        #[test]
        fn test_sum_in_parallel_empty() {
            let sum = sum_in_parallel(vec![]);
            assert_eq!(sum, 0);
        }

        #[test]
        fn test_sum_in_parallel_single() {
            let sum = sum_in_parallel(vec![42]);
            assert_eq!(sum, 42);
        }
    }

    mod step_02_scoped_threads {
        use super::*;

        #[test]
        fn test_scoped_worker() {
            let data = vec![1, 2, 3];
            let result = scoped_worker(data);
            assert_eq!(result.len(), 3);
        }
    }

    mod step_03_channels {
        use super::*;

        #[test]
        fn test_channel_send_receive() {
            let msgs = channel_send_receive();
            assert!(!msgs.is_empty(), "Should receive at least one message");
        }
    }

    mod step_04_locks {
        use super::*;

        #[test]
        fn test_shared_counter() {
            let count = shared_counter(100);
            assert_eq!(count, 100);
        }

        #[test]
        fn test_shared_counter_zero() {
            let count = shared_counter(0);
            assert_eq!(count, 0);
        }

        #[test]
        fn test_rwlock_counter() {
            let count = rwlock_counter(50);
            assert_eq!(count, 50);
        }
    }
}
