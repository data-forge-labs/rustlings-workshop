use tokio::sync::{mpsc, Notify, Semaphore};
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

pub async fn race_two(fast: Duration, slow: Duration) -> &'static str {
    todo!()
}

pub async fn with_timeout<F>(fut: F, limit: Duration) -> Result<F::Output, Duration>
where
    F: std::future::Future,
{
    todo!()
}

pub async fn acquire_permits(sem: &Semaphore, n: u32) -> Vec<tokio::sync::OwnedSemaphorePermit> {
    todo!()
}

pub async fn run_with_concurrency_limit<F, T>(items: Vec<T>, limit: usize, op: F) -> Vec<T>
where
    F: Fn(T) -> futures::future::BoxFuture<'static, T> + Send + Sync + 'static,
    T: Send + 'static,
{
    todo!()
}

pub async fn notify_one(notify: &Notify) {
    todo!()
}

pub async fn wait_for(notify: &Notify) {
    todo!()
}

pub async fn joinset_spawn_all<F, T>(count: usize, f: F) -> Vec<T>
where
    F: Fn(usize) -> futures::future::BoxFuture<'static, T> + Send + Sync + 'static,
    T: Send + 'static,
{
    todo!()
}

pub async fn joinset_first_error<F, T>(count: usize, f: F) -> Result<(), &'static str>
where
    F: Fn(usize) -> futures::future::BoxFuture<'static, Result<T, &'static str>> + Send + Sync + 'static,
    T: Send + 'static,
{
    todo!()
}

pub async fn bounded_send_n(tx: mpsc::Sender<i32>, n: i32) -> Result<(), mpsc::error::SendError<i32>> {
    todo!()
}

pub async fn bounded_drain(mut rx: mpsc::Receiver<i32>, n: usize) -> Vec<i32> {
    todo!()
}

pub fn is_cancelled(token: &CancellationToken) -> bool {
    todo!()
}

pub async fn cancel_after(token: CancellationToken, delay: Duration) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    mod step_01_select {
        use super::*;

        #[tokio::test]
        async fn test_race_two_fast_wins() {
            let result = race_two(Duration::from_millis(10), Duration::from_millis(500)).await;
            assert_eq!(result, "fast");
        }

        #[tokio::test]
        async fn test_with_timeout_succeeds() {
            let fut = async { 42 };
            let r = with_timeout(fut, Duration::from_millis(100)).await;
            assert_eq!(r, Ok(42));
        }

        #[tokio::test]
        async fn test_with_timeout_expires() {
            let fut = async {
                tokio::time::sleep(Duration::from_millis(500)).await;
                99
            };
            let r = with_timeout(fut, Duration::from_millis(50)).await;
            assert!(r.is_err());
        }
    }

    mod step_02_semaphore {
        use super::*;

        #[tokio::test]
        async fn test_acquire_permits() {
            let sem = Semaphore::new(3);
            let p = acquire_permits(&sem, 2).await;
            assert_eq!(p.len(), 2);
            assert_eq!(sem.available_permits(), 1);
        }

        #[tokio::test]
        async fn test_concurrency_limit() {
            let active = Arc::new(AtomicUsize::new(0));
            let max = Arc::new(AtomicUsize::new(0));
            let items: Vec<usize> = (0..20).collect();
            let max_for_op = Arc::clone(&max);
            let _ = run_with_concurrency_limit(items, 4, move |i| {
                let active = Arc::clone(&active);
                let max = Arc::clone(&max_for_op);
                Box::pin(async move {
                    let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                    max.fetch_max(now, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    active.fetch_sub(1, Ordering::SeqCst);
                    i
                })
            })
            .await;
            assert!(max.load(Ordering::SeqCst) <= 4);
        }
    }

    mod step_03_notify {
        use super::*;

        #[tokio::test]
        async fn test_notify_one_wakes_one() {
            let n = Arc::new(Notify::new());
            let n1 = Arc::clone(&n);
            let n2 = Arc::clone(&n);
            let h1 = tokio::spawn(async move { wait_for(&n1).await; 1 });
            let h2 = tokio::spawn(async move { wait_for(&n2).await; 2 });
            tokio::time::sleep(Duration::from_millis(20)).await;
            notify_one(&n).await;
            let r1 = tokio::time::timeout(Duration::from_millis(200), h1).await.unwrap().unwrap();
            let r2 = tokio::time::timeout(Duration::from_millis(50), h2).await;
            assert_eq!(r1, 1);
            assert!(r2.is_err(), "second waiter should still be sleeping");
        }
    }

    mod step_04_joinset {
        use super::*;

        #[tokio::test]
        async fn test_joinset_spawn_all() {
            let results = joinset_spawn_all(5, |i| Box::pin(async move { i * 2 })).await;
            assert_eq!(results, vec![0, 2, 4, 6, 8]);
        }

        #[tokio::test]
        async fn test_joinset_first_error() {
            let r = joinset_first_error(4, |i| {
                Box::pin(async move {
                    if i == 2 { Err("boom") } else { Ok(i) }
                })
            })
            .await;
            assert!(r.is_err());
        }
    }

    mod step_05_bounded {
        use super::*;

        #[tokio::test]
        async fn test_bounded_send_n() {
            let (tx, mut rx) = mpsc::channel::<i32>(3);
            let tx2 = tx.clone();
            drop(tx);
            bounded_send_n(tx2, 5).await.unwrap();
            let mut out = vec![];
            while let Some(v) = rx.recv().await {
                out.push(v);
            }
            assert_eq!(out, vec![1, 2, 3, 4, 5]);
        }

        #[tokio::test]
        async fn test_bounded_drain() {
            let (tx, rx) = mpsc::channel::<i32>(4);
            for i in 0..4 {
                tx.send(i).await.unwrap();
            }
            drop(tx);
            let out = bounded_drain(rx, 4).await;
            assert_eq!(out, vec![0, 1, 2, 3]);
        }
    }

    mod step_06_cancel {
        use super::*;

        #[tokio::test]
        async fn test_is_cancelled() {
            let t = CancellationToken::new();
            assert!(!is_cancelled(&t));
            t.cancel();
            assert!(is_cancelled(&t));
        }

        #[tokio::test]
        async fn test_cancel_after_propagates() {
            let t = CancellationToken::new();
            let t2 = t.clone();
            let worker = tokio::spawn(async move {
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(10)) => "timeout",
                    _ = t2.cancelled() => "cancelled",
                }
            });
            cancel_after(t, Duration::from_millis(20)).await;
            let r = tokio::time::timeout(Duration::from_millis(200), worker).await.unwrap().unwrap();
            assert_eq!(r, "cancelled");
        }
    }
}
