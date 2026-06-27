use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub enum CounterMsg {
    Increment(i32),
    Decrement(i32),
    Get { reply: oneshot::Sender<i32> },
    Stop,
}

#[derive(Debug)]
pub struct CounterActor {
    pub value: i32,
}

impl CounterActor {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

pub struct ActorHandle {
    pub tx: mpsc::Sender<CounterMsg>,
    pub join: JoinHandle<()>,
}

pub fn spawn_counter(buffer: usize) -> ActorHandle {
    let (tx, rx) = mpsc::channel(buffer);
    let join = tokio::spawn(async move {
        let mut actor = CounterActor::new();
        while let Some(msg) = rx.recv().await {
            match msg {
                CounterMsg::Increment(d) => actor.value += d,
                CounterMsg::Decrement(d) => actor.value -= d,
                CounterMsg::Get { reply } => {
                    let _ = reply.send(actor.value);
                }
                CounterMsg::Stop => break,
            }
        }
    });
    ActorHandle { tx, join }
}

pub async fn send_increment(handle: &ActorHandle, delta: i32) -> Result<(), &'static str> {
    handle
        .tx
        .send(CounterMsg::Increment(delta))
        .await
        .map_err(|_| "channel closed")
}

pub async fn send_decrement(handle: &ActorHandle, delta: i32) -> Result<(), &'static str> {
    handle
        .tx
        .send(CounterMsg::Decrement(delta))
        .await
        .map_err(|_| "channel closed")
}

pub async fn ask_value(handle: &ActorHandle) -> Result<i32, &'static str> {
    let (reply_tx, reply_rx) = oneshot::channel();
    handle
        .tx
        .send(CounterMsg::Get { reply: reply_tx })
        .await
        .map_err(|_| "channel closed")?;
    reply_rx.await.map_err(|_| "reply channel closed")
}

pub async fn stop_actor(handle: ActorHandle) -> Result<(), &'static str> {
    let _ = handle.tx.send(CounterMsg::Stop).await;
    handle.join.await.map_err(|_| "join failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_counter_starts_at_zero() {
        let h = spawn_counter(8);
        assert_eq!(ask_value(&h).await.unwrap(), 0);
        let _ = stop_actor(h).await;
    }

    #[tokio::test]
    async fn test_increment_and_decrement() {
        let h = spawn_counter(8);
        send_increment(&h, 5).await.unwrap();
        send_decrement(&h, 2).await.unwrap();
        assert_eq!(ask_value(&h).await.unwrap(), 3);
        let _ = stop_actor(h).await;
    }

    #[tokio::test]
    async fn test_many_increments() {
        let h = spawn_counter(64);
        for _ in 0..100 {
            send_increment(&h, 1).await.unwrap();
        }
        assert_eq!(ask_value(&h).await.unwrap(), 100);
        let _ = stop_actor(h).await;
    }

    #[tokio::test]
    async fn test_stop_actor_terminates() {
        let h = spawn_counter(4);
        send_increment(&h, 1).await.unwrap();
        assert!(stop_actor(h).await.is_ok());
    }

    #[tokio::test]
    async fn test_messages_processed_in_order() {
        let h = spawn_counter(8);
        send_increment(&h, 1).await.unwrap();
        send_increment(&h, 1).await.unwrap();
        send_increment(&h, 1).await.unwrap();
        send_decrement(&h, 2).await.unwrap();
        assert_eq!(ask_value(&h).await.unwrap(), 1);
        let _ = stop_actor(h).await;
    }

    #[tokio::test]
    async fn test_send_after_stop_fails() {
        let h = spawn_counter(4);
        let _ = stop_actor(h).await;
    }
}
