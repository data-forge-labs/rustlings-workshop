// ============================================================
// 8-Futures — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

use std::time::Duration;

/// An async function that returns a greeting.
/// README §1: async fn — lazy futures
pub async fn async_hello() -> String {
    String::from("Hello from async!")
}

/// Chain two async operations sequentially.
/// README §1: .await — driving futures to completion
pub async fn process_chain() -> String {
    let step1 = async { String::from("Step 1") }.await;
    let step2 = async { String::from(" + Step 2") }.await;
    format!("{step1}{step2}")
}

/// Run two async tasks concurrently and collect results.
/// README §2: tokio::spawn — concurrent tasks
pub async fn run_concurrent() -> Vec<String> {
    let h1 = tokio::spawn(async { String::from("task1") });
    let h2 = tokio::spawn(async { String::from("task2") });
    let r1 = h1.await.unwrap();
    let r2 = h2.await.unwrap();
    vec![r1, r2]
}

/// Block on an async function using a runtime.
/// README §3: Runtime — blocking on async code
pub fn block_on_hello() -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_hello())
}

/// Simulate a delay using tokio::time::sleep.
/// README §5: Blocking vs async I/O
pub async fn delayed_greeting(seconds: u64) -> String {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    String::from("Hello after delay!")
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_async_fn {
        use super::*;

        #[tokio::test]
        async fn test_async_hello() {
            let result = async_hello().await;
            assert!(!result.is_empty(), "Should return a greeting");
        }

        #[tokio::test]
        async fn test_process_chain() {
            let result = process_chain().await;
            assert!(!result.is_empty());
        }
    }

    mod step_02_spawn {
        use super::*;

        #[tokio::test]
        async fn test_run_concurrent() {
            let results = run_concurrent().await;
            assert_eq!(results.len(), 2, "Two concurrent tasks");
        }
    }

    mod step_03_runtime {
        use super::*;

        #[test]
        fn test_block_on_hello() {
            let result = block_on_hello();
            assert!(!result.is_empty());
        }
    }

    mod step_04_delay {
        use super::*;

        #[tokio::test]
        async fn test_delayed_greeting() {
            let result = delayed_greeting(0).await;
            assert!(!result.is_empty());
        }
    }
}
