use proptest_workshop::{
    count_above, dedup_sorted, is_sorted_ascending, min_max, normalize_floats, reverse_vec,
    sort_ascending, sum_vec,
};
use proptest::prelude::*;
use std::error::Error;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]

    #[test]
    fn cli_smoke(v in proptest::collection::vec(-100i32..100, 1..20)) {
        let sorted = sort_ascending(v.clone());
        assert!(is_sorted_ascending(&sorted));
        let n = min_max(&v).map(|(a, b)| b - a).unwrap_or(0);
        assert!(n >= 0);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![5, -2, 0, 7, 1, 3];
    println!("data:          {:?}", data);
    println!("sorted:        {:?}", sort_ascending(data.clone()));
    println!("reversed:      {:?}", reverse_vec(data.clone()));
    println!("sum:           {}", sum_vec(data.clone()));
    println!("min,max:       {:?}", min_max(&data));
    println!("deduped:       {:?}", dedup_sorted(data.clone()));
    println!("count>2:       {}", count_above(&data, 2));
    println!("is_sorted:     {}", is_sorted_ascending(&data));
    println!("normalized:    {:?}", normalize_floats(vec![1.0, 5.0, 10.0]));
    Ok(())
}
