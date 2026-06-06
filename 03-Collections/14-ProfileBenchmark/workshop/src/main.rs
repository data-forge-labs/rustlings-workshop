use profile_benchmark::*;

const PANGRAM: &str = "the quick brown fox jumps over the lazy dog \
                       the quick brown fox jumps over the lazy dog \
                       the quick brown fox jumps over the lazy dog";

fn main() {
    let counts = count_hashmap(PANGRAM);
    let total: usize = counts.values().sum();
    let unique = counts.len();

    println!("Corpus stats:");
    println!("  Total words : {}", total);
    println!("  Unique words: {}", unique);

    let top = top_n(&counts, 5);

    println!("\nTop 5 words:");
    for (word, count) in top {
        println!("  {:>4} : {}", count, word);
    }
}
