use hashmap_count::count_frequencies;

fn main() {
    let numbers = vec![1, 2, 3, 4, 7, 7, 5, 6, 1, 7, 1, 8, 2, 2, 2, 2, 9, 10];
    let result = count_frequencies(numbers);
    println!("{:?}", result);
}
