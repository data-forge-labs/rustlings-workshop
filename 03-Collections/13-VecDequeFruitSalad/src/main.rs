///! # What is a VecDeque in Rust and how is it different from a Vector or LinkedList?
///! In Rust, VecDeque is a double-ended queue, which is a data structure that allows 
///! insertion and deletion at both ends of the queue. It is different from a Vector 
///! (or Vec in Rust) in that a Vec only allows efficient insertion and deletion at the 
///! end of the vector, while a LinkedList allows efficient insertion and deletion at both ends, 
///! but it does not provide random access to elements like a Vec does. VecDeque provides a 
///! combination of the capabilities of both Vec and LinkedList, allowing efficient insertion and
///! deletion at both ends as well as random access to elements.
///!
///! # What is the significance of converting VecDeque to a Vector and then back to VecDeque in the program?
///! Converting VecDeque to a Vector and then back to VecDeque in the program allows 
///! shuffling the elements using the shuffle method, which is available for the 
///! standard library Vec. After shuffling, the elements are converted back to 
///! VecDeque to maintain the double-ended queue properties, allowing further 
///! operations on both ends of the queue.
///!
///! # Why do we push "Pomegranate" to the front of the queue and "Fig" and "Cherry" to the back of the queue after shuffling?
///! Pushing "Pomegranate" to the front of the queue and "Fig" and "Cherry" to the back of 
///! the queue after shuffling is done to demonstrate the double-ended queue properties of 
///! VecDeque. This illustrates how elements can be efficiently added to both ends of the 
///! queue, showcasing the flexibility of VecDeque as a double-ended queue.

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    print_fruit_salad(&fruit);

    // Remove the last item from the queue
    let last_item = fruit.pop_back();
    println!("Last item removed: {:?}", last_item);

    // Remove the first item from the queue
    let first_item = fruit.pop_front();
    println!("First item removed: {:?}", first_item);

    // Print out the fruit salad again
    print_fruit_salad(&fruit);
}

fn print_fruit_salad(fruit: &VecDeque<&str>) {
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}