use ractor::Actor;

#[derive(Debug, Clone)]
pub struct CounterState {
    pub value: i32,
}

#[derive(Debug, Clone)]
pub enum CounterMsg {
    Increment(i32),
    Decrement(i32),
    Reset,
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
        todo!()
    }

    async fn handle(
        &self,
        _myself: ractor::ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ractor::ActorProcessingErr> {
        todo!()
    }
}

pub async fn spawn_counter() -> Result<ractor::ActorRef<CounterMsg>, String> {
    todo!()
}

pub fn cast_increment(
    actor: &ractor::ActorRef<CounterMsg>,
    delta: i32,
) -> Result<(), String> {
    todo!()
}

pub fn cast_decrement(
    actor: &ractor::ActorRef<CounterMsg>,
    delta: i32,
) -> Result<(), String> {
    todo!()
}

pub async fn call_get_value(actor: &ractor::ActorRef<CounterMsg>) -> Result<i32, String> {
    todo!()
}

pub async fn stop_counter(actor: ractor::ActorRef<CounterMsg>) -> Result<(), String> {
    todo!()
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
