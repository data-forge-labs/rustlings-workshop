//! Warehouse benchmark — main entry point
//!
//! Generates 1M synthetic e-commerce events, writes them in Parquet + Lance
//! (Nimble and F3 are mocked from published claims), and produces a JSON
//! + Markdown report.

use std::path::Path;
use std::time::Instant;

use arrow_array::{Array, Float32Array, RecordBatch, StringArray};
use rand::Rng;
use rand::SeedableRng;
use std::sync::Arc;
use warehouse::*;

const TOTAL_ROWS: usize = 1_000_000;
const PARTITION_ROWS: usize = 100_000;  // 10 partitions
const N_TAKE_INDICES: usize = 1_000;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    let data_root = std::env::current_dir()?.join("data");
    let results_dir = std::env::current_dir()?.join("results");
    std::fs::create_dir_all(&data_root)?;
    std::fs::create_dir_all(&results_dir)?;

    println!("=== Warehouse Benchmark ===");
    println!("Total rows: {TOTAL_ROWS}");
    println!("Partition size: {PARTITION_ROWS}");
    println!("Machine: {}\n", machine_info());

    let mut report = FullBenchReport {
        timestamp: now_iso(),
        machine: machine_info(),
        total_rows: TOTAL_ROWS,
        results: Vec::new(),
    };

    // -------------------------------------------------------------------------
    // Step 1: Generate data
    // -------------------------------------------------------------------------
    println!("[1/8] Generating {TOTAL_ROWS} synthetic e-commerce events...");
    let t = Instant::now();
    let batches: Vec<RecordBatch> = (0..TOTAL_ROWS / PARTITION_ROWS)
        .map(|i| generate_events(PARTITION_ROWS, 42 + i as u64))
        .collect();
    println!("  Done in {:.2}s ({} partitions)\n", t.elapsed().as_secs_f64(), batches.len());

    // -------------------------------------------------------------------------
    // Step 2: Parquet write
    // -------------------------------------------------------------------------
    println!("[2/8] Writing Parquet (10 partitions)...");
    let parquet_root = data_root.join("parquet");
    std::fs::create_dir_all(&parquet_root)?;
    let mut total_parquet_size = 0u64;
    let t = Instant::now();
    for (i, batch) in batches.iter().enumerate() {
        let day = (i % 28) + 1;
        let p = partition_path(&parquet_root, 2024, 1, day as u32);
        make_dirs(&p)?;
        let file = p.join("events.parquet");
        total_parquet_size += write_parquet(&file, batch)?;
    }
    let dur = t.elapsed();
    let rows_per_sec = TOTAL_ROWS as f64 / dur.as_secs_f64();
    println!("  Done in {:.2}s — {:.0} rows/s — {:.2} MB\n", dur.as_secs_f64(), rows_per_sec, total_parquet_size as f64 / 1_048_576.0);
    report.results.push(BenchResult {
        name: "write_partitioned".into(),
        format: "parquet".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: dur.as_millis() as f64,
        rows_per_sec,
        file_size_bytes: total_parquet_size,
        notes: Some("Snappy compression, 10 partitions".into()),
    });

    // -------------------------------------------------------------------------
    // Step 3: Lance write
    // -------------------------------------------------------------------------
    println!("[3/8] Writing Lance (10 partitions)...");
    let lance_root = data_root.join("lance");
    std::fs::create_dir_all(&lance_root)?;
    let mut total_lance_size = 0u64;
    let t = Instant::now();
    for (i, batch) in batches.iter().enumerate() {
        let day = (i % 28) + 1;
        let p = partition_path(&lance_root, 2024, 1, day as u32).join("events.lance");
        make_dirs(p.parent().unwrap())?;
        total_lance_size += write_lance(&p, batch).await?;
    }
    let dur = t.elapsed();
    let rows_per_sec = TOTAL_ROWS as f64 / dur.as_secs_f64();
    println!("  Done in {:.2}s — {:.0} rows/s — {:.2} MB\n", dur.as_secs_f64(), rows_per_sec, total_lance_size as f64 / 1_048_576.0);
    report.results.push(BenchResult {
        name: "write_partitioned".into(),
        format: "lance".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: dur.as_millis() as f64,
        rows_per_sec,
        file_size_bytes: total_lance_size,
        notes: Some("Default Lance compression, 10 partitions".into()),
    });

    // -------------------------------------------------------------------------
    // Step 4: Vortex write
    // -------------------------------------------------------------------------
    println!("[4/9] Writing Vortex (10 partitions)...");
    let vortex_root = data_root.join("vortex");
    std::fs::create_dir_all(&vortex_root)?;
    let mut total_vortex_size = 0u64;
    let t = Instant::now();
    for (i, batch) in batches.iter().enumerate() {
        let day = (i % 28) + 1;
        let p = partition_path(&vortex_root, 2024, 1, day as u32).join("events.vortex");
        make_dirs(p.parent().unwrap())?;
        total_vortex_size += write_vortex(&p, batch).await?;
    }
    let dur = t.elapsed();
    let rows_per_sec = TOTAL_ROWS as f64 / dur.as_secs_f64();
    println!("  Done in {:.2}s — {:.0} rows/s — {:.2} MB\n", dur.as_secs_f64(), rows_per_sec, total_vortex_size as f64 / 1_048_576.0);
    report.results.push(BenchResult {
        name: "write_partitioned".into(),
        format: "vortex".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: dur.as_millis() as f64,
        rows_per_sec,
        file_size_bytes: total_vortex_size,
        notes: Some("Default Vortex compression, 10 partitions".into()),
    });

    // -------------------------------------------------------------------------
    // Step 5: Sequential scan
    // -------------------------------------------------------------------------
    println!("[4/8] Sequential scan (read all 1M rows)...");
    // Parquet: read first partition
    let pq_first = partition_path(&parquet_root, 2024, 1, 1).join("events.parquet");
    let t = Instant::now();
    let _ = read_parquet(&pq_first)?;
    let pq_first_dur = t.elapsed();
    println!("  Parquet 1 partition: {:.2}ms", pq_first_dur.as_millis() as f64);

    // Read all 10 partitions
    let t = Instant::now();
    let mut total_pq_rows = 0;
    for day in 1..=10u32 {
        let p = partition_path(&parquet_root, 2024, 1, day).join("events.parquet");
        let batch = read_parquet(&p)?;
        total_pq_rows += batch.num_rows();
    }
    let pq_scan = t.elapsed();
    println!("  Parquet  full scan: {:.2}ms ({} rows)", pq_scan.as_millis() as f64, total_pq_rows);
    report.results.push(BenchResult {
        name: "sequential_scan".into(),
        format: "parquet".into(),
        n_rows: total_pq_rows,
        duration_ms: pq_scan.as_millis() as f64,
        rows_per_sec: total_pq_rows as f64 / pq_scan.as_secs_f64(),
        file_size_bytes: 0,
        notes: None,
    });

    // Lance: read first partition
    let ln_first = partition_path(&lance_root, 2024, 1, 1).join("events.lance");
    let t = Instant::now();
    let _ = read_lance(&ln_first).await?;
    let ln_first_dur = t.elapsed();
    println!("  Lance   1 partition: {:.2}ms", ln_first_dur.as_millis() as f64);

    let t = Instant::now();
    let mut total_ln_rows = 0;
    for day in 1..=10u32 {
        let p = partition_path(&lance_root, 2024, 1, day).join("events.lance");
        let batch = read_lance(&p).await?;
        total_ln_rows += batch.num_rows();
    }
    let ln_scan = t.elapsed();
    println!("  Lance   full scan: {:.2}ms ({} rows)", ln_scan.as_millis() as f64, total_ln_rows);
    report.results.push(BenchResult {
        name: "sequential_scan".into(),
        format: "lance".into(),
        n_rows: total_ln_rows,
        duration_ms: ln_scan.as_millis() as f64,
        rows_per_sec: total_ln_rows as f64 / ln_scan.as_secs_f64(),
        file_size_bytes: 0,
        notes: None,
    });

    // Vortex: read first partition
    let vx_first = partition_path(&vortex_root, 2024, 1, 1).join("events.vortex");
    let t = Instant::now();
    let _ = read_vortex(&vx_first).await?;
    let vx_first_dur = t.elapsed();
    println!("  Vortex  1 partition: {:.2}ms", vx_first_dur.as_millis() as f64);

    let t = Instant::now();
    let mut total_vx_rows = 0;
    for day in 1..=10u32 {
        let p = partition_path(&vortex_root, 2024, 1, day).join("events.vortex");
        let batch = read_vortex(&p).await?;
        total_vx_rows += batch.num_rows();
    }
    let vx_scan = t.elapsed();
    println!("  Vortex  full scan: {:.2}ms ({} rows)", vx_scan.as_millis() as f64, total_vx_rows);
    report.results.push(BenchResult {
        name: "sequential_scan".into(),
        format: "vortex".into(),
        n_rows: total_vx_rows,
        duration_ms: vx_scan.as_millis() as f64,
        rows_per_sec: total_vx_rows as f64 / vx_scan.as_secs_f64(),
        file_size_bytes: 0,
        notes: None,
    });

    // Mocks
    let nimble_scan = mock_nimble_scan(TOTAL_ROWS);
    report.results.push(BenchResult {
        name: "sequential_scan".into(),
        format: "nimble (mocked)".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: nimble_scan.latency_ms,
        rows_per_sec: nimble_scan.throughput_rows_per_sec,
        file_size_bytes: 0,
        notes: Some("Mock from Meta benchmark claims".into()),
    });
    let f3_scan = mock_f3_scan(TOTAL_ROWS);
    report.results.push(BenchResult {
        name: "sequential_scan".into(),
        format: "f3 (mocked)".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: f3_scan.latency_ms,
        rows_per_sec: f3_scan.throughput_rows_per_sec,
        file_size_bytes: 0,
        notes: Some("Mock from F3 SIGMOD 2026 paper".into()),
    });
    println!();

    // -------------------------------------------------------------------------
    // Step 5: Column projection (read 2 of 6 columns)
    // -------------------------------------------------------------------------
    println!("[5/8] Column projection (2 of 6 columns: id, amount)...");
    let proj_cols = vec!["id".to_string(), "amount".to_string()];

    // Parquet
    let t = Instant::now();
    for day in 1..=10u32 {
        let p = partition_path(&parquet_root, 2024, 1, day).join("events.parquet");
        let _ = read_parquet_columns(&p, &proj_cols)?;
    }
    let pq_proj = t.elapsed();
    println!("  Parquet: {:.2}ms", pq_proj.as_millis() as f64);
    report.results.push(BenchResult {
        name: "column_projection_2_of_6".into(),
        format: "parquet".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: pq_proj.as_millis() as f64,
        rows_per_sec: TOTAL_ROWS as f64 / pq_proj.as_secs_f64(),
        file_size_bytes: 0,
        notes: Some("Projection pushed to row group level".into()),
    });

    // Lance
    let t = Instant::now();
    for day in 1..=10u32 {
        let p = partition_path(&lance_root, 2024, 1, day).join("events.lance");
        let _ = read_lance_columns(&p, &["id", "amount"]).await?;
    }
    let ln_proj = t.elapsed();
    println!("  Lance:  {:.2}ms", ln_proj.as_millis() as f64);
    report.results.push(BenchResult {
        name: "column_projection_2_of_6".into(),
        format: "lance".into(),
        n_rows: TOTAL_ROWS,
        duration_ms: ln_proj.as_millis() as f64,
        rows_per_sec: TOTAL_ROWS as f64 / ln_proj.as_secs_f64(),
        file_size_bytes: 0,
        notes: None,
    });
    println!();

    // -------------------------------------------------------------------------
    // Step 6: Random take (1000 random rows)
    // -------------------------------------------------------------------------
    println!("[6/8] Random take ({} random rows from first partition)...", N_TAKE_INDICES);
    let mut rng = rand::rngs::StdRng::seed_from_u64(99);
    let indices: Vec<u32> = (0..N_TAKE_INDICES).map(|_| rng.gen_range(0..PARTITION_ROWS as u32)).collect();

    // Parquet: full read then sample (no native take)
    let t = Instant::now();
    let pq_batch = read_parquet(&pq_first)?;
    let pq_take = t.elapsed();
    println!("  Parquet (read-all-then-sample): {:.2}ms ({} rows total read)", pq_take.as_millis() as f64, pq_batch.num_rows());
    report.results.push(BenchResult {
        name: format!("random_take_{}", N_TAKE_INDICES),
        format: "parquet".into(),
        n_rows: N_TAKE_INDICES,
        duration_ms: pq_take.as_millis() as f64,
        rows_per_sec: N_TAKE_INDICES as f64 / pq_take.as_secs_f64(),
        file_size_bytes: 0,
        notes: Some("Parquet has no native take — must read all".into()),
    });

    // Lance: native take
    let t = Instant::now();
    let _ = take_lance(&ln_first, &indices).await?;
    let ln_take = t.elapsed();
    println!("  Lance  (native take):           {:.2}ms", ln_take.as_millis() as f64);
    report.results.push(BenchResult {
        name: format!("random_take_{}", N_TAKE_INDICES),
        format: "lance".into(),
        n_rows: N_TAKE_INDICES,
        duration_ms: ln_take.as_millis() as f64,
        rows_per_sec: N_TAKE_INDICES as f64 / ln_take.as_secs_f64(),
        file_size_bytes: 0,
        notes: Some("Lance structural encoding: O(1) seeks".into()),
    });

    // Nimble mock
    let nimble_take = mock_nimble_random_take(PARTITION_ROWS, N_TAKE_INDICES);
    report.results.push(BenchResult {
        name: format!("random_take_{}", N_TAKE_INDICES),
        format: "nimble (mocked)".into(),
        n_rows: N_TAKE_INDICES,
        duration_ms: nimble_take.latency_ms,
        rows_per_sec: nimble_take.throughput_rows_per_sec,
        file_size_bytes: 0,
        notes: Some("Mock from Meta benchmark".into()),
    });

    // F3 mock
    let f3_take = mock_f3_random_take(PARTITION_ROWS, N_TAKE_INDICES);
    report.results.push(BenchResult {
        name: format!("random_take_{}", N_TAKE_INDICES),
        format: "f3 (mocked)".into(),
        n_rows: N_TAKE_INDICES,
        duration_ms: f3_take.latency_ms,
        rows_per_sec: f3_take.throughput_rows_per_sec,
        file_size_bytes: 0,
        notes: Some("F3 has no structural encoding, similar to Parquet".into()),
    });
    println!();

    // -------------------------------------------------------------------------
    // Step 7: Predicate pushdown (filter event_type='purchase')
    // -------------------------------------------------------------------------
    println!("[7/8] Predicate pushdown (event_type = 'purchase')...");
    // Parquet: full read, no row-group level filter
    let t = Instant::now();
    let mut pq_purchase = 0;
    for day in 1..=10u32 {
        let p = partition_path(&parquet_root, 2024, 1, day).join("events.parquet");
        let batch = read_parquet(&p)?;
        let arr = batch.column(2).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..arr.len() {
            if arr.value(i) == "purchase" { pq_purchase += 1; }
        }
    }
    let pq_filter = t.elapsed();
    println!("  Parquet (post-read filter): {:.2}ms ({} matches)", pq_filter.as_millis() as f64, pq_purchase);
    report.results.push(BenchResult {
        name: "filter_purchase".into(),
        format: "parquet".into(),
        n_rows: pq_purchase,
        duration_ms: pq_filter.as_millis() as f64,
        rows_per_sec: TOTAL_ROWS as f64 / pq_filter.as_secs_f64(),
        file_size_bytes: 0,
        notes: Some("Parquet has row-group statistics but limited string pushdown".into()),
    });

    // Lance: native filter
    let t = Instant::now();
    let mut ln_purchase = 0;
    for day in 1..=10u32 {
        let p = partition_path(&lance_root, 2024, 1, day).join("events.lance");
        let batch = filter_lance(&p, "purchase").await?;
        ln_purchase += batch.num_rows();
    }
    let ln_filter = t.elapsed();
    println!("  Lance  (native filter):     {:.2}ms ({} matches)", ln_filter.as_millis() as f64, ln_purchase);
    report.results.push(BenchResult {
        name: "filter_purchase".into(),
        format: "lance".into(),
        n_rows: ln_purchase,
        duration_ms: ln_filter.as_millis() as f64,
        rows_per_sec: TOTAL_ROWS as f64 / ln_filter.as_secs_f64(),
        file_size_bytes: 0,
        notes: Some("Lance pushdown via DataFusion predicate".into()),
    });
    println!();

    // -------------------------------------------------------------------------
    // Step 8: Compaction (Lance optimize)
    // -------------------------------------------------------------------------
    println!("[8/8] Compaction (Lance: optimize 10 small files → 1 large)...");
    let pre_size = dir_size(&ln_first);
    let t = Instant::now();
    compact_lance(&ln_first).await?;
    let compact_dur = t.elapsed();
    let post_size = dir_size(&ln_first);
    println!("  Compaction: {:.2}ms ({} -> {} bytes)\n", compact_dur.as_millis() as f64, pre_size, post_size);
    report.results.push(BenchResult {
        name: "compact".into(),
        format: "lance".into(),
        n_rows: PARTITION_ROWS,
        duration_ms: compact_dur.as_millis() as f64,
        rows_per_sec: 0.0,
        file_size_bytes: post_size,
        notes: Some(format!("Reduced {} -> {} bytes", pre_size, post_size)),
    });

    // -------------------------------------------------------------------------
    // Step 9: Schema evolution (add 2 new columns to existing Lance)
    // -------------------------------------------------------------------------
    println!("[9/9] Schema evolution: add 2 new columns, then append...");
    let t = Instant::now();

    // First: add new columns as all-null (Lance requires schema before append)
    let mut dataset = lance::dataset::Dataset::open(
        ln_first.to_str().unwrap(),
    ).await?;
    use lance::dataset::NewColumnTransform;
    let new_cols = Arc::new(arrow_schema::Schema::new(vec![
        arrow_schema::Field::new("device", arrow_schema::DataType::Utf8, true),
        arrow_schema::Field::new("session_id", arrow_schema::DataType::Utf8, true),
    ]));
    dataset.add_columns(
        NewColumnTransform::AllNulls(new_cols),
        None,  // read_columns = None (read all)
        Some(1024),  // batch size
    ).await?;

    // Now append a batch with all 8 columns
    let new_schema = Arc::new(arrow_schema::Schema::new(vec![
        arrow_schema::Field::new("id", arrow_schema::DataType::UInt32, false),
        arrow_schema::Field::new("user_id", arrow_schema::DataType::UInt32, false),
        arrow_schema::Field::new("event_type", arrow_schema::DataType::Utf8, false),
        arrow_schema::Field::new("amount", arrow_schema::DataType::Float32, true),
        arrow_schema::Field::new("country", arrow_schema::DataType::Utf8, false),
        arrow_schema::Field::new("timestamp", arrow_schema::DataType::Int64, false),
        arrow_schema::Field::new("device", arrow_schema::DataType::Utf8, true),
        arrow_schema::Field::new("session_id", arrow_schema::DataType::Utf8, true),
    ]));
    let small_batch = RecordBatch::try_new(
        new_schema,
        vec![
            Arc::new(arrow_array::UInt32Array::from(vec![1, 2, 3])),
            Arc::new(arrow_array::UInt32Array::from(vec![100, 200, 300])),
            Arc::new(StringArray::from(vec!["click", "view", "purchase"])),
            Arc::new(Float32Array::from(vec![None, None, Some(50.0)])),
            Arc::new(StringArray::from(vec!["US", "UK", "DE"])),
            Arc::new(arrow_array::Int64Array::from(vec![1_000_000, 1_000_001, 1_000_002])),
            Arc::new(StringArray::from(vec!["ios", "android", "web"])),
            Arc::new(StringArray::from(vec!["s1", "s2", "s3"])),
        ],
    )?;
    let total = append_lance(&ln_first, &small_batch).await?;
    let evol_dur = t.elapsed();
    println!("  Schema evolution: {:.2}ms (new total: {} rows)\n", evol_dur.as_millis() as f64, total);
    report.results.push(BenchResult {
        name: "schema_evolution_append".into(),
        format: "lance".into(),
        n_rows: 3,
        duration_ms: evol_dur.as_millis() as f64,
        rows_per_sec: 0.0,
        file_size_bytes: 0,
        notes: Some("Added 2 new columns without rewriting existing data".into()),
    });

    // -------------------------------------------------------------------------
    // Write reports
    // -------------------------------------------------------------------------
    let json_path = results_dir.join("bench_results.json");
    write_json_report(&json_path, &report)?;
    println!("JSON report written to {}", json_path.display());

    let md_path = results_dir.join("bench_results.md");
    write_markdown_report(&md_path, &report)?;
    println!("Markdown report written to {}", md_path.display());

    Ok(())
}

