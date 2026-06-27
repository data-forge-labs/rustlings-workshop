use std::collections::{BTreeSet, HashMap};

pub fn generate_fruit_set<'a>(fruits: &'a [&'a str], amount: usize, rng: &mut impl rand::Rng) -> (BTreeSet<&'a str>, HashMap<&'a str, u32>) {
    let mut set = BTreeSet::new();
    let mut counter = HashMap::new();
    for _ in 0..amount {
        let fruit = fruits[rng.random_range(0..fruits.len())];
        set.insert(fruit);
        *counter.entry(fruit).or_insert(0) += 1;
    }
    (set, counter)
}

pub fn format_set_sorted<'a>(set: &'a BTreeSet<&'a str>) -> Vec<&'a str> {
    set.iter().copied().collect()
}

pub fn format_set_reverse<'a>(set: &'a BTreeSet<&'a str>) -> Vec<&'a str> {
    set.iter().rev().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    mod step_01_btreeset {
        #[test]
        fn test_generate_set_unique() {
            let fruits = vec!["apple", "banana", "cherry"];
            let mut rng = rng();
            let (set, _) = generate_fruit_set(&fruits, 3, &mut rng);
            assert_eq!(set.len(), 3);
        }

        #[test]
        fn test_generate_set_no_duplicates() {
            let fruits = vec!["apple", "apple"];
            let mut rng = rng();
            let (set, _) = generate_fruit_set(&fruits, 2, &mut rng);
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn test_format_set_sorted() {
            let mut set = BTreeSet::new();
            set.insert("banana");
            set.insert("apple");
            set.insert("cherry");
            let sorted = format_set_sorted(&set);
            assert_eq!(sorted, vec!["apple", "banana", "cherry"]);
        }

        #[test]
        fn test_format_set_reverse() {
            let mut set = BTreeSet::new();
            set.insert("apple");
            set.insert("banana");
            set.insert("cherry");
            let rev = format_set_reverse(&set);
            assert_eq!(rev, vec!["cherry", "banana", "apple"]);
        }
    }
}
