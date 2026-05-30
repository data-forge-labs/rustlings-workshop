use mutable_fruit_salad::{add_fruit, sort_fruits, pick_random_fruit, remove_fruit};
use rand::thread_rng;

fn main() {
    let mut salad = vec!["banana", "apple", "cherry", "date"];
    sort_fruits(&mut salad);
    println!("Sorted: {:?}", salad);

    add_fruit(&mut salad, "fig");
    println!("After add: {:?}", salad);

    let mut rng = thread_rng();
    if let Some(picked) = pick_random_fruit(&salad, &mut rng) {
        println!("Removing: {}", picked);
        remove_fruit(&mut salad, picked);
        println!("After remove: {:?}", salad);
    }
}
