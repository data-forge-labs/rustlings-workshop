use rust_iterators::{sum_with_fold, double_all, keep_even};

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    println!("Sum: {}", sum_with_fold(&nums));
    println!("Doubled: {:?}", double_all(&nums));
    println!("Even: {:?}", keep_even(&nums));
}
