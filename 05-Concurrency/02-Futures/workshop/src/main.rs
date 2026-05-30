use futures::{async_hello, process_chain, block_on_hello, delayed_greeting};

#[tokio::main]
async fn main() {
    let hello = async_hello().await;
    println!("{hello}");

    let chain = process_chain().await;
    println!("{chain}");

    let blocked = block_on_hello();
    println!("{blocked}");

    let delayed = delayed_greeting(1).await;
    println!("{delayed}");
}
