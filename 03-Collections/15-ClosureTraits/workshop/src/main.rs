use closure_traits::*;

fn main() {
    let result = apply_fn_once(|| String::from("hello"));
    println!("apply_fn_once: {}", result);

    let mut counter = 0;
    let result = apply_fn_mut(&mut |x| { counter += 1; x * 2 }, 5);
    println!("apply_fn_mut: {} (counter: {})", result, counter);

    let result = apply_fn(&|x| x > 10, 15);
    println!("apply_fn: {}", result);

    let add_five = make_adder(5);
    println!("make_adder(5)(3): {}", add_five(3));

    let mut data = vec![("Alice", 90), ("Bob", 70), ("Carol", 85)];
    sort_by_score(&mut data);
    println!("sorted: {:?}", data);

    let sum: u32 = Counter::new(5).sum();
    println!("Counter sum: {}", sum);

    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fibonacci: {:?}", fib);

    let result = run_pipeline(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("pipeline: {:?}", result);
}
