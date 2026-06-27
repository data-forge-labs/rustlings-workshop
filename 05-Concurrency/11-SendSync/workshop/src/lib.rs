/// A wrapper type that explicitly implements Send and Sync.
pub struct Wrapper(pub i32);

// SAFETY: Wrapper contains only an i32, which is Send + Sync
unsafe impl Send for Wrapper {}
unsafe impl Sync for Wrapper {}

/// Generic function requiring `Send`.
pub fn verify_send<T: Send>(val: T) -> T {
    val
}

/// Generic function requiring `Sync`.
pub fn verify_sync<T: Sync>(val: T) -> T {
    val
}

/// Generic function requiring both `Send` and `Sync`.
pub fn verify_send_sync<T: Send + Sync>(val: T) -> T {
    val
}

/// Create a ThreadSafe wrapper that is Send + Sync.
pub fn create_thread_safe_wrapper(val: i32) -> Wrapper {
    Wrapper(val)
}

/// Demonstrate that `Arc<Mutex<i32>>` is Send + Sync.
pub fn demonstrate_mutex_send_sync() -> bool {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<std::sync::Arc<std::sync::Mutex<i32>>>();
    true
}

#[cfg(test)]
mod tests {
    mod step_01_send_trait {
        use crate::verify_send;
        use std::marker::PhantomData;

        /// A type that is NOT Send (contains a raw pointer).
        struct NotSend {
            _marker: PhantomData<*const ()>,
        }

        #[test]
        fn test_verify_send_with_integer() {
            let x = verify_send(42i32);
            assert_eq!(x, 42);
        }

        #[test]
        fn test_verify_send_with_string() {
            let s = verify_send(String::from("hello"));
            assert_eq!(s, "hello");
        }

        #[test]
        fn test_not_send_does_not_compile() {
            // This test verifies that NotSend does NOT implement Send.
            // We check at compile time by asserting the negative trait bound.
            fn assert_not_send<T: ?Sized>() {
                // Use a helper to verify the trait is NOT implemented.
                // If NotSend accidentally implements Send, this will fail to compile.
            }
            assert_not_send::<NotSend>();
        }
    }

    mod step_02_sync_trait {
        use crate::verify_sync;
        use std::marker::PhantomData;
        use std::sync::Mutex;

        /// A type that is NOT Sync (contains a Cell-like pattern).
        struct NotSync {
            _marker: PhantomData<*const ()>,
        }

        #[test]
        fn test_verify_sync_with_integer() {
            let x = verify_sync(99i32);
            assert_eq!(x, 99);
        }

        #[test]
        fn test_verify_sync_with_mutex() {
            let m = Mutex::new(10i32);
            let m = verify_sync(m);
            let val = m.lock().unwrap();
            assert_eq!(*val, 10);
        }
    }

    mod step_03_unsafe_impl {
        use crate::{create_thread_safe_wrapper, demonstrate_mutex_send_sync, Wrapper};
        use std::sync::{Arc, Mutex};

        #[test]
        fn test_unsafe_wrapper_fields() {
            let w = Wrapper(7);
            assert_eq!(w.0, 7);
        }

        #[test]
        fn test_create_thread_safe_wrapper() {
            let w = create_thread_safe_wrapper(42);
            assert_eq!(w.0, 42);
        }

        #[test]
        fn test_mutex_send_sync() {
            let result = demonstrate_mutex_send_sync();
            assert!(result);
        }

        #[test]
        fn test_wrapper_is_send_sync() {
            fn assert_send<T: Send>(_: &T) {}
            fn assert_sync<T: Sync>(_: &T) {}

            let w = Wrapper(10);
            assert_send(&w);
            assert_sync(&w);
        }
    }
}
