use array_fruit_salad::{sum_array, max_array, reverse_array, take_first};

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Sum: {}", sum_array(&arr));
    println!("Max: {}", max_array(&arr));
    println!("Reversed: {:?}", reverse_array(&arr));
    println!("First 3: {:?}", take_first(&arr, 3));
}
