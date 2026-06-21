use crate::storage::Store;
use std::time::{Duration, Instant};
use tokio::time;

pub fn start_cleanup(store: Store) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut store = store.lock().unwrap();
            let now = Instant::now();
            store.retain(|_, entry| entry.expires_at.map_or(true, |exp| now < exp));
        }
    });
}
