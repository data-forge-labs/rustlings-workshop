use turboclean_core::{CleaningEngine, CleaningRule, ProcessingLimits};

fn main() {
    println!("Turboclean — Rust Data Cleansing Engine");
    println!("=======================================");

    let limits = ProcessingLimits::default();
    let engine = CleaningEngine::new(limits);

    let rules = vec![
        CleaningRule::DropNulls {
            columns: vec!["id".into()],
        },
        CleaningRule::FilterOutliersIQR {
            column: "amount".into(),
            factor: 1.5,
        },
    ];

    println!("Engine created with {} rules", rules.len());
    println!("Max memory: {} MB", engine.limits().max_memory_bytes / 1024 / 1024);
    println!("Max columns: {}", engine.limits().max_columns);
    println!();
    println!("Usage:");
    println!("  cargo test                  — run all 16 tests");
    println!("  (CLI coming soon — requires real CSV data)");
}
