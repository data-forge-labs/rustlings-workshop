use cdc_pipeline::{
    batch_ready, routing_key, should_forward, topic_for, CdcEvent, CdcOp,
    Checkpoint, InMemorySink, LeaderClaim, Sink,
};
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let source = "dataeng.orders";
    let sink = InMemorySink::default();
    let mut cp = Checkpoint::new(source);

    // ---- 1. Simulate a stream of CDC events ----
    let order_id = Uuid::new_v4();
    let events = vec![
        CdcEvent {
            source: source.into(),
            op: CdcOp::Create,
            before: None,
            after: Some(json!({"id": order_id, "status": "pending", "qty": 2})),
            ts_ms: 1_700_000_000_000,
            tx_id: Some(Uuid::new_v4()),
        },
        CdcEvent {
            source: source.into(),
            op: CdcOp::Update,
            before: Some(json!({"id": order_id, "status": "pending"})),
            after: Some(json!({"id": order_id, "status": "paid"})),
            ts_ms: 1_700_000_000_500,
            tx_id: Some(Uuid::new_v4()),
        },
        CdcEvent {
            source: source.into(),
            op: CdcOp::Read, // snapshot event -> dropped
            before: None,
            after: Some(json!({"id": Uuid::new_v4(), "status": "paid"})),
            ts_ms: 1_700_000_001_000,
            tx_id: None,
        },
    ];

    for (i, e) in events.iter().enumerate() {
        if !should_forward(e) { tracing::info!(i, op = ?e.op, "filtered"); continue; }
        let topic = topic_for(&e.source, e.op);
        let op_char: String = e.op.as_char().to_string();
        let key = e.after.as_ref()
            .and_then(|v| v.get("id"))
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .map(|id| routing_key(&e.source, &id))
            .unwrap_or_else(|| e.source.clone());
        sink.send(e).await?;
        cp.advance(i as u64, e.ts_ms);
        tracing::info!(topic = %topic, key = %key, op = %op_char, "forwarded");
    }
    sink.flush().await?;

    // ---- 2. Leader-claim example ----
    let claim = LeaderClaim::new("cdc:orders", "worker-1", 30_000);
    tracing::info!(holder = %claim.holder, valid = claim.is_valid(chrono::Utc::now()),
        "leader claim");

    // ---- 3. Batch decision (size=2, max=10, age=0, max_wait=1000, tx_changed=true) ----
    let ready = batch_ready(sink.len(), 10, 0, 1000, true);
    tracing::info!(in_memory = sink.len(), batch_ready = ready, "batch decision");
    tracing::info!(processed = cp.events_processed, last_lsn = cp.last_lsn,
        "checkpoint");

    Ok(())
}
