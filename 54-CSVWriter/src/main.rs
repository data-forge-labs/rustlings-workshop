//! Reflection Questions:
//!
//! # What are the advantages of using Rust for processing CSV data over scripting languages like Python?
//!
//! Rust has several advantages over scripting languages like Python for processing CSV data:
//!
//! - **Memory Safety**: Rust guarantees memory safety, preventing common bugs such as null pointer
//!   dereferencing, data races, and buffer overflows. This makes it easier to write robust and
//!   efficient CSV processing code.
//!
//! - **Performance**: Rust is known for its high-performance, zero-cost abstractions, and its
//!   ownership-based memory management can result in faster and more efficient CSV processing
//!   code compared to scripting languages.
//!
//! - **Ecosystem**: Rust has a mature ecosystem with well-established libraries for working with
//!   CSV data, such as `csv` and `serde_csv`. These libraries provide a simple and consistent
//!   API for parsing and writing CSV data, reducing the need to reinvent the wheel.
//!
//! - **Type Safety**: Rust's type system ensures that the data being processed is of the correct
//!   type, reducing the risk of type-related errors.
//!
//! - **Compile-time Error Checking**: Rust performs compile-time error checking, catching errors
//!   during the build process and preventing runtime errors.
//!
//! # How does buffered reading with `csv::Reader` improve CSV parsing performance?
//!
//! `csv::Reader` provides a buffered reader that can improve CSV parsing
//! performance by reducing the number of system calls made to read data from
//! disk.
//!
//! When reading CSV data from disk, the operating system performs a system call
//! to read a small chunk of data from the file into memory. This system call is
//! relatively expensive, and can result in poor performance when parsing CSV
//! data.
//!
//! By default, `csv::Reader` uses an unbuffered reader, which reads data from
//! disk one byte at a time. However, using a buffered reader, such as a
//! `BufReader`, can improve performance by reading larger chunks of data into
//! memory at once. This reduces the number of system calls needed to read data
//! from disk, resulting in faster and more efficient parsing of CSV data.
//!
//! By default, `csv::Reader` uses a buffer size of 4096 bytes, which can be
//! configured using the `buffer_size` method.
//!
//! # What error handling strategies should be used when working with I/O in Rust?
//!
//! Handling I/O operations in Rust requires careful consideration of potential
//! errors. The language's type system and pattern matching capabilities offer
//! powerful tools to deal with errors in a structured and predictable way.
//! Here are some recommended strategies:
//!
//! - Utilize the `Result` type: I/O operations typically return a `Result` type
//!   to represent either success (`Ok`) or error (`Err`). Always handle the
//!   `Result` using pattern matching or combinators to ensure errors do not go
//!   unchecked.
//!
//! - Propagate errors with `?`: Within functions that return a `Result`, use the
//!   `?` operator to propagate errors up the call stack. This simplifies error
//!   handling by allowing higher-level code to decide on the error handling
//!   strategy.
//!
//! - Define custom error types: For complex applications, create custom error
//!   types that can encapsulate various error conditions and provide additional
//!   context. This can be done with `enum` or `struct` and the `thiserror` or
//!   `anyhow` crates can assist in simplifying this process.
//!
//! - Avoid panicking: While methods like `unwrap` and `expect` are convenient,
//!   they cause the program to panic when they encounter an `Err` value. Reserve
//!   these methods for scenarios where a panic is the correct response to an
//!   unrecoverable error.
//!
//! - Log errors: Use logging to record errors, which can aid in diagnosing issues
//!   without crashing the program. Rust's log crate provides a flexible logging
//!   system that can be configured for different environments.
//!
//! - Match against `std::io::ErrorKind`: When you need fine-grained control over
//!   I/O error handling, match against the specific kinds of I/O errors using
//!   `std::io::ErrorKind`. This allows tailored responses to different error
//!   conditions.
//!
//! - Leverage community crates: The Rust ecosystem includes crates that offer
//!   extended capabilities for error handling. Crates such as `anyhow` for
//!   simplified error handling and `thiserror` for deriving custom error types
//!   can help manage the complexity of error handling.
//!
//! Applying these strategies will help you create more resilient Rust
//! applications that can handle I/O errors effectively and provide clear
//! diagnostics when things go wrong.
//!
//! # How can the code be extended to support more complex CSV processing workflows?
//!
//! Extending the current CSV processing code to support more complex workflows
//! involves adding functionality that can handle a wider range of processing
//! tasks. Here are some enhancements that could be made:
//!
//! - **Streaming Data**: Implement streaming to process large CSV files without
//!   loading the entire file into memory. This can be achieved with iterators
//!   that yield records as they are read.
//!
//! - **Data Transformation**: Add support for transforming data through custom
//!   functions or closures that can modify each field or record.
//!
//! - **Error Handling**: Improve error handling by capturing and logging errors
//!   for individual records without stopping the entire processing workflow.
//!
//! - **Parallel Processing**: Utilize Rust's concurrency features to process
//!   multiple CSV files or records in parallel, improving performance on
//!   multi-core systems.
//!
//! - **Data Validation**: Integrate data validation to check for missing or
//!   invalid data and either clean it or report it as needed.
//!
//! - **Dynamic Typing**: Allow for dynamic typing of CSV fields so that the
//!   schema can be inferred at runtime or specified in a configuration file.
//!
//! - **Complex Queries**: Provide the ability to perform complex queries or
//!   aggregations on the data, similar to SQL operations on databases.
//!
//! - **Output Formatting**: Support various output formats other than CSV, such
//!   as JSON, XML, or custom text formats, to enable easy integration with
//!   other systems or services.
//!
//! - **Configuration and Extensibility**: Create a configuration system that
//!   allows users to specify processing rules and workflows, making the tool
//!   adaptable to different use cases without altering the codebase.
//!
//! - **Plug-in System**: Develop a plug-in system that lets users extend
//!   functionality with custom components or modules.
//!
//! By implementing these features, the CSV processing code can be made more
//! versatile and suitable for a wider range of data processing tasks.
//!
//! # What Rust concurrency features could help speed up large CSV processing jobs?
//!
//! To speed up large CSV processing jobs, leveraging Rust's concurrency features
//! can provide significant performance improvements. Here are some of the
//! concurrency features and techniques in Rust that could be beneficial:
//!
//! - **Threads**: Use `std::thread` to perform processing tasks concurrently
//!   across multiple cores. Threads can be used to parallelize reading, parsing,
//!   and writing operations.
//!
//! - **Rayon**: The Rayon crate provides data parallelism and can automatically
//!   distribute processing tasks across available CPU cores. It is particularly
//!   effective for parallel iterations and map-reduce operations.
//!
//! - **Async/Await**: For I/O-bound tasks, such as reading from or writing to
//!   disk or a network, Rust's asynchronous programming model with `async` and
//!   `await` can help achieve concurrency without blocking threads.
//!
//! - **Message Passing**: Use channels (via `std::sync::mpsc`) for safe
//!   communication between threads. This can help in creating producer-consumer
//!   patterns where one set of threads reads and parses CSV data while another
//!   set processes it.
//!
//! - **Shared State**: Employ atomic operations and synchronization primitives
//!   like `Mutex` and `Arc` from `std::sync` to safely share mutable state
//!   between threads when necessary.
//!
//! - **Crossbeam**: This crate provides advanced data structures and primitives
//!   for concurrent programming, offering more flexibility and performance for
//!   complex concurrency patterns.
//!
//! - **Tokio**: For asynchronous I/O, the Tokio runtime offers a non-blocking
//!   event-driven model that can handle a large number of I/O tasks with a
//!   small number of system threads.
//!
//! By applying these concurrency features, CSV processing can be significantly
//! accelerated, especially for CPU-intensive parsing or I/O-bound workflows.
//! However, it is important to carefully manage concurrency to avoid issues like
//! race conditions, deadlocks, and bottlenecks.
//! 

use csv;
use serde::{Deserialize, Serialize};
use std::error::Error;

static DISCOUNT: f64 = 0.1;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Product {
    name: String,
    price: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/products.csv")?;
    let mut wtr = csv::Writer::from_path("data/discounted_products.csv")?;

    let mut savings = 0.0;
    for result in rdr.deserialize::<Product>() {
        let record = result?;
        wtr.serialize(make_discounts(&record)?)?;

        // Challenge(2): Calculate the total savings
        savings += record.price * DISCOUNT;
    }

    wtr.flush()?;

    // Challenge(2): Print the total savings
    println!("Savings: ${:.2}", savings);

    Ok(())
}

fn make_discounts(product: &Product) -> Result<Product, Box<dyn Error>> {
    Ok(Product {
        name: product.name.clone(),
        price: product.price * (1.0 - DISCOUNT), 
    })
}
