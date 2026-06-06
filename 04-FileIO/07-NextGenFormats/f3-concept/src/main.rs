use f3_concept::{build_decoder_bundle, build_mock_footer, footer_to_json, total_wasm_overhead};

fn main() {
    let bundle = build_decoder_bundle();
    println!("F3 Wasm decoder bundle:");
    for d in &bundle {
        println!(
            "  {:20}  v{}  {:>6} bytes  encodings={:?}",
            d.name, d.version, d.bytes_len, d.supported_encodings
        );
    }
    println!("Total Wasm overhead: {} bytes ({:.1} KB)", total_wasm_overhead(&bundle), total_wasm_overhead(&bundle) as f64 / 1024.0);

    let footer = build_mock_footer();
    println!("\nF3 mock footer (JSON manifest):");
    println!("{}", footer_to_json(&footer));
}
