use btreeset_fruit::format_set_sorted;
use std::collections::BTreeSet;

fn main() {
    let mut fruits_set = BTreeSet::new();
    fruits_set.insert("apple");
    fruits_set.insert("banana");
    fruits_set.insert("cherry");
    println!("Sorted: {:?}", format_set_sorted(&fruits_set));
}
