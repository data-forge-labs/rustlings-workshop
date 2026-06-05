use ractor_workshop::{
    call_get_value, cast_decrement, cast_increment, spawn_counter, stop_counter, CounterMsg,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let actor = spawn_counter().await?;

    for _ in 0..10 {
        cast_increment(&actor, 1)?;
    }
    cast_decrement(&actor, 3)?;
    println!("value = {}", call_get_value(&actor).await?);

    actor.cast(CounterMsg::Reset)?;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    println!("after reset = {}", call_get_value(&actor).await?);

    stop_counter(actor).await?;
    Ok(())
}
