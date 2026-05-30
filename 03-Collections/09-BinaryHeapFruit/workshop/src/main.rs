use binaryheap_fruit::{generate_fruit_salad, Fruit};

fn main() {
    let mut fruit_salad = generate_fruit_salad();
    println!("Fruit salad (sorted by priority):");
    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(name) => println!("{}", name),
        }
    }
}
