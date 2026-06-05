use diy_actor_workshop::{ask_value, send_decrement, send_increment, spawn_counter, stop_actor};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let actor = spawn_counter(8);

    for _ in 0..10 {
        send_increment(&actor, 1).await?;
    }
    send_decrement(&actor, 3).await?;
    println!("value = {}", ask_value(&actor).await?);

    stop_actor(actor).await?;
    Ok(())
}
