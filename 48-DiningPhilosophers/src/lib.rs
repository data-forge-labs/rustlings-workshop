//! This is an alternative implementation of the classic
//! [Dining Philosophers](https://lass.cs.umass.edu/~shenoy/courses/fall13/lectures/Lec10_notes.pdf)
//! problem.
//!
//! The problem is a classic example of a deadlock, which occurs when multiple
//! processes are blocked, each waiting for the other to release a resource. In
//! this case, the philosophers are blocked because they are each waiting for
//! both forks, and the forks are being held by other philosophers.
//!
//! The 4 conditions that must be satisfied in order to have a deadlock are:
//!
//! 1. Mutual exclusion: At least one resource must be held in a non-shareable
//!    mode.
//! 2. Hold and wait: A process must be holding at least one resource and
//!    waiting for another resource to be released.
//! 3. No preemption: The resources being held by the processes cannot be
//!    preempted by another process.
//! 4. Circular wait: There must be a circular chain of processes, each holding
//!    a resource required by the next process in the chain.
//!
//! # How is deadlock avoided in this example?
//!
//! Deadlock is avoided in two different ways by the `get_forks_try_lock` and
//! `get_forks_odd_even` functions in the dining philosophers example. Each
//! function breaks different conditions that are necessary for a deadlock.
//!
//! ## get_forks_try_lock
//! The `get_forks_try_lock` function uses a non-blocking approach to acquire
//! locks on the forks. If a philosopher cannot obtain a lock on both forks, the
//! function returns `None`, and the philosopher does not eat at that time.
//!
//! - **No Hold and Wait**: Philosophers do not wait for forks; they either
//!   obtain both immediately or none at all, breaking the hold and wait
//!   condition.
//! - **No Circular Wait**: By not waiting for forks to become available, this
//!   approach also breaks circular wait.
//!
//! ## get_forks_odd_even
//! The `get_forks_odd_even` function enforces an order in which forks are
//! picked up based on the philosopher's ID. Philosophers with even IDs pick up
//! the left fork first, and those with odd IDs pick up the right fork first.
//!
//! - **Breaking Circular Wait**: By having a defined order of acquiring forks,
//!   this function prevents the circular wait condition. There cannot be a cycle
//!   of philosophers each waiting for the next one's fork because the order
//!   prevents such a closed chain from forming.
//!
//! Both functions effectively prevent deadlocks by ensuring at least one of the
//! necessary conditions for a deadlock cannot occur in the system.
//! 

use rand::Rng;
use std::{
    collections::HashMap,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

#[derive(PartialEq)]
enum ForkState {
    Taken,
    Free,
}

struct Fork {
    pub id: u32,
    state: ForkState,
}

impl Fork {
    pub fn take(&mut self) {
        match self.state {
            ForkState::Free => self.state = ForkState::Taken,
            ForkState::Taken => panic!("Fork is already taken"),
        }
    }

    pub fn free(&mut self) {
        self.state = ForkState::Free;
    }
}

struct Philosopher {
    id: u32,
    name: String,
    left_hand: Arc<Mutex<Fork>>,
    right_hand: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn eat(&self) {
        let (mut first_fork, mut second_fork) = match self.get_forks_odd_even() {
            Some(value) => value,
            None => return,
        };

        first_fork.take();
        println!("{} is taking first fork {}.", self.name, first_fork.id);

        second_fork.take();
        println!("{} is taking second fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        first_fork.free();
        second_fork.free();
    }

    #[allow(unused)]
    fn get_forks_try_lock(&self) -> Option<(std::sync::MutexGuard<Fork>, std::sync::MutexGuard<Fork>)> {
        let mut left_fork = match self.left_hand.try_lock() {
            Ok(left_fork) => left_fork,
            Err(_) => return None,
        };

        let mut right_fork = match self.right_hand.try_lock() {
            Ok(right_fork) => right_fork,
            Err(_) => {
                left_fork.free();
                return None;
            }
        };
        Some((left_fork, right_fork))
    }
    
    #[allow(unused)]
    fn get_forks_odd_even(&self) -> Option<(std::sync::MutexGuard<Fork>, std::sync::MutexGuard<Fork>)> {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (self.left_hand.lock().unwrap(), self.right_hand.lock().unwrap())
        } else {
            (self.right_hand.lock().unwrap(), self.left_hand.lock().unwrap())
        };
        Some((first_fork, second_fork))
    }

    fn think(&self) {
        let mut rng = rand::thread_rng();
        let time_to_think = rng.gen_range(1..5);
        println!("{} is thinking for {} seconds.", self.name, time_to_think);
        thread::sleep(Duration::from_secs(time_to_think));
    }
}

pub fn start_dining() {
    let running = Arc::new(AtomicBool::new(true));
    let stats = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    set_ctrlc_handler(running.clone(), stats.clone());

    let fork1 = Arc::new(Mutex::new(Fork {
        id: 1,
        state: ForkState::Free,
    }));
    let fork2 = Arc::new(Mutex::new(Fork {
        id: 2,
        state: ForkState::Free,
    }));
    let fork3 = Arc::new(Mutex::new(Fork {
        id: 3,
        state: ForkState::Free,
    }));
    let fork4 = Arc::new(Mutex::new(Fork {
        id: 4,
        state: ForkState::Free,
    }));
    let fork5 = Arc::new(Mutex::new(Fork {
        id: 5,
        state: ForkState::Free,
    }));

    let plato = Philosopher {
        id: 0,
        name: "Plato".to_string(),
        left_hand: fork1.clone(),
        right_hand: fork2.clone(),
    };
    let aristotles = Philosopher {
        id: 1,
        name: "Aristotles".to_string(),
        left_hand: fork2.clone(),
        right_hand: fork3.clone(),
    };
    let phytagoras = Philosopher {
        id: 2,
        name: "Phytagoras".to_string(),
        left_hand: fork3.clone(),
        right_hand: fork4.clone(),
    };
    let democritus = Philosopher {
        id: 3,
        name: "Democritus".to_string(),
        left_hand: fork4.clone(),
        right_hand: fork5.clone(),
    };
    let epicurus = Philosopher {
        id: 4,
        name: "Epicurus".to_string(),
        left_hand: fork5.clone(),
        right_hand: fork1.clone(),
    };

    let phylosophers = vec![plato, aristotles, phytagoras, democritus, epicurus];
    let mut handles = vec![];

    for philosopher in phylosophers {
        let signal = running.clone();
        let stats = stats.clone();
        handles.push(thread::spawn(move || {
            while signal.load(Ordering::Relaxed) {
                philosopher.eat();
                increment_stat(&stats, &philosopher.name);
                philosopher.think();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn increment_stat(stats: &Arc<Mutex<HashMap<String, u32>>>, name: &str) {
    let mut counter = stats.lock().expect("Acquiring lock");
    *counter.entry(name.to_string()).or_insert(0) += 1;
}

fn set_ctrlc_handler(running: Arc<AtomicBool>, stats: Arc<Mutex<HashMap<String, u32>>>) {
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
        println!("Stats: {:?}", stats.lock().unwrap());
        println!("Ctrl-C pressed. Exiting...");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}