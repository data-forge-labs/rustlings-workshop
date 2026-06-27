use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub fn with_mutex<F: FnOnce(&mut i32) -> R, R>(m: &Mutex<i32>, f: F) -> R {
    let mut guard = m.lock();
    f(&mut guard)
}

pub fn try_with_mutex<F: FnOnce(&mut i32) -> R, R>(m: &Mutex<i32>, f: F) -> Option<R> {
    match m.try_lock() {
        Some(mut guard) => Some(f(&mut guard)),
        None => None,
    }
}

pub fn update_counter(counter: &Mutex<i32>, delta: i32) -> i32 {
    let mut guard = counter.lock();
    *guard += delta;
    *guard
}

pub fn benchmark_parking_lot_vs_std(iterations: u32) -> Duration {
    let parking = Arc::new(Mutex::new(0i32));
    let std_mutex = Arc::new(std::sync::Mutex::new(0i32));

    let start = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..4 {
        let p = Arc::clone(&parking);
        handles.push(std::thread::spawn(move || {
            for _ in 0..iterations {
                *p.lock() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let _ = start.elapsed();
    let start2 = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..4 {
        let s = Arc::clone(&std_mutex);
        handles.push(std::thread::spawn(move || {
            for _ in 0..iterations {
                *s.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    start2.elapsed()
}

pub fn read_under_rwlock<F: FnOnce(&i32) -> R, R>(
    lock: &parking_lot::RwLock<i32>,
    f: F,
) -> R {
    let guard = lock.read();
    f(&guard)
}

pub fn write_under_rwlock<F: FnOnce(&mut i32) -> R, R>(
    lock: &parking_lot::RwLock<i32>,
    f: F,
) -> R {
    let mut guard = lock.write();
    f(&mut guard)
}

pub fn crossbeam_send(tx: &crossbeam_channel::Sender<i32>, value: i32) -> Result<(), &'static str> {
    tx.send(value).map_err(|_| "send failed")
}

pub fn crossbeam_collect(rx: crossbeam_channel::Receiver<i32>, n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        match rx.recv() {
            Ok(v) => result.push(v),
            Err(_) => break,
        }
    }
    result
}

pub fn arc_swap_load(swap: &arc_swap::ArcSwap<String>) -> arc_swap::Guard<Arc<String>> {
    swap.load()
}

pub fn arc_swap_store(swap: &arc_swap::ArcSwap<String>, value: String) {
    swap.store(Arc::new(value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::RwLock;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    mod step_01_parking_lot_mutex {
        use super::*;

        #[test]
        fn test_with_mutex() {
            let m = Mutex::new(10);
            let r = with_mutex(&m, |v| {
                *v += 5;
                *v
            });
            assert_eq!(r, 15);
            assert_eq!(*m.lock(), 15);
        }

        #[test]
        fn test_try_with_mutex_uncontended() {
            let m = Mutex::new(1);
            let r = try_with_mutex(&m, |v| *v * 2);
            assert_eq!(r, Some(2));
        }

        #[test]
        fn test_update_counter() {
            let m = Mutex::new(0);
            assert_eq!(update_counter(&m, 5), 5);
            assert_eq!(update_counter(&m, -2), 3);
        }

        #[test]
        fn test_parking_lot_concurrent() {
            let m = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            for _ in 0..10 {
                let m = Arc::clone(&m);
                handles.push(thread::spawn(move || {
                    for _ in 0..100 {
                        update_counter(&m, 1);
                    }
                }));
            }
            for h in handles {
                h.join().unwrap();
            }
            assert_eq!(*m.lock(), 1000);
        }
    }

    mod step_02_rwlock {
        use super::*;

        #[test]
        fn test_read_under_rwlock() {
            let l = RwLock::new(42);
            let v = read_under_rwlock(&l, |x| *x);
            assert_eq!(v, 42);
        }

        #[test]
        fn test_write_under_rwlock() {
            let l = RwLock::new(0);
            write_under_rwlock(&l, |v| *v = 100);
            assert_eq!(*l.read(), 100);
        }

        #[test]
        fn test_rwlock_multi_reader() {
            let l = Arc::new(RwLock::new(0i32));
            let readers_done = Arc::new(AtomicUsize::new(0));
            let mut handles = vec![];
            for _ in 0..4 {
                let l = Arc::clone(&l);
                let readers_done = Arc::clone(&readers_done);
                handles.push(thread::spawn(move || {
                    let _ = read_under_rwlock(&l, |v| *v);
                    readers_done.fetch_add(1, Ordering::SeqCst);
                }));
            }
            for h in handles {
                h.join().unwrap();
            }
            assert_eq!(readers_done.load(Ordering::SeqCst), 4);
        }
    }

    mod step_03_crossbeam {
        use super::*;

        #[test]
        fn test_crossbeam_send_recv() {
            let (tx, rx) = crossbeam_channel::unbounded();
            assert!(crossbeam_send(&tx, 42).is_ok());
            let out = crossbeam_collect(rx, 1);
            assert_eq!(out, vec![42]);
        }

        #[test]
        fn test_crossbeam_mpmc() {
            let (tx, rx) = crossbeam_channel::unbounded::<i32>();
            let mut handles = vec![];
            for i in 0..3 {
                let tx = tx.clone();
                handles.push(thread::spawn(move || {
                    for j in 0..10 {
                        tx.send(i * 10 + j).unwrap();
                    }
                }));
            }
            drop(tx);
            let out = crossbeam_collect(rx, 30);
            assert_eq!(out.len(), 30);
            let mut sorted = out.clone();
            sorted.sort();
            assert_eq!(sorted, (0..30).collect::<Vec<_>>());
        }
    }

    mod step_04_arc_swap {
        use super::*;

        #[test]
        fn test_arc_swap_store_load() {
            let s = arc_swap::ArcSwap::from(Arc::new("v1".to_string()));
            arc_swap_store(&s, "v2".to_string());
            let g = arc_swap_load(&s);
            assert_eq!(**g, "v2");
        }

        #[test]
        fn test_arc_swap_readers_dont_block() {
            let s = Arc::new(arc_swap::ArcSwap::from(Arc::new("init".to_string())));
            let reads = Arc::new(AtomicUsize::new(0));
            let mut handles = vec![];
            for _ in 0..4 {
                let s = Arc::clone(&s);
                let reads = Arc::clone(&reads);
                handles.push(thread::spawn(move || {
                    for _ in 0..100 {
                        let g = arc_swap_load(&s);
                        let _ = (**g).len();
                        reads.fetch_add(1, Ordering::SeqCst);
                    }
                }));
            }
            for i in 0..10 {
                arc_swap_store(&s, format!("v{}", i));
            }
            for h in handles {
                h.join().unwrap();
            }
            assert_eq!(reads.load(Ordering::SeqCst), 400);
        }
    }
}
