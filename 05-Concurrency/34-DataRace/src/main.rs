//! # Reflection Questions:
//!
//! # What is the purpose of using Mutex in this lab?
//!
//! The purpose of using `Mutex` in this lab is to synchronize access to shared
//! data across multiple threads. In a multithreaded context, data races can occur
//! when two or more threads access the same memory location concurrently, and at
//! least one of the accesses is for writing. `Mutex` provides mutual exclusion,
//! ensuring that only one thread can access the guarded data at a time, thereby
//! preventing data races and ensuring thread safety. The lab exercises demonstrate
//! the use of `Mutex` in practical scenarios to teach how Rust's safety features
//! lead to more secure, robust, and efficient code, especially in concurrent
//! applications.
//!
//! # How does Rust prevent data races when using Mutex?
//!
//! Rust prevents data races when using `Mutex` through its ownership and type
//! system. When a thread attempts to access the data guarded by a `Mutex`, it
//! must first acquire the lock. The `lock` method returns a `MutexGuard`, which
//! is a smart pointer that provides both scoped and exclusive access to the
//! underlying data. The Rust compiler ensures that the `MutexGuard` cannot be
//! copied or sent to other threads, and the `MutexGuard` must be released
//! (dropped) before another thread can acquire the lock. This mechanism ensures
//! that only one thread at a time can mutate the protected data, thereby
//! preventing simultaneous access that could lead to data races.
//!
//! # What happens when you try to compile the second block of code, and why?
//!
//! Compiling the second block of code that attempts to use a `Mutex` without
//! `Arc` in a multithreaded context will result in a compilation error. The error
//! occurs because `Mutex<T>` alone does not implement `Clone`, and without `Arc`,
//! it cannot be safely shared between multiple threads. Rust's ownership rules
//! stipulate that once the `Mutex` is moved into the first thread, it cannot be
//! used or accessed again in the main thread or any other threads, as that would
//! violate Rust's guarantees of no data races. The compiler enforces these rules
//! at compile time, preventing the program from building if it finds that the
//! `Mutex` could potentially be accessed concurrently without proper synchronization,
//! which `Arc` provides. The error points to the line where the `data` Mutex is moved
//! into the closure for the first iteration of the loop. Rust's ownership rules state
//! that once a value has been moved, it is no longer available for use again as it does
//! not have the `Copy` trait. Since a `Mutex` does not implement `Copy`, attempting to move
//! it into multiple closures will result in a compilation error. Each iteration
//! of the loop attempts to move the same `Mutex` into a new closure, but after
//! the first move, `data` is no longer available to be moved again. This
//! enforces at compile time the guarantee that there will be no data races due to
//! multiple ownership of the `Mutex`.
//!
//! # Challenge Questions:
//!
//! # Can you modify the code to use read-write locks (RwLock) instead of a Mutex?
//! # What are the advantages and disadvantages?
//!
//! Yes, the code can be modified to use `RwLock` in place of `Mutex`. `RwLock`
//! allows multiple readers or at most one writer at any point in time, which can
//! lead to higher performance in scenarios where reads are frequent and writes
//! are infrequent.
//!
//! Advantages of using `RwLock`:
//! - Increased concurrency in read-heavy scenarios since multiple threads can
//!   read the data simultaneously without blocking each other.
//! - More granular control over access, allowing optimization of access patterns.
//!
//! Disadvantages of using `RwLock`:
//! - `RwLock` can be more complex than `Mutex` because it requires managing two
//!   different kinds of locks (read and write), which can lead to potential
//!   deadlocks if not handled correctly (e.g., reader and writer starvation).
//! - `RwLock` may have higher overhead than `Mutex` due to this increased
//!   complexity.
//!
//! Here is the modified code using `RwLock` instead of `Mutex`:
//!
//! ```rust
//! use std::sync::{Arc, RwLock};
//! use std::thread;
//!
//! fn main() {
//!     let data = Arc::new(RwLock::new(0));
//!     let mut handles = vec![];
//!
//!     for _ in 0..10 {
//!         let data_clone = Arc::clone(&data);
//!         let handle = thread::spawn(move || {
//!             let mut num = data_clone.write().unwrap();
//!             *num += 1;
//!         });
//!         handles.push(handle);
//!     }
//!
//!     for handle in handles {
//!         handle.join().unwrap();
//!     }
//!
//!     println!("Result: {}", *data.read().unwrap());
//! }
//! ```
//!
//! In this example, `RwLock` is used to guard the data, allowing for multiple
//! concurrent read accesses or exclusive write access by only one thread at a
//! time. Note that in this particular case, since the threads are only writing,
//! there is no advantage over `Mutex` as the `RwLock` will only allow one writer
//! at a time, similar to a `Mutex`. The benefit of `RwLock` would be more apparent
//! in a scenario with both reads and writes where reads are more frequent.
//!
//! # How would you handle potential deadlocks in a more complex application that
//! uses multiple Mutexes?
//!
//! Rust provides several mechanisms and patterns that can be used to apply
//! deadlock avoidance strategies:
//!
//! - **Lock Ordering**:
//!   Define a clear order for acquiring multiple `Mutexes` and consistently
//!   follow this order throughout the application. This can be done by
//!   establishing a convention or by wrapping the `Mutexes` in a struct that
//!   enforces the order of acquisition.
//!
//! - **Timeouts**:
//!   Use the `try_lock_for` or `try_lock_until` methods provided by `std::sync::Mutex`
//!   to attempt to acquire a lock with a timeout. This way, if the lock cannot
//!   be acquired within the timeout period, the thread can take alternate
//!   actions, such as retrying or aborting the operation.
//!
//! - **Lock Hierarchies**:
//!   Implement a lock hierarchy by designing your system in layers or modules,
//!   where each layer can only access locks in its own layer or lower layers.
//!
//! - **Deadlock Detection**:
//!   Rust does not provide built-in deadlock detection, but you can create
//!   monitoring tools that track lock acquisition and release, and check for
//!   cycles in lock dependencies. This is advanced and typically not necessary
//!   if other preventative measures are in place.
//!
//! - **Reducing Granularity**:
//!   Refactor code to use broader locks where appropriate, or use data
//!   structures that do not require fine-grained locks. This simplifies the
//!   locking logic and can prevent deadlocks, but it may also reduce
//!   concurrency.
//!
//! - **Avoid Locking**:
//!   Use `std::cell::RefCell` and `std::cell::Cell` for single-threaded scenarios
//!   or `std::sync::atomic` for lock-free programming in multithreaded scenarios.
//!
//! - **Ownership Transfer**:
//!   Use channels (from `std::sync::mpsc` or external crates like `crossbeam`)
//!   to pass data between threads, which avoids shared state and the associated
//!   locking. This is in line with Rust's ownership model and the actor
//!   concurrency pattern.
//!
//! These strategies should be applied within the context of Rust's ownership and
//! borrowing rules, leveraging the compiler's checks to maintain safe concurrency.
//! Careful design and understanding of Rust's concurrency primitives are essential
//! to effectively prevent deadlocks.


