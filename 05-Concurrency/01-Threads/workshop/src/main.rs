use threads::{spawn_and_join, sum_in_parallel, channel_send_receive, shared_counter};

fn main() {
    let greeting = spawn_and_join();
    println!("{greeting}");

    let sum = sum_in_parallel(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("Sum: {sum}");

    let msgs = channel_send_receive();
    println!("Channel messages: {:?}", msgs);

    let count = shared_counter(1000);
    println!("Shared counter: {count}");
}
