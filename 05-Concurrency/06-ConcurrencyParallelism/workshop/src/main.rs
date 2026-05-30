use concurrency_parallelism::{
    is_send, is_sync, parallel_map, parallel_sum, rwlock_read_heavy, rwlock_write_once,
};
use std::sync::Arc;

fn main() {
    let x = 42i32;
    println!("is_send(i32) = {}", is_send(&x));
    println!("is_sync(i32) = {}", is_sync(&x));

    let arc = Arc::new(42);
    println!("is_send(Arc<i32>) = {}", is_send(&arc));
    println!("is_sync(Arc<i32>) = {}", is_sync(&arc));

    let reads = rwlock_read_heavy(4, 100);
    println!("rwlock_read_heavy(4, 100) = {reads}");

    let writes = rwlock_write_once(10);
    println!("rwlock_write_once(10) = {writes}");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = parallel_sum(data.clone());
    println!("parallel_sum(1..=10) = {sum}");

    let mapped = parallel_map(data, |x| x * 2);
    println!("parallel_map(1..=10, *2) = {:?}", mapped);
}
