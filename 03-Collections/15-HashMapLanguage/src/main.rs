use hashmap_language::{languages, normalize};

fn main() {
    let weights = normalize(&mut languages());
    for (name, weight) in weights {
        println!("{}: {}", name, weight);
    }
}
