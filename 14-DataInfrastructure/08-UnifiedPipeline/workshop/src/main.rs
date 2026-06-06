use serde_json::json;
use unified_pipeline::{
    fanout_targets, sink_backoff_ms, DeadLetter, PipelineConfig, PipelineEvent, PipelineStats,
    SinkOutcome, WindowCounters,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    // ---- 1. Configure the pipeline ----
    let cfg = PipelineConfig::new("orders-pipeline", "dataeng.orders")
        .with_sink("redis-cache");
    tracing::info!(name = %cfg.name, source = %cfg.source,
        sinks = cfg.sinks.len(), max_batch = cfg.max_batch,
        "pipeline config");

    // ---- 2. Build a small event stream ----
    let aggregate = Uuid::new_v4();
    let events: Vec<PipelineEvent> = (0..5).map(|i| PipelineEvent::new(
        "dataeng.orders",
        if i % 2 == 0 { "order.created" } else { "order.paid" },
        aggregate,
        json!({"seq": i}),
    )).collect();

    // ---- 3. Per-sink fan-out outcomes (synchronous simulation) ----
    let mut stats = PipelineStats::default();
    let mut counters = WindowCounters::default();
    for e in &events {
        stats.events_in += 1;
        counters.record(e);
        let targets = fanout_targets(e, &cfg);
        for sink in &targets {
            // Simulate: every 3rd kafka send "fails" so the
            // retry/DLQ paths get exercised in the demo.
            let outcome = if sink == "kafka" && e.created_at.timestamp() % 3 == 0 {
                SinkOutcome::err(sink.clone(), "broker timeout", 50)
            } else {
                SinkOutcome::ok(sink.clone(), 5)
            };
            if outcome.success {
                stats.events_fanned_out += 1;
            } else {
                stats.sink_failures += 1;
                let backoff = sink_backoff_ms(0, 100, 10_000);
                tracing::warn!(sink = %outcome.sink, error = ?outcome.error,
                    retry_in_ms = ?backoff, "sink failed, would retry");
                if let Some(_b) = backoff {
                    // In a real loop we would sleep(_b) and resend.
                    // Demo: mark as dead-lettered after exhausting retries.
                    let _dlq = DeadLetter::new(e.clone(), outcome.sink.clone(),
                        outcome.error.clone().unwrap_or_default(), 1);
                    stats.dead_letters += 1;
                }
                counters.record_error();
            }
        }
    }
    tracing::info!(counters_total = counters.total, errors = counters.errors,
        types = counters.distinct_types(), "window counters");

    // ---- 4. Final stats ----
    stats.uptime_ms = 1_234;
    tracing::info!(in = stats.events_in, out = stats.events_fanned_out,
        fails = stats.sink_failures, dlq = stats.dead_letters,
        success_rate = stats.success_rate(), "pipeline stats");

    Ok(())
}
