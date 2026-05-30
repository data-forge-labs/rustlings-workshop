//! Reflection Questions:
//!
//! # How much faster was the parallelized version?
//!
//! The parallelized version was between 2x and 3x faster than the sequential version
//! in general. But with the original version that has a vec with only 3 elements
//! the difference is inperceptable, or even the sequential version is faster.
//! Only when the vec was orders of magnitude larger does the parallelized version
//! get faster.
//!
//! |  Number of elements | Sequential time (ms) | Parallel time (ms) | Speedup |
//! |--------------------:|---------------------:|-------------------:|--------:|
//! |             100,000 |                    1 |                  1 |     0.0 |
//! |             200,000 |                    2 |                  2 |     0.0 |
//! |             300,000 |                    3 |                  1 |     3.0 |
//! |             400,000 |                    4 |                  2 |     2.0 |
//! |             500,000 |                    5 |                  2 |     2.5 |
//! |             600,000 |                    7 |                  2 |     3.5 |
//! |             700,000 |                    7 |                  2 |     3.5 |
//! |             800,000 |                    8 |                  2 |     4.0 |
//! |             900,000 |                    9 |                  2 |     4.5 |
//! |           1,000,000 |                   10 |                  3 |     3.3 |
//!
//! # What hyperparameter could you tweak to influence performance?
//!
//! In the context of machine learning and computational algorithms, hyperparameters
//! are adjustable parameters that govern the training process. Tweaking these can
//! significantly influence the performance of the algorithm. Some common
//! hyperparameters that affect performance include:
//!
//! - **Learning Rate**: Determines the size of the steps taken during the
//!   optimization process. Too high can lead to overshooting minima; too low can
//!   cause slow convergence.
//!
//! - **Number of Epochs**: The number of times the learning algorithm will work
//!   through the entire training dataset. More epochs can lead to better
//!   performance but also to overfitting.
//!
//! - **Batch Size**: The number of training samples to work through before the
//!   model's internal parameters are updated. It affects the memory footprint and
//!   can influence the stability of the learning process.
//!
//! - **Number of Hidden Layers and Neurons**: In neural networks, more layers and
//!   neurons can model more complex functions, but they also make the network
//!   prone to overfitting and increase computational cost.
//!
//! - **Regularization Parameter**: Determines the amount of regularization to
//!   apply during training, which can prevent overfitting by introducing a penalty
//!   for complex models.
//!
//! - **Momentum**: Helps accelerate the optimizer in the right direction and
//!   dampens oscillations, potentially leading to faster convergence.
//!
//! - **Early Stopping**: A technique to stop training once the model's
//!   performance ceases to improve on a hold-out validation dataset.
//!
//! - **Feature Selection**: Choosing the right set of features to train on can
//!   significantly affect both the speed of training and the quality of the model.
//!
//! By carefully tuning these and other hyperparameters, you can improve the
//! model's ability to learn from data and generalize to unseen data, ultimately
//! leading to better performance.
//!
//! # What tradeoffs exist when parallelizing algorithms?
//!
//! Parallelizing algorithms involves dividing a task into concurrent subtasks,
//! which can lead to significant performance improvements. However, several
//! tradeoffs must be considered:
//!
//! - **Complexity vs. Speedup**: Parallel algorithms are often more complex to
//!   design, implement, and debug than their sequential counterparts. The speedup
//!   gained must justify this additional complexity.
//!
//! - **Overhead**: Introducing parallelism incurs overhead from thread creation,
//!   synchronization, and communication between threads or processes, which can
//!   sometimes offset the performance benefits.
//!
//! - **Scalability**: As the number of processors increases, the performance gain
//!   may not scale linearly due to bottlenecks such as shared resources or
//!   diminishing returns on subdivided tasks.
//!
//! - **Load Balancing**: Efficiently distributing work among parallel tasks to
//!   avoid idle processors can be challenging, especially when tasks have
//!   unpredictable execution times.
//!
//! - **Consistency and State Management**: Maintaining data consistency across
//!   tasks requires careful management of shared state, which may involve locking
//!   mechanisms that can become a source of contention.
//!
//! - **Resource Utilization**: Parallel algorithms may consume more computational
//!   resources, which could lead to contention with other processes and overall
//!   system performance degradation.
//!
//! - **Amdahl's Law**: The theoretical maximum speedup is limited by the
//!   sequential portion of the task, which means that beyond a certain point,
//!   adding more parallel resources achieves minimal gains.
//!
//! - **Data Locality**: Parallel tasks may not utilize data locality effectively,
//!   leading to increased cache misses and memory access times that can
//!   negatively impact performance.
//!
//! Weighing these tradeoffs is crucial when deciding to parallelize an algorithm,
//! as the goal is to achieve the best performance improvement for the resources
//! invested.
//!
//! # In what scenario would concurrency not improve performance?
//!
//! Concurrency involves multiple computations happening simultaneously, which
//! can improve performance in many scenarios. However, there are situations
//! where concurrency might not lead to performance gains and could even degrade
//! performance:
//!
//! - **I/O-Bound Processes**: If a program is limited by input/output operations,
//!   such as disk access or network communication, adding concurrency to CPU
//!   processing will not improve performance because the bottleneck is the I/O
//!   latency.
//!
//! - **Single-Core Processors**: On single-core processors, true parallelism is
//!   not possible, and concurrent tasks might lead to context switching overhead
//!   without any real performance gain.
//!
//! - **High Contention for Resources**: If concurrent tasks frequently access
//!   shared resources requiring exclusive locks, the overhead of waiting and
//!   context switching can diminish or negate the benefits of concurrency.
//!
//! - **Small Workloads**: For tasks that are already quick to complete, the
//!   overhead of managing concurrency (like thread creation and synchronization)
//!   might outweigh the benefits.
//!
//! - **Poorly Scalable Algorithms**: Some algorithms cannot be efficiently
//!   divided into independent concurrent tasks, resulting in little to no
//!   performance improvement.
//!
//! - **Overhead of Synchronization**: Concurrency requires careful synchronization
//!   to avoid race conditions, and the associated overhead can sometimes outweigh
//!   the performance benefits, especially if not implemented efficiently.
//!
//! - **Amdahl's Law**: If a significant portion of the program must be executed
//!   serially, the benefits of parallel execution of the remaining portion are
//!   limited. Amdahl's Law is formalized by the formula:
//!   ```text
//!   Speedup ≤ 1 / (S + (1 - S) / P)
//!   ```
//!   where `S` is the proportion of execution time that the serial part of the
//!   program takes, and `P` is the number of processors. As `S` approaches 1,
//!   the potential speedup gained from increasing `P` diminishes.
//!
//! In these scenarios, the overhead of managing concurrency or the nature of the
//! workload itself means that a concurrent approach might not result in improved
//! performance and could be counterproductive.
//!
//! # How does Rust help mitigate common pitfalls like race conditions?
//!
//! Rust is designed to provide strong guarantees about memory safety and
//! concurrency, which helps to mitigate common pitfalls like race conditions.
//! It does this through several language features and principles:
//!
//! - **Ownership System**: Rust enforces unique ownership of data by default,
//!   meaning that only one variable can own a piece of data at any one time.
//!   This prevents data from being modified from multiple places simultaneously.
//!
//! - **Borrowing Rules**: Rust's borrowing rules allow either multiple immutable
//!   references (`&T`) or one mutable reference (`&mut T`) to a resource at any
//!   one time. These rules prevent race conditions by ensuring that you cannot
//!   have unsynchronized mutable access to data when it might be read elsewhere.
//!
//! - **Lifetime Tracking**: Lifetimes ensure that references to data do not
//!   outlive the data they point to, which helps prevent use-after-free bugs that
//!   can lead to race conditions in concurrent contexts.
//!
//! - **No Null Pointers**: Rust's type system ensures that references are always
//!   valid (unless explicitly using unsafe code), reducing the risk of null
//!   pointer dereferences and subsequent race conditions.
//!
//! - **Thread Safety Guarantees**: Rust's type system and trait bounds can
//!   enforce thread safety at compile time. The `Send` and `Sync` traits
//!   indicate whether it is safe to transfer ownership of values across threads
//!   or reference values from multiple threads, respectively.
//!
//! - **Concurrency Primitives**: Rust provides concurrency primitives like
//!   `Mutex`, `RwLock`, and `Atomic` types in the standard library, which are
//!   designed to provide safe concurrent access to data.
//!
//! - **Immutable by Default**: Data in Rust is immutable by default, meaning
//!   you must explicitly mark it as mutable with the `mut` keyword. This reduces
//!   the risk of unintended data modification.
//!
//! - **Compile-time Error Checking**: Rust's compiler checks for common
//!   concurrency issues at compile time, preventing code with potential race
//!   conditions from compiling.
//!
//! These features of Rust work together to prevent race conditions and other
//! concurrency issues before the code is even run, leading to more robust and
//! reliable software.
//!
//! # Explore num_threads() setting - what is the impact?
//!
//! The `num_threads()` setting in Rayon's `ThreadPoolBuilder` allows you to
//! specify the number of threads in a thread pool. This can have various impacts
//! on the performance and behavior of parallel operations:
//!
//! - **Performance Tuning**: Matching the number of threads to the number of
//!   logical cores on the system can maximize CPU utilization and minimize
//!   context switching, potentially improving performance.
//!
//! - **Resource Management**: Reducing the number of threads can lower resource
//!   usage, which might be beneficial in resource-constrained environments or to
//!   avoid overwhelming the system with too many threads.
//!
//! - **Task Granularity**: The ideal number of threads may depend on the nature
//!   of the tasks. For fine-grained tasks, having more threads might reduce
//!   execution time. For coarse-grained tasks, too many threads might lead to
//!   inefficiency.
//!
//! - **I/O-Bound Workloads**: For I/O-bound workloads, having more threads than
//!   CPU cores can be beneficial, as it allows other threads to make progress
//!   while some threads are blocked waiting for I/O operations to complete.
//!
//! - **Parallel Overhead**: There is a trade-off between parallel speedup and
//!   overhead. Overhead includes thread creation, synchronization, and
//!   communication. Too many threads can increase overhead and degrade performance.
//!
//! - **Load Balancing**: A higher number of threads can improve load balancing
//!   for uneven workloads, as tasks can be distributed across more workers.
//!
//! - **Diminishing Returns**: Due to Amdahl's Law, after a certain point, adding
//!   more threads may result in diminishing returns in terms of performance
//!   improvements.
//!
//! - **Testing**: In testing environments, reducing the number of threads can
//!   simplify the debugging process and make the execution more predictable.
//!
//! In terms of the major impact observed when adjusting the `num_threads()`
//! setting, the most significant performance gains are often seen when moving
//! from a single-threaded to a multi-threaded context, such as going from 1
//! thread to 2 threads. A further increase to 3 threads may still show
//! performance benefits, especially if the workload is well-suited to parallelism.
//! However, beyond this point, the performance gains can level off, and adding
//! more threads may not lead to substantial improvements. This plateau effect is
//! due to the overhead of managing additional threads and the limited parallelism
//! that can be extracted from the workload. It's crucial to profile and benchmark
//! the application with different thread counts to find the optimal setting.
//!
//! | Number of Threads | Processing Time (ms) | Improvement (vs 1 thread) |
//! |-------------------|----------------------|---------------------------|
//! | 1                 | 12                   | x1                        |
//! | 2                 | 8                    | x1.5                      |
//! | 3                 | 5                    | x2.4                      |
//! | 4                 | 4                    | x3                        |
//! | 5                 | 4                    | x3                        |
//! | 6                 | 3                    | x4                        |
//! | 7                 | 3                    | x4                        |
//! | 8                 | 3                    | x4                        |
//!
//! Beyond five threads, improvements in processing time
//! become marginal, suggesting that additional threads are contributing less to
//! overall performance gains. This data set reflects a common
//! pattern where after a certain point, the overhead of managing additional
//! threads outweighs the benefits, leading to a plateau in performance
//! improvements.
//!
//! # Could parallel slowdown ever happen? Explore.
//!
//! Parallel slowdown is a counterintuitive phenomenon that can occur when adding
//! more resources to a parallel computation results in increased execution time,
//! rather than a decrease. Several factors can contribute to parallel slowdown:
//!
//! - **Overhead of Coordination**: The management of multiple threads or
//!   processes involves overhead for communication, synchronization, and data
//!   distribution. As the number of concurrent units increases, this overhead can
//!   grow to the point where it outweighs the benefits of parallel execution.
//!
//! - **Contention for Shared Resources**: When threads or processes compete for
//!   shared resources, such as memory or I/O bandwidth, contention can lead to
//!   delays and reduced efficiency, causing slowdowns.
//!
//! - **Load Imbalance**: If the workload is not evenly distributed across the
//!   available processing units, some may remain idle while others are
//!   overburdened, leading to inefficient use of resources and slower overall
//!   performance.
//!
//! - **False Sharing**: In shared-memory systems, threads may inadvertently
//!   contend for cache lines when they read and write to independent variables
//!   that are located close to each other in memory. This can cause unnecessary
//!   cache invalidations and reduce performance.
//!
//! - **Algorithmic Limitations**: Some algorithms have limited parallelizability
//!   due to inherent sequential steps or data dependencies. Adding more parallel
//!   units does not improve, and can degrade, performance due to increased
//!   coordination complexity.
//!
//! - **Amdahl's Law**: According to Amdahl's Law, the maximum possible
//!   speedup of a program is determined by its serial portion. If a significant
//!   portion of the program cannot be parallelized, then adding more processing
//!   units will lead to a negligible speedup and could result in slowdown due to
//!   overhead.
//!
//! To avoid parallel slowdown, it is crucial to analyze the parallelizability of
//! the workload, optimize the use of resources, and carefully manage the trade-offs
//! between the number of processing units and the overhead they introduce.
//!
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let data = (0..1_000_000).collect::<Vec<i64>>();
    println!("number of cores: {}", num_cpus::get());
    println!("number of threads: {}", rayon::current_num_threads());
    println!("Data length: {}", data.len());

    // Sequential calculation
    let start = Instant::now();
    let sequential_sum: i64 = data
        .iter() // Specify the type
        .map(|x| x * x)
        .sum();
    let end = Instant::now();
    println!(
        "Sequential sum: {}, took {}ms",
        sequential_sum,
        end.duration_since(start).as_millis()
    );

    // Parallel calculation
    let start = Instant::now();
    let parallel_sum: i64 = data
        .par_iter() 
        .map(|x| x * x)
        .sum();
    let end = Instant::now();
    println!(
        "Parallel sum: {}, took {}ms",
        parallel_sum,
        end.duration_since(start).as_millis()
    );
}
