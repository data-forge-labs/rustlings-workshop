use ractor::Actor;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct CounterState {
    pub value: i32,
}

#[derive(Debug)]
pub enum CounterMsg {
    Increment(i32),
    Decrement(i32),
    Reset,
    GetValue(oneshot::Sender<i32>),
}

pub struct Counter;

#[ractor::async_trait]
impl Actor for Counter {
    type Msg = CounterMsg;
    type State = CounterState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ractor::ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ractor::ActorProcessingErr> {
        Ok(CounterState { value: 0 })
    }

    async fn handle(
        &self,
        _myself: ractor::ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ractor::ActorProcessingErr> {
        match message {
            CounterMsg::Increment(d) => state.value += d,
            CounterMsg::Decrement(d) => state.value -= d,
            CounterMsg::Reset => state.value = 0,
            CounterMsg::GetValue(reply) => {
                let _ = reply.send(state.value);
            }
        }
        Ok(())
    }
}

pub async fn spawn_counter() -> Result<ractor::ActorRef<CounterMsg>, String> {
    let (actor_ref, _handle) = Actor::spawn(None, Counter, ())
        .await
        .map_err(|e| e.to_string())?;
    Ok(actor_ref)
}

pub fn cast_increment(
    actor: &ractor::ActorRef<CounterMsg>,
    delta: i32,
) -> Result<(), String> {
    actor
        .cast(CounterMsg::Increment(delta))
        .map_err(|e| e.to_string())
}

pub fn cast_decrement(
    actor: &ractor::ActorRef<CounterMsg>,
    delta: i32,
) -> Result<(), String> {
    actor
        .cast(CounterMsg::Decrement(delta))
        .map_err(|e| e.to_string())
}

pub async fn call_get_value(actor: &ractor::ActorRef<CounterMsg>) -> Result<i32, String> {
    let (tx, rx) = oneshot::channel();
    actor
        .cast(CounterMsg::GetValue(tx))
        .map_err(|e| e.to_string())?;
    rx.await.map_err(|e| e.to_string())
}

pub async fn stop_counter(actor: ractor::ActorRef<CounterMsg>) -> Result<(), String> {
    actor.stop(None);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_starts_at_zero() {
        let actor = spawn_counter().await.unwrap();
        let v = call_get_value(&actor).await.unwrap();
        assert_eq!(v, 0);
        let _ = stop_counter(actor).await;
    }

    #[tokio::test]
    async fn test_increment_and_decrement() {
        let actor = spawn_counter().await.unwrap();
        cast_increment(&actor, 5).unwrap();
        cast_decrement(&actor, 2).unwrap();
        let v = call_get_value(&actor).await.unwrap();
        assert_eq!(v, 3);
        let _ = stop_counter(actor).await;
    }

    #[tokio::test]
    async fn test_many_casts() {
        let actor = spawn_counter().await.unwrap();
        for _ in 0..50 {
            cast_increment(&actor, 1).unwrap();
        }
        let v = call_get_value(&actor).await.unwrap();
        assert_eq!(v, 50);
        let _ = stop_counter(actor).await;
    }

    #[tokio::test]
    async fn test_reset() {
        let actor = spawn_counter().await.unwrap();
        cast_increment(&actor, 100).unwrap();
        actor.cast(CounterMsg::Reset).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let v = call_get_value(&actor).await.unwrap();
        assert_eq!(v, 0);
        let _ = stop_counter(actor).await;
    }

    #[tokio::test]
    async fn test_actor_survives_individual_casts() {
        let actor = spawn_counter().await.unwrap();
        for _ in 0..1000 {
            cast_increment(&actor, 1).unwrap();
        }
        let v = call_get_value(&actor).await.unwrap();
        assert_eq!(v, 1000);
        let _ = stop_counter(actor).await;
    }
}