// use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering::Relaxed;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let vector = Mutex::new(Vec::new());
//     static TURN: AtomicBool = AtomicBool::new(true);
//     static STOP: AtomicBool = AtomicBool::new(false);

//     thread::scope(|scope| {
//         scope.spawn(|| loop {
//             if TURN.load(Relaxed) {
//                 let mut vector = vector.lock().unwrap();
//                 vector.push(1);
//                 TURN.store(false, Relaxed);
//             }
//             if STOP.load(Relaxed) {
//                 break;
//             }
//         });

//         scope.spawn(|| loop {
//             if !TURN.load(Relaxed) {
//                 let mut vector = vector.lock().unwrap();
//                 vector.push(2);
//                 TURN.store(true, Relaxed);
//             }
//             if STOP.load(Relaxed) {
//                 break;
//             }
//         });

//         scope.spawn(|| {
//             thread::sleep(std::time::Duration::from_nanos(1));
//             STOP.store(true, Relaxed);
//         });
//     });

//     println!("{:?}", vector.lock().unwrap());
// }

// use std::sync::{Arc, RwLock};
// use std::thread;

// fn main() {
//     // Challenge(1): modify the code to use read-write locks instead of Mutex
//     let data = Arc::new(RwLock::new(vec![1, 2, 3]));

//     let handles: Vec<_> = (0..3).map(|i| {
//         let data = data.clone();
//         thread::spawn(move || {
//             let mut data = data.write().unwrap();
//             data[i] += 1;
//         })
//     }).collect();

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("{:?}", data);
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let data = Arc::new(Mutex::new(vec![1, 2, 3]));

//     let handles: Vec<_> = (0..3).map(|i| {
//         let data = data.clone();
//         thread::spawn(move || {
//             let mut data = data.lock().unwrap();
//             data[i] += 1;
//         })
//     }).collect();

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("{:?}", data);
// }

use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

// Challenge(3): modify the code to use condition variables
fn main() {
    static MAX_ITEMS: usize = 3;
    let data = Arc::new(Mutex::new(Vec::new()));
    let not_empty = Arc::new(Condvar::new());

    let mut handles = Vec::new();
    for i in 0..MAX_ITEMS {
        let thr = thread::spawn({
            let data = data.clone();
            let not_empty = not_empty.clone();
            move || {
                let mut data = data.lock().unwrap();
                let item = loop {
                    if let Some(item) = data.get(i) {
                        break item;
                    } else {
                        data = not_empty.wait(data).unwrap();
                    }
                };
                data[i] = item + 1;
            }
        });
        handles.push(thr);
    }

    for i in 1..=MAX_ITEMS {
        data.lock().unwrap().push(i);
        not_empty.notify_one();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}

/*
// Mutex that protects the data vector, and then we spawn three threads 
//that each acquire a lock on the mutex and modify an element of the vector.

use std::sync::Mutex;
use std::thread;

fn main() {
    let data = Mutex::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}

*/

// use std::thread;

// fn main() {
//     let mut data = vec![1, 2, 3];

//     for i in 0..3 {
//         // Try to capture a mutable reference in multiple threads
//         // This will fail to compile!
//         thread::spawn(move || {
//             data[i] += 1;
//         });
//     }

//     // No data race can occur, this will not compile.
// }