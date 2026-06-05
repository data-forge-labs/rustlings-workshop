use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct PipelineMetrics {
    pub source_emitted: AtomicUsize,
    pub transform_passed: AtomicUsize,
    pub transform_dropped: AtomicUsize,
    pub sink_written: AtomicUsize,
    pub source_errors: AtomicUsize,
    pub transform_errors: AtomicUsize,
    pub sink_errors: AtomicUsize,
}

impl PipelineMetrics {
    pub fn new() -> Self {
        Self {
            source_emitted: AtomicUsize::new(0),
            transform_passed: AtomicUsize::new(0),
            transform_dropped: AtomicUsize::new(0),
            sink_written: AtomicUsize::new(0),
            source_errors: AtomicUsize::new(0),
            transform_errors: AtomicUsize::new(0),
            sink_errors: AtomicUsize::new(0),
        }
    }

    pub fn snapshot(&self) -> (usize, usize, usize, usize, usize, usize, usize) {
        (
            self.source_emitted.load(Ordering::Relaxed),
            self.transform_passed.load(Ordering::Relaxed),
            self.transform_dropped.load(Ordering::Relaxed),
            self.sink_written.load(Ordering::Relaxed),
            self.source_errors.load(Ordering::Relaxed),
            self.transform_errors.load(Ordering::Relaxed),
            self.sink_errors.load(Ordering::Relaxed),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub id: u64,
    pub value: f64,
}

pub fn make_source(rows: Vec<Row>) -> mpsc::Sender<Row> {
    todo!()
}

pub fn make_transform(
    rx: mpsc::Receiver<Row>,
    metrics: Arc<PipelineMetrics>,
    predicate: fn(&Row) -> bool,
) -> mpsc::Sender<Row> {
    todo!()
}

pub fn make_sink(
    rx: mpsc::Receiver<Row>,
    metrics: Arc<PipelineMetrics>,
) -> mpsc::Receiver<Row> {
    todo!()
}

pub async fn run_source(
    rows: Vec<Row>,
    tx: mpsc::Sender<Row>,
    metrics: Arc<PipelineMetrics>,
) {
    todo!()
}

pub async fn run_transform(
    mut rx: mpsc::Receiver<Row>,
    tx: mpsc::Sender<Row>,
    metrics: Arc<PipelineMetrics>,
    predicate: fn(&Row) -> bool,
) {
    todo!()
}

pub async fn run_sink(mut rx: mpsc::Receiver<Row>, metrics: Arc<PipelineMetrics>) -> Vec<Row> {
    todo!()
}

pub fn filter_positive(r: &Row) -> bool {
    r.value > 0.0
}

pub fn filter_large(r: &Row) -> bool {
    r.value > 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_source_emits_all_rows() {
        let rows = vec![
            Row { id: 1, value: 1.0 },
            Row { id: 2, value: 2.0 },
            Row { id: 3, value: 3.0 },
        ];
        let (tx, mut rx) = mpsc::channel(8);
        let m = Arc::new(PipelineMetrics::new());
        run_source(rows, tx, Arc::clone(&m)).await;
        let mut out = vec![];
        while let Some(r) = rx.recv().await {
            out.push(r);
        }
        assert_eq!(out.len(), 3);
        assert_eq!(m.source_emitted.load(Ordering::Relaxed), 3);
    }

    #[tokio::test]
    async fn test_transform_passes_positive() {
        let (tx_in, rx_in) = mpsc::channel(8);
        let (tx_out, mut rx_out) = mpsc::channel(8);
        let m = Arc::new(PipelineMetrics::new());
        let _t = tokio::spawn(run_transform(rx_in, tx_out, Arc::clone(&m), filter_positive));
        tx_in.send(Row { id: 1, value: 5.0 }).await.unwrap();
        tx_in.send(Row { id: 2, value: -1.0 }).await.unwrap();
        tx_in.send(Row { id: 3, value: 0.0 }).await.unwrap();
        tx_in.send(Row { id: 4, value: 10.0 }).await.unwrap();
        drop(tx_in);
        let mut out = vec![];
        while let Some(r) = rx_out.recv().await {
            out.push(r);
        }
        assert_eq!(out.len(), 2);
        assert_eq!(m.transform_passed.load(Ordering::Relaxed), 2);
        assert_eq!(m.transform_dropped.load(Ordering::Relaxed), 2);
        let _ = _t.await;
    }

    #[tokio::test]
    async fn test_sink_collects_rows() {
        let (tx_in, rx_in) = mpsc::channel(8);
        let m = Arc::new(PipelineMetrics::new());
        let sink_handle = tokio::spawn(run_sink(rx_in, Arc::clone(&m)));
        tx_in.send(Row { id: 1, value: 1.0 }).await.unwrap();
        tx_in.send(Row { id: 2, value: 2.0 }).await.unwrap();
        drop(tx_in);
        let collected = sink_handle.await.unwrap();
        assert_eq!(collected.len(), 2);
        assert_eq!(m.sink_written.load(Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_end_to_end_pipeline() {
        let rows = (0..10)
            .map(|i| Row { id: i, value: i as f64 * 50.0 })
            .collect::<Vec<_>>();
        let (src_tx, src_rx) = mpsc::channel(4);
        let (trans_tx, trans_rx) = mpsc::channel(4);
        let m = Arc::new(PipelineMetrics::new());
        let src_handle = tokio::spawn(run_source(rows, src_tx, Arc::clone(&m)));
        let trans_handle = tokio::spawn(run_transform(src_rx, trans_tx, Arc::clone(&m), filter_large));
        let sink_handle = tokio::spawn(run_sink(trans_rx, Arc::clone(&m)));
        src_handle.await.unwrap();
        let _ = trans_handle.await;
        let collected = sink_handle.await.unwrap();
        let (emitted, passed, dropped, written, _, _, _) = m.snapshot();
        assert_eq!(emitted, 10);
        assert_eq!(written, collected.len());
        assert_eq!(passed, 3);
        assert_eq!(dropped, 7);
        assert!(collected.iter().all(|r| r.value > 100.0));
    }

    #[tokio::test]
    async fn test_empty_pipeline() {
        let m = Arc::new(PipelineMetrics::new());
        let (src_tx, src_rx) = mpsc::channel::<Row>(4);
        let (trans_tx, trans_rx) = mpsc::channel::<Row>(4);
        let src_h = tokio::spawn(run_source(vec![], src_tx, Arc::clone(&m)));
        let trans_h = tokio::spawn(run_transform(src_rx, trans_tx, Arc::clone(&m), filter_positive));
        let sink_h = tokio::spawn(run_sink(trans_rx, Arc::clone(&m)));
        src_h.await.unwrap();
        let _ = trans_h.await;
        let collected = sink_h.await.unwrap();
        assert!(collected.is_empty());
        let (emitted, _, _, written, _, _, _) = m.snapshot();
        assert_eq!(emitted, 0);
        assert_eq!(written, 0);
    }
}
