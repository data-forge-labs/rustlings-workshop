//! Reflection Questions
//!
//! # How does Rust provide memory safety in concurrent programs?
//!
//! Rust provides memory safety in concurrent programs through its ownership and
//! borrowing rules, which are enforced at compile time. These rules ensure that
//! data races, which can lead to undefined behavior, are prevented. Here are
//! the key mechanisms that contribute to memory safety during concurrency:
//!
//! - **Ownership and Borrowing**: Rust's ownership system ensures that each
//!   piece of data has a single owner at any given time and that only one
//!   mutable reference or any number of immutable references to a resource
//!   exist. This eliminates concurrent write and write-read access.
//!
//! - **Lifetime Annotations**: Lifetimes are used to explicitly specify how
//!   long references to data should be valid. The compiler uses these
//!   annotations to enforce that data is not accessed after it has been freed,
//!   preventing use-after-free bugs.
//!
//! - **Type System and Safe Abstractions**: Rust's type system allows creating
//!   safe abstractions like `Arc<T>` for atomic reference counting and
//!   `Mutex<T>` for mutual exclusion. These abstractions enable safe shared
//!   state management across threads without risking data races.
//!
//! - **Zero-Cost Abstractions**: Rust provides abstractions like channels
//!   (`std::sync::mpsc`) for message passing and thread spawning APIs that
//!   abstract away the details of thread management, allowing developers to
//!   focus on the logic rather than the safety of the concurrency model.
//!
//! - **No Data Races Guarantee**: By combining the ownership rules with the
//!   type system, Rust guarantees at compile time that there will be no data
//!   races. This is a stark contrast to many other languages where such issues
//!   might only be discovered at runtime.
//!
//! Rust's approach to memory safety in concurrency not only prevents common
//! errors but also encourages developers to think about the structure and
//! design of concurrent code, leading to more robust and maintainable systems.
//!
//! # When is message passing preferable to shared mutable state?
//!
//! Message passing is a communication method where components exchange data
//! through messages instead of directly sharing the underlying state. This
//! approach fits more naturally than shared state mechanisms like mutexes in
//! several scenarios:
//!
//! 1. **Distributed Systems**: When components are spread across different
//!    servers or geographical locations, message passing is often the only viable
//!    communication method, as shared memory is not practical.
//!
//! 2. **Microservices Architecture**: Microservices are designed to be autonomous
//!    and communicate over the network, making message passing through APIs or
//!    event streams the standard approach.
//!
//! 3. **Actor-Based Concurrency**: In the actor model, each actor encapsulates
//!    its state and communicates with other actors exclusively through message
//!    passing, which simplifies concurrency and system design.
//!
//! 4. **State Machine Implementations**: Systems modeled as state machines react
//!    to incoming events (messages) and transition between states, which is a
//!    natural fit for message passing paradigms.
//!
//! 5. **Real-time Event Processing**: Systems that process streams of real-time
//!    events, such as sensor data or user interactions, can benefit from message
//!    passing to handle these events asynchronously.
//!
//! 6. **Serverless Computing**: In serverless architectures, functions are
//!    triggered by events (messages), and there is typically no shared state
//!    between invocations.
//!
//! 7. **Inter-process Communication (IPC)**: When different processes on the same
//!    machine need to communicate, message passing can be used to avoid the
//!    complexity of synchronizing access to shared memory.
//!
//! 8. **Parallel Processing**: When tasks can be processed in parallel without
//!    shared state, message passing can be used to distribute tasks to worker
//!    processes or threads.
//!
//! In each of these scenarios, message passing is favored due to its ability to
//! facilitate communication without shared state, thereby avoiding the complexity
//! and potential pitfalls of synchronization primitives like mutexes.
//!
//! # What causes a deadlock and how can it be avoided?
//!
//! A deadlock is a state in a concurrent system where two or more processes are
//! unable to proceed because each is waiting for the other to release a resource
//! they need. Deadlocks are caused by the following four conditions occurring
//! simultaneously:
//!
//! 1. **Mutual Exclusion**: At least one resource must be held in a non-sharable
//!    mode, meaning only one process can use the resource at any given time.
//!
//! 2. **Hold and Wait**: A process is holding at least one resource and waiting
//!    to acquire additional resources held by other processes.
//!
//! 3. **No Preemption**: Resources cannot be forcibly removed from the processes
//!    that are holding them until the resources are used to completion.
//!
//! 4. **Circular Wait**: A closed chain of processes exists, where each process
//!    holds at least one resource needed by the next process in the chain.
//!
//! To avoid deadlocks, one or more of the above conditions must be prevented:
//!
//! - **Preventing Mutual Exclusion**: This is not always possible since some
//!   resources, like printers or files, inherently require exclusive access.
//!
//! - **Preventing Hold and Wait**: Ensure that a process requests all required
//!   resources at once, releasing all of its currently held resources if any are
//!   unavailable, and then retrying.
//!
//! - **Preventing No Preemption**: Allow the system to force processes to release
//!   resources if those resources are needed by other processes with higher
//!   priority.
//!
//! - **Preventing Circular Wait**: Impose a total ordering of all resource types,
//!   and require each process to request resources in an increasing order of
//!   enumeration.
//!
//! Other strategies to avoid deadlocks include:
//!
//! - **Deadlock Detection and Recovery**: Allow the system to enter a deadlock
//!   state but have a mechanism to detect and recover from it.
//!
//! - **Deadlock Ignorance**: Simply ignore the problem altogether, which can be
//!   an option in systems where deadlocks are so rare that it's not cost-effective
//!   to address them.
//!
//! By using a combination of these strategies, systems can be designed to either
//! prevent deadlocks or recover from them when they do occur.
//!
//! # In what scenarios have you used or could use concurrency?
//!
//! Concurrency can be used in a wide range of scenarios to improve the performance
//! and responsiveness of software systems by allowing multiple operations to
//! progress simultaneously. Some common scenarios include:
//!
//! - **Web Servers**: Handling multiple client requests in parallel to maximize
//!   throughput and reduce response times.
//!
//! - **User Interfaces**: Maintaining a responsive UI by performing long-running
//!   tasks (like I/O operations or computation) in the background.
//!
//! - **Data Processing**: Processing large datasets or performing complex
//!   calculations using parallel algorithms to reduce execution time.
//!
//! - **Real-Time Systems**: Managing multiple tasks that must respond to events
//!   in real-time, such as in embedded systems or gaming.
//!
//! - **Networked Applications**: Concurrently managing connections, data transfer,
//!   and protocol handling in client-server or peer-to-peer applications.
//!
//! - **Financial Systems**: Executing and settling trades concurrently in stock
//!   exchanges to handle high-frequency trading volumes.
//!
//! - **Multimedia Applications**: Encoding or decoding audio and video streams in
//!   real-time where different streams or parts of streams are processed in
//!   parallel.
//!
//! - **Batch Processing**: Running multiple batch jobs in parallel to maximize
//!   the utilization of CPU and I/O resources.
//!
//! - **Distributed Computing**: Coordinating tasks across distributed systems,
//!   such as in cloud computing and big data analysis, where tasks are executed
//!   concurrently on multiple nodes.
//!
//! Concurrency is a powerful tool that, when used appropriately, can significantly
//! enhance the performance and efficiency of software systems.
//!
//! # How could parallelism speed up an operation in your code?
//!
//! Parallelism can speed up operations in code by dividing a task into smaller
//! subtasks and executing them concurrently across multiple processing units like
//! CPU cores. This can lead to a reduction in overall execution time, particularly
//! for compute-intensive or data-intensive tasks. Here are some ways in which
//! parallelism can be applied to speed up operations:
//!
//! - **Data Parallelism**: If an operation can be performed on a collection of
//!   data independently, it can be split across multiple threads or processes.
//!   For example, applying a function to each item in an array can be done in
//!   parallel, with each thread processing a subset of the array.
//!
//! - **Task Parallelism**: Different independent tasks or stages of a pipeline
//!   can be executed in parallel. For instance, reading from disk, processing
//!   data, and writing results can occur simultaneously in different threads.
//!
//! - **Batch Processing**: Large batch jobs can be broken down into smaller units
//!   that are processed in parallel, thus reducing the time to completion.
//!
//! - **Computational Algorithms**: Algorithms that involve a lot of calculations,
//!   such as simulations, optimizations, or machine learning training, can
//!   often be parallelized to take advantage of multiple processors.
//!
//! - **Concurrency in I/O Operations**: Parallelism can be used to overlap I/O
//!   operations with computation. While one thread waits for I/O, others can
//!   continue with computation, leading to better resource utilization.
//!
//! By utilizing parallelism effectively, you can often achieve significant
//! performance improvements, particularly in systems with multi-core processors.
//! However, it's important to note that not all operations can be parallelized,
//! and there is overhead associated with coordinating parallel tasks, so the
//! benefits need to be weighed against these factors.
//!

use std::sync::mpsc;
use std::thread;

fn main() {
    // Challenge(1): Use threads and channels to pass messages between concurrent tasks.
    // This examples shows how to use channels to divide work among multiple threads.

    // Create a new channel
    let (tx, rx) = mpsc::channel();

    let numbers_to_add = (1..=100).collect::<Vec<u32>>();
    let number_of_threads = 10;
    let chunk_size = numbers_to_add.len() / number_of_threads;

    // Make owned chunks to move into the threads
    let chunks = numbers_to_add
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<u32>>>();

    for chunk in chunks {
        let tx = tx.clone();
        thread::spawn(move || {
            let sum = chunk.into_iter().sum::<u32>();
            tx.send(sum).expect("To send the partial sum");
        });
    }

    // Close the channel
    drop(tx);

    let mut sum = 0;
    // Receive messages from the channel
    for received in rx {
        println!("Partial sum: {}", received);
        sum += received;
    }

    println!("Final sum: {}", sum);
}
