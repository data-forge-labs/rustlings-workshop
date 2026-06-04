use rand::rng;
use vector_fruit_salad::{select_random_fruits, FRUITS};

fn main() {
    let mut rng = rng();
    let fruit = select_random_fruits(5, &FRUITS, &mut rng);
    println!("Fruit salad: {:?}", fruit);
}
