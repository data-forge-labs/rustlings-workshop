use rand::thread_rng;
use vector_fruit_salad::{select_random_fruits, FRUITS};

fn main() {
    let mut rng = thread_rng();
    let fruit = select_random_fruits(5, &FRUITS, &mut rng);
    println!("Fruit salad: {:?}", fruit);
}