/// Write a Markdown table of the benchmark results.
fn write_markdown_report(path: &Path, report: &FullBenchReport) -> Result<()> {
    use std::fmt::Write;
    let mut s = String::new();
    writeln!(s, "# Warehouse Benchmark Results")?;
    writeln!(s)?;
    writeln!(s, "- **Timestamp:** {}", report.timestamp)?;
    writeln!(s, "- **Machine:** {}", report.machine)?;
    writeln!(s, "- **Total rows:** {}", report.total_rows)?;
    writeln!(s)?;
    writeln!(s, "| Benchmark | Format | Rows | Duration (ms) | Rows/s | Size (bytes) | Notes |")?;
    writeln!(s, "|-----------|--------|------|---------------|--------|--------------|-------|")?;
    for r in &report.results {
        let notes = r.notes.clone().unwrap_or_default();
        let notes = notes.replace('|', "\\|");
        writeln!(
            s, "| {} | {} | {} | {:.1} | {:.0} | {} | {} |",
            r.name, r.format, r.n_rows, r.duration_ms, r.rows_per_sec, r.file_size_bytes, notes
        )?;
    }
    writeln!(s)?;
    writeln!(s, "## Format Summary")?;
    writeln!(s)?;
    writeln!(s, "- **Parquet**: baseline (Snappy compression)")?;
    writeln!(s, "- **Lance**: real benchmarks via `lance = \"0.20\"`")?;
    writeln!(s, "- **Nimble (mocked)**: numbers from Meta's published benchmark claims")?;
    writeln!(s, "- **F3 (mocked)**: numbers from SIGMOD 2026 paper (FFF-bench)")?;
    std::fs::write(path, s)?;
    Ok(())
}
