use graph_visualize::{
    ascii_bar_chart, create_series, data_summary, generate_sample_data, normalize_data,
};
use rasciigraph::{plot, Config};

fn main() {
    // --- lib.rs demo ---
    let data = generate_sample_data();
    println!("Sample data: {:?}", data);

    let (min, max, mean) = data_summary(&data);
    println!("Summary: min={}, max={}, mean={}", min, max, mean);

    let norm = normalize_data(&data);
    println!("Normalized: {:?}", norm);

    let labels = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    let chart = ascii_bar_chart(&data, &labels);
    println!("ASCII bar chart:");
    for line in &chart {
        println!("{}", line);
    }

    let names = vec!["X", "Y", "Z"];
    let vals = vec![10.0, 20.0, 30.0];
    let series = create_series(&names, &vals);
    println!("Series: {:?}", series);

    // --- original rasciigraph demo ---
    let cities = vec![
        "Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow",
    ];
    let distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

    let city_path = cities.join(" > ");
    println!("\n{}", city_path);

    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_string())
        )
    );

    let cos_vec = coseno_data();
    println!(
        "{}",
        plot(
            cos_vec.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Coseno".to_string())
        )
    );
}

fn coseno_data() -> Vec<f64> {
    let mut cos_vec = Vec::new();
    for i in 0..=30 {
        cos_vec.push((i as f64).cos());
    }
    cos_vec
}