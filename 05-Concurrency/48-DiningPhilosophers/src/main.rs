//! Reflection Questions:
//! 
//! # How does Mutex help in safely sharing the fork between philosophers?
//!
//! In the context of the dining philosophers problem, a `Mutex` is used to ensure
//! safe sharing of forks between philosophers (threads) to avoid race conditions 
//! and ensure that no two philosophers can hold the same fork at the same time.
//!
//! Here's how `Mutex` contributes to thread safety:
//!
//! - **Exclusive Access**: The `Mutex` (mutual exclusion) primitive ensures that 
//!   only one philosopher at a time can hold a `lock` on a fork. When a 
//!   philosopher tries to pick up a fork, they must acquire the `lock`. If the 
//!   `lock` is already held by another philosopher, the current philosopher must 
//!   wait until the fork is available (i.e., the `lock` is released).
//!
//! - **Prevention of Data Races**: By using a `Mutex`, we ensure that the state 
//!   of the fork (whether it is being used or not) cannot be accessed by multiple 
//!   philosophers concurrently, thus preventing data races.
//!
//! - **Deadlock Avoidance**: In the dining philosophers problem, strategies such 
//!   as acquiring forks in a particular order (e.g., lower-numbered forks first) 
//!   can prevent deadlocks, where two or more philosophers hold one fork each and 
//!   wait indefinitely for the other. The `Mutex` alone does not prevent 
//!   deadlocks, but it is used as part of the strategy to do so.
//!
//! - **Atomicity**: `Mutex` provides atomicity for the operations of acquiring 
//!   and releasing the fork. When a philosopher finishes eating, they release 
//!   both forks at once, ensuring that atomicity is maintained.
//!
//! In summary, `Mutex` helps maintain the integrity of shared resources in 
//! concurrent programming by ensuring that only one thread can access the 
//! resource at a time, thus providing a foundation for building safe and 
//! deadlock-free concurrent applications.
//! 
//! # How is deadlock avoided in this example?
//!
//! Deadlock is avoided in this implementation of the dining philosophers problem
//! by employing a strategy that prevents all four conditions required for a
//! deadlock from occurring simultaneously. The four conditions are mutual
//! exclusion, hold and wait, no preemption, and circular wait.
//!
//! The key technique used to avoid deadlock in this example is:
//!
//! - **Even/Odd Ordering**: To prevent a situation where all philosophers pick up
//!   one fork and then wait for the other (which would lead to deadlock), the
//!   philosophers are divided into two groups based on their ID (even or odd).
//!   Even-numbered philosophers pick up the left fork first, and odd-numbered
//!   ones pick up the right fork first. This further reduces the chance of
//!   entering a circular wait state.
//!
//! These approaches, in combination with the use of `Mutex` to ensure exclusive
//! access to the forks (mutual exclusion), help to prevent deadlock in the dining
//! philosophers problem. By ensuring that at least one of the necessary conditions
//! for deadlock cannot occur, the system is made deadlock-free. 
//! 

/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::{Duration, Instant};

// struct Fork {
//     id: u32,
//     mutex: Mutex<()>,
// }

// struct Philosopher {
//     id: u32,
//     name: String,
//     left_fork: Arc<Fork>,
//     right_fork: Arc<Fork>,
// }

// impl Philosopher {
//     fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
//         Philosopher {
//             id,
//             name: name.to_string(),
//             left_fork,
//             right_fork,
//         }
//     }

//     fn eat(&self) {
//         let (first_fork, second_fork) = if self.id % 2 == 0 {
//             (&self.left_fork, &self.right_fork)
//         } else {
//             (&self.right_fork, &self.left_fork)
//         };

//         let _first_guard = first_fork.mutex.lock().unwrap();
//         println!("{} picked up fork {}.", self.name, first_fork.id);
//         let _second_guard = second_fork.mutex.lock().unwrap();
//         println!("{} picked up fork {}.", self.name, second_fork.id);

//         println!("{} is eating.", self.name);
//         thread::sleep(Duration::from_secs(3));
//         println!("{} finished eating.", self.name);

//         println!("{} put down fork {}.", self.name, first_fork.id);
//         println!("{} put down fork {}.", self.name, second_fork.id);
//     }
// }

// fn main() {
//     println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

//     //we only have 4 forks at the table
//     let forks = (0..4)
//         .map(|id| {
//             Arc::new(Fork {
//                 id,
//                 mutex: Mutex::new(()),
//             })
//         })
//         .collect::<Vec<_>>();

//     let philosophers = vec![
//         ("Jürgen Habermas", 0, 1),
//         ("Friedrich Engels", 1, 2),
//         ("Karl Marx", 2, 3),
//         ("Thomas Piketty", 3, 0),
//         ("Michel Foucault", 0, 1),
//         ("Socrates", 1, 2),
//         ("Plato", 2, 3),
//         ("Aristotle", 3, 0),
//         ("Pythagoras", 0, 1),
//         ("Heraclitus", 1, 2),
//         ("Democritus", 2, 3),
//         ("Diogenes", 3, 0),
//         ("Epicurus", 0, 1),
//         ("Zeno of Citium", 1, 2),
//         ("Thales of Miletus", 2, 3),
//     ]
//     .into_iter()
//     .enumerate()
//     .map(|(id, (name, left, right))| {
//         Philosopher::new(
//             id as u32,
//             name,
//             Arc::clone(&forks[left]),
//             Arc::clone(&forks[right]),
//         )
//     })
//     .collect::<Vec<_>>();

//     let start = Instant::now();

//     let handles = philosophers
//         .into_iter()
//         .map(|philosopher| {
//             thread::spawn(move || {
//                 philosopher.eat();
//             })
//         })
//         .collect::<Vec<_>>();

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Total time: {:?}", start.elapsed());sdfj78

// }

use dining_philosophers::start_dining;
fn main() {
    start_dining();
}