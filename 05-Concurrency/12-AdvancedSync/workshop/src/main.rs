use advanced_sync_workshop::{
    arc_swap_load, arc_swap_store, crossbeam_collect, crossbeam_send, read_under_rwlock,
    update_counter, write_under_rwlock, with_mutex,
};
use parking_lot::{Mutex, RwLock};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let m = Mutex::new(10);
    let r = with_mutex(&m, |v| {
        *v += 5;
        *v
    });
    println!("with_mutex returned {}", r);

    let c = Mutex::new(0);
    for _ in 0..1000 {
        update_counter(&c, 1);
    }
    println!("counter = {}", *c.lock());

    let l = RwLock::new(0);
    let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];
    for _ in 0..4 {
        let l = parking_lot::RwLock::new(0);
        handles.push(std::thread::spawn(move || {
            let v = read_under_rwlock(&l, |x| *x);
            println!("read: {}", v);
        }));
    }
    write_under_rwlock(&l, |v| *v = 99);
    std::thread::sleep(std::time::Duration::from_millis(50));
    for h in handles {
        let _ = h.join();
    }

    let (tx, rx) = crossbeam_channel::unbounded();
    crossbeam_send(&tx, 1)?;
    crossbeam_send(&tx, 2)?;
    crossbeam_send(&tx, 3)?;
    drop(tx);
    println!("crossbeam: {:?}", crossbeam_collect(rx, 3));

    let s = arc_swap::ArcSwap::from(std::sync::Arc::new("init".to_string()));
    arc_swap_store(&s, "after".to_string());
    let g = arc_swap_load(&s);
    println!("arc_swap: {}", **g);

    Ok(())
}
