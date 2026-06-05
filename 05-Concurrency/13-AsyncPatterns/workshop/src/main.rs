use async_patterns_workshop::{
    acquire_permits, bounded_drain, bounded_send_n, cancel_after, is_cancelled,
    joinset_spawn_all, race_two, run_with_concurrency_limit, with_timeout,
};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, Notify, Semaphore};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("race_two: {}", race_two(Duration::from_millis(10), Duration::from_millis(500)).await);

    let r = with_timeout(
        async { tokio::time::sleep(Duration::from_millis(10)).await; "done" },
        Duration::from_millis(50),
    )
    .await;
    println!("with_timeout: {:?}", r);

    let sem = Semaphore::new(2);
    let _p = acquire_permits(&sem, 2).await;
    println!("permits left: {}", sem.available_permits());

    let out = joinset_spawn_all(3, |i| Box::pin(async move { i + 100 })).await;
    println!("joinset: {:?}", out);

    let (tx, rx) = mpsc::channel::<i32>(4);
    tokio::spawn(async move {
        let _ = bounded_send_n(tx, 5).await;
    });
    println!("drained: {:?}", bounded_drain(rx, 5).await);

    let n = Arc::new(Notify::new());
    let n2 = Arc::clone(&n);
    tokio::spawn(async move {
        n2.notify_one();
    });
    n.notified().await;
    println!("notify: ok");

    let items: Vec<u32> = (0..10).collect();
    let _ = run_with_concurrency_limit(items, 3, |i| {
        Box::pin(async move {
            tokio::time::sleep(Duration::from_millis(5)).await;
            i * 10
        })
    })
    .await;

    let token = CancellationToken::new();
    let token2 = token.clone();
    tokio::spawn(async move {
        cancel_after(token2, Duration::from_millis(20)).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;
    println!("cancelled: {}", is_cancelled(&token));

    Ok(())
}
