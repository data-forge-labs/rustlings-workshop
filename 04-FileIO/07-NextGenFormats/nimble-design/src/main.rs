use nimble_design::{recommend_encoding_for_sample, stream_to_json, Stream, Encoding};

fn main() {
    // Recommend encoding for various samples
    let constant = vec![42_i64; 100];
    let sorted: Vec<i64> = (0..100).collect();
    let small_range = vec![1000_i64, 1001, 1002, 1003, 1004, 1005];
    let random: Vec<i64> = vec![1, 5, 100, 50_000, 99, 1_000_000];

    println!("Constant values      -> {:?}", recommend_encoding_for_sample(&constant));
    println!("Sorted sequence      -> {:?}", recommend_encoding_for_sample(&sorted));
    println!("Small-range integers -> {:?}", recommend_encoding_for_sample(&small_range));
    println!("Random integers      -> {:?}", recommend_encoding_for_sample(&random));

    // Serialize a stream manifest
    let s = Stream {
        name: "user_id".into(),
        encoding: Encoding::Dictionary,
        byte_size: 8192,
        null_count: 0,
    };
    println!("Stream manifest: {}", stream_to_json(&s));
}
