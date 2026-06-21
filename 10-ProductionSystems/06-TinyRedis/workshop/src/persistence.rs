use crate::storage::{Entry, Store};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time;

#[derive(Serialize, Deserialize)]
struct PersistedEntry {
    value: String,
    expires_at_unix: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct Snapshot {
    entries: HashMap<String, PersistedEntry>,
}

pub async fn load_from_disk(path: &str, store: &Store) {
    if !Path::new(path).exists() {
        println!("No snapshot found at {}. Starting fresh.", path);
        return;
    }
    match tokio::fs::read_to_string(path).await {
        Err(e) => eprintln!("Could not read snapshot: {}", e),
        Ok(content) => match serde_json::from_str::<Snapshot>(&content) {
            Err(e) => eprintln!("Could not parse snapshot: {}", e),
            Ok(snapshot) => {
                let now_unix = unix_now();
                let mut store = store.lock().unwrap();
                for (key, pe) in snapshot.entries {
                    let expires_at = pe.expires_at_unix.and_then(|exp_unix| {
                        if exp_unix > now_unix {
                            let remaining = exp_unix - now_unix;
                            Some(Instant::now() + Duration::from_secs(remaining))
                        } else {
                            None
                        }
                    });
                    if pe.expires_at_unix.map_or(true, |_| expires_at.is_some()) {
                        store.insert(
                            key,
                            Entry {
                                value: pe.value,
                                expires_at,
                            },
                        );
                    }
                }
                println!("Loaded {} keys from {}", store.len(), path);
            }
        },
    }
}

pub fn start_persistence(store: Store, path: &str) {
    let path = path.to_string();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            save_snapshot(&store, &path).await;
        }
    });
}

async fn save_snapshot(store: &Store, path: &str) {
    let snapshot = {
        let store = store.lock().unwrap();
        let now = Instant::now();
        let now_unix = unix_now();
        let entries = store
            .iter()
            .filter(|(_, e)| e.expires_at.map_or(true, |exp| now < exp))
            .map(|(k, e)| {
                let expires_at_unix = e.expires_at.map(|exp| {
                    let remaining = (exp - now).as_secs();
                    now_unix + remaining
                });
                (
                    k.clone(),
                    PersistedEntry {
                        value: e.value.clone(),
                        expires_at_unix,
                    },
                )
            })
            .collect();
        Snapshot { entries }
    };
    match serde_json::to_string_pretty(&snapshot) {
        Err(e) => eprintln!("Serialization failed: {}", e),
        Ok(json) => match tokio::fs::write(path, json).await {
            Err(e) => eprintln!("Could not write snapshot: {}", e),
            Ok(_) => println!(
                "[snapshot] Saved {} keys to {}",
                snapshot.entries.len(),
                path
            ),
        },
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
