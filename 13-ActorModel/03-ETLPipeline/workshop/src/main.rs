use etl_pipeline_workshop::{
    filter_large, run_sink, run_source, run_transform, PipelineMetrics, Row,
};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rows: Vec<Row> = (0..10).map(|i| Row { id: i, value: i as f64 * 50.0 }).collect();

    let (src_tx, src_rx) = mpsc::channel(4);
    let (trans_tx, trans_rx) = mpsc::channel(4);
    let m = Arc::new(PipelineMetrics::new());

    let src_h = tokio::spawn(run_source(rows, src_tx, Arc::clone(&m)));
    let trans_h = tokio::spawn(run_transform(src_rx, trans_tx, Arc::clone(&m), filter_large));
    let sink_h = tokio::spawn(run_sink(trans_rx, Arc::clone(&m)));

    src_h.await?;
    let _ = trans_h.await;
    let collected = sink_h.await?;

    let (emitted, passed, dropped, written, _, _, _) = m.snapshot();
    println!("emitted: {} passed: {} dropped: {} written: {}", emitted, passed, dropped, written);
    println!("collected: {:?}", collected);
    Ok(())
}
