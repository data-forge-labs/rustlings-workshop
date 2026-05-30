use vecdeque_fruit_salad::{make_fruit_deque, format_fruit_salad};

fn main() {
    let mut fruit = make_fruit_deque();
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");
    println!("{}", format_fruit_salad(&fruit));
    let _front = fruit.pop_front();
    let _back = fruit.pop_back();
    println!("After pop: {}", format_fruit_salad(&fruit));
}
