use rust_collections_docs::{word_counter, PriorityQueue, Item};

fn main() {
    let sample_text = "Hello, world! This is a sample text. This is another sample text.";
    let word_count = word_counter(sample_text);
    println!("Word count: {:?}", word_count);

    let mut pq = PriorityQueue::new();
    pq.push(Item { priority: 1, value: "A".into() });
    pq.push(Item { priority: 3, value: "C".into() });
    pq.push(Item { priority: 2, value: "B".into() });
    while let Some(item) = pq.pop() {
        println!("{:?}", item);
    }
}
