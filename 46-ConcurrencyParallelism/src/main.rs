//! # Reflection Questions:
//!
//! # What stood out to you about Rust's approach to concurrency and parallelism?
//!
//! Rust's approach to concurrency and parallelism is primarily focused on safety
//! and preventing data races. The language enforces this through its ownership
//! and type systems, making it unique among system programming languages.
//!
//! One of the key features is the ownership system, which ensures that there are
//! clear rules about how data is accessed. Rust uses the concepts of ownership,
//! borrowing, and lifetimes to manage access to memory. This system prevents
//! bugs common in concurrent programming, such as dangling pointers and data
//! races, at compile time.
//!
//! Rust also makes extensive use of the `Send` and `Sync` traits to determine
//! which types are safe to transfer across threads or to be accessed from
//! multiple threads concurrently. `Send` allows the transfer of ownership
//! between threads, while `Sync` allows multiple threads to reference data if
//! no mutation occurs. These traits are automatically implemented by the
//! compiler for compatible types, enhancing safety.
//!
//! The Rust standard library provides several concurrency primitives like
//! `Mutex`, `RwLock`, and `Atomic` types, which are safe to use within Rust’s
//! strict ownership and borrowing guidelines. Rust's type system and these
//! safe concurrency primitives ensure that the programmer handles synchronization
//! explicitly, reducing the risk of subtle bugs.
//!
//! In summary, Rust’s approach emphasizes compile-time enforcement of safety
//! guarantees, explicit handling of mutable shared state, and leveraging its
//! powerful type system to prevent common concurrency errors.
//!
//! # How does Rust help prevent race conditions compared to other languages? What language features enable this?
//!
//! Rust's approach to preventing race conditions is fundamentally different from
//! many other languages, mainly due to its ownership and type systems. Here are
//! the key features that enable Rust to prevent race conditions:
//!
//! - **Ownership Rules**: Rust enforces unique ownership of data, meaning that
//!   only one variable can own a piece of data at a time. This prevents one
//!   thread from modifying data while another thread is reading it.
//!
//! - **Borrow Checker**: The borrow checker enforces rules about how references
//!   to data are used. It ensures that either multiple immutable references or
//!   one mutable reference can exist at a time, preventing simultaneous mutable
//!   access.
//!
//! - **Type System**: Rust's type system is designed to be aware of
//!   concurrency concerns. Types like `Arc` (Atomic Reference Counted) enable
//!   shared ownership across threads, while `Mutex` and `RwLock` provide
//!   exclusive access to data.
//!
//! - **No Data Races Guarantee**: Rust's compiler guarantees that race
//!   conditions are caught at compile time, which is not the case in many
//!   other languages where race conditions might only be discovered through
//!   runtime testing or not at all.
//!
//! - **Message Passing Concurrency**: Rust encourages the use of channels for
//!   communication between threads, following the message passing model which
//!   naturally avoids shared state.
//!
//! - **Zero-Cost Abstractions**: The language provides abstractions that make
//!   it easier to write safe concurrent code without introducing overhead,
//!   such as the `rayon` crate for data parallelism.
//!
//! These language features, when used appropriately, enable developers to write
//! concurrent code with confidence, reducing the risk of race conditions that
//! are common in other languages lacking these strict concurrency guarantees.
//!
//! # Why doesn't Rust choose a single concurrency model in the standard library? What are the trade-offs?
//!
//! Rust does not prescribe a single concurrency model in its standard library
//! because it aims to provide flexibility and cater to a wide variety of use
//! cases. Different models offer different trade-offs:
//!
//! - **Flexibility**: By supporting multiple concurrency paradigms, Rust gives
//!   developers the flexibility to choose the most appropriate one for their
//!   task, whether it's message passing, shared state, or a lock-free approach.
//!
//! - **Performance**: Different models can offer better performance depending
//!   on the context. For CPU-bound tasks, data parallelism might be ideal,
//!   while for IO-bound tasks, an async model could be more efficient.
//!
//! - **Complexity**: Some models are simpler and more intuitive for certain
//!   problems, reducing the potential for bugs and developer error.
//!
//! - **Library Ecosystem**: A single model might limit the potential for third-
//!   party libraries to innovate and provide solutions tailored to specific
//!   problems or domains.
//!
//! - **Interoperability**: Different parts of a program might benefit from
//!   different concurrency models, and Rust allows these models to interoperate
//!   within the same program.
//!
//! The trade-off of not choosing a single model is that developers must
//! understand the strengths and weaknesses of each to make an informed choice.
//! However, Rust's strong type system and ownership model ensure that, regardless
//! of the chosen concurrency paradigm, the code remains safe and free of data
//! races.
//!
//! # Do you think Rust strikes the right balance in its concurrency support? Why or why not?
//!
//! Many in the Rust community believe that Rust strikes a good balance in its
//! concurrency support because:
//!
//! - **Safety**: Rust's strict type system and ownership model provide strong
//!   guarantees about memory safety and data race prevention, which are critical
//!   for writing reliable concurrent code.
//!
//! - **Flexibility**: Rust does not enforce a single concurrency model, allowing
//!   developers to choose the best approach for their specific problem, whether
//!   it's using threads, async/await, message passing, or other paradigms.
//!
//! - **Performance**: Rust's zero-cost abstractions mean that the overhead
//!   introduced by concurrency support is minimal, allowing for high-performance
//!   concurrent applications.
//!
//! - **Ecosystem**: The Rust ecosystem includes a variety of third-party
//!   libraries offering different concurrency models, which can be seamlessly
//!   integrated into Rust applications.
//!
//! However, some may argue that the balance is not yet perfect because:
//!
//! - **Complexity**: The learning curve for understanding Rust's concurrency
//!   model can be steep, especially for those new to the concepts of ownership
//!   and borrowing.
//!
//! - **Ergonomics**: Some developers find the verbosity associated with safe
//!   concurrency in Rust to be a hurdle, as it can make concurrent code more
//!   cumbersome to write and read.
//!
//! - **Async Ecosystem**: While growing, the async ecosystem is still maturing,
//!   and some find it to be less cohesive compared to more established async
//!   ecosystems like those in Node.js or C#.
//!
//! Overall, Rust's concurrency support is generally seen as robust and
//! well-rounded, but opinions on its balance may vary depending on individual
//! use cases and developer experience.
//!
//! # Discussion Prompts:
//!
//! # Compare and contrast message passing and shared memory models of concurrency. What are the pros and cons of each?
//!
//! Message passing and shared memory are two fundamental models for handling
//! concurrency. Here is a comparison of the two models along with their pros and
//! cons:
//!
//! ## Message Passing
//! **Pros**:
//! - *Safety*: Avoids the risks of concurrent access to shared data, as
//!   communication happens through passing messages, which does not require
//!   shared state.
//! - *Decoupling*: Encourages loose coupling between concurrent components,
//!   facilitating easier reasoning and maintenance.
//! - *Scalability*: Scales well in distributed systems where processes run
//!   across different machines.
//!
//! **Cons**:
//! - *Overhead*: Can introduce overhead due to message copying and serialization,
//!   especially for large data transfers.
//! - *Complexity*: May require complex routing and handling logic for messages,
//!   especially in systems with many interacting components.
//!
//! ## Shared Memory
//! **Pros**:
//! - *Performance*: Provides the potential for high performance through direct
//!   access to shared data without the overhead of message passing.
//! - *Convenience*: Can be more straightforward to implement for simple cases
//!   where data is frequently accessed and modified by multiple threads.
//!
//! **Cons**:
//! - *Race Conditions*: Requires careful synchronization to prevent race
//!   conditions, which can lead to bugs that are difficult to detect and fix.
//! - *Complexity*: Properly managing locks and synchronization mechanisms can
//!   add complexity to the codebase.
//! - *Scalability*: Might not scale as well in distributed systems due to the
//!   difficulty of synchronizing memory across network boundaries.
//!
//! Rust provides abstractions for both models—channels for message passing and
//! synchronization primitives like `Mutex` and `Arc` for shared memory—which
//! allows developers to choose the most suitable approach for their use case.
//!
//! # How do languages like Go, Erlang, and Pony differ from Rust in their concurrency approaches? What can Rust learn from them?
//!
//! Go, Erlang, and Pony offer distinct concurrency models that differ from Rust's
//! approach:
//!
//! ## Go
//! Go uses goroutines, lightweight threads managed by the Go runtime, along with
//! channels for communication. Its concurrency model is based on CSP
//! (Communicating Sequential Processes).
//! **Pros**: Simple and lightweight abstraction for concurrency.
//! **Cons**: Runtime can introduce overhead and less control over thread
//! management.
//! **Lessons for Rust**: Simplifying asynchronous programming can lead to wider
//! adoption.
//!
//! ## Erlang
//! Erlang's concurrency is based on the Actor model, with lightweight processes
//! and message passing. It also provides strong fault tolerance and "let it
//! crash" philosophy.
//! **Pros**: Mature model for distributed systems and fault tolerance.
//! **Cons**: Performance overhead due to immutable data copying.
//! **Lessons for Rust**: Robust fault tolerance and system recovery can be
//! beneficial.
//!
//! ## Pony
//! Pony uses an Actor model with capabilities-based security. It guarantees
//! deadlock-free execution and uses work-stealing for scheduling.
//! **Pros**: Provides formal guarantees around safety and deadlock prevention.
//! **Cons**: Steeper learning curve due to unique concepts like capabilities.
//! **Lessons for Rust**: Formal safety guarantees are a valuable asset.
//!
//! Rust can learn from these languages by integrating their strengths, such as
//! Go's simplicity, Erlang's fault tolerance, and Pony's formal safety
//! guarantees, into its own ecosystem through libraries or language features,
//! while still maintaining Rust's principles of zero-cost abstractions and
//! memory safety.
//!
//! # What real-world examples demonstrate the benefits of Rust's concurrency safety? When could it help prevent bugs?
//!
//! Real-world examples that demonstrate the benefits of Rust's concurrency safety
//! include web servers, database engines, and simulation software where
//! multiple threads or tasks need to access shared resources concurrently.
//!
//! - **Web Servers**: Rust's concurrency safety prevents race conditions when
//!   handling multiple client requests that access and modify shared state, such
//!   as in-memory caches or session data.
//!
//! - **Database Engines**: Rust ensures that concurrent transactions on a
//!   database engine do not lead to data corruption or inconsistencies, which
//!   is critical for maintaining data integrity.
//!
//! - **Simulation Software**: In systems that simulate complex environments with
//!   parallel operations, Rust's safety guarantees help ensure that the
//!   simulation state remains consistent across iterations.
//!
//! Rust's concurrency safety can prevent bugs in scenarios such as:
//!
//! - **Data Races**: When two threads access the same mutable data concurrently
//!   without adequate synchronization, leading to unpredictable results. Rust's
//!   borrow checker prevents this at compile time.
//!
//! - **Deadlocks**: Rust's type system and ownership model can prevent deadlocks
//!   by enforcing certain locking orders or by using lock-free programming
//!   patterns.
//!
//! - **Use-after-free**: Rust's lifetime tracking ensures that data is not
//!   accessed after it has been freed, which can cause undefined behavior in
//!   less safe languages.
//!
//! In essence, Rust's concurrency safety features help maintain the integrity of
//! systems and prevent a class of bugs that are difficult to diagnose and
//! reproduce in other languages, ultimately leading to more reliable software.
//!
//! # Should languages optimize for parallelism on multi-core hardware? Do the safety trade-offs outweigh performance gains?
//!
//! As multi-core hardware becomes the standard, languages that optimize for
//! parallelism can leverage the full potential of the hardware, leading to
//! significant performance gains in many applications. However, parallelism
//! introduces complexity and potential safety trade-offs:
//!
//! **Safety vs. Performance**:
//! - Safety concerns in parallel execution include race conditions, deadlocks,
//!   and other concurrency-related bugs that can be challenging to detect and
//!   resolve.
//! - Performance gains from parallelism can be substantial, especially for
//!   compute-intensive tasks or when processing large data sets.
//!
//! **Language Design**:
//! - A language designed with parallelism in mind can offer abstractions that
//!   make it safer and easier to write parallel code, such as Rust's ownership
//!   and borrowing model.
//! - Some languages may sacrifice certain safety guarantees for the sake of
//!   performance, requiring developers to be more vigilant and experienced in
//!   managing concurrency.
//!
//! **Developer Experience**:
//! - Languages that prioritize safety may have a steeper learning curve but can
//!   lead to more robust and maintainable codebases.
//! - Languages that offer greater control over hardware optimization may provide
//!   better performance but require developers to handle safety manually.
//!
//! In conclusion, whether safety trade-offs outweigh performance gains depends on
//! the specific use case, the domain of the application, and the skill level of
//! the developers. Some domains may prioritize safety and correctness over raw
//! performance, while others may require every ounce of performance available.
//! Ideally, a language should aim to provide both safety and performance, giving
//! developers the tools to write safe parallel code that fully utilizes modern
//! multi-core processors.
//!
//! # Can complexity due to concurrency be contained? Or is it unavoidably difficult no matter the language?
//!
//! Concurrency inherently introduces complexity due to the need to coordinate
//! between concurrently executing tasks. However, the level of difficulty in
//! managing this complexity can vary based on the language and its tools:
//!
//! - **Abstractions and Models**: High-level abstractions and concurrency
//!   models provided by a language can contain complexity by simplifying the
//!   way developers write concurrent code. For example, Rust's ownership model
//!   naturally extends to concurrency, preventing certain classes of bugs.
//!
//! - **Libraries and Frameworks**: Robust libraries and frameworks that offer
//!   well-designed interfaces for concurrency can encapsulate complex
//!   synchronization logic, making it easier for developers to use.
//!
//! - **Language Design**: Some languages are designed with a focus on
//!   simplifying concurrent programming. For instance, languages like Erlang use
//!   the Actor model to abstract away the details of thread management and
//!   communication.
//!
//! - **Tooling and Compiler Checks**: Strong compile-time checks, static
//!   analysis tools, and runtime diagnostics can help identify concurrency
//!   issues early in the development process.
//!
//! While concurrency complexity can be reduced, it may not be completely
//! eliminated due to the nondeterministic nature of concurrent execution.
//! Languages and tools can only assist in managing this complexity, not remove
//! it entirely. The goal is to minimize the burden on developers and reduce the
//! likelihood of concurrency-related bugs.
//!
use std::{
    cmp::max,
    mem::{align_of, size_of},
    ptr,
};

#[derive(Debug)]
struct Carton<T>(ptr::NonNull<T>);

impl<T> Carton<T> {
    fn new(value: T) -> Self {
        // Allocate enough memory on the heap to store one T.
        assert_ne!(size_of::<T>(), 0, "Zero-sized types are not allowed");
        let mut memptr: *mut T = ptr::null_mut();
        unsafe {
            let ret = libc::posix_memalign(
                (&mut memptr as *mut *mut T).cast(),
                max(align_of::<T>(), size_of::<usize>()),
                size_of::<T>(),
            );
            assert_eq!(ret, 0, "Failed to allocate memory");
        };

        // NonNull is just a wrapper that enforces that the pointer isn't null.
        let prt = {
            // Safety: memptr is dereferencable because we created it from a reference and have exclusive access.
            ptr::NonNull::new(memptr)
                .expect("Guaranteed to be non-null if posix_memalign succeeds (return 0)")
        };

        unsafe {
            // Safety: If non-null, posix_memalign gives us a ptr that is valid for writes and properly aligned.
            prt.as_ptr().write(value);
        }

        Self(prt)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require readers to borrow the
            //   Carton, and the lifetime of the return value is elided to the
            //   lifetime of the input. This means the borrow checker will
            //   enforce that no one can mutate the contents of the Carton until
            //   the reference returned is dropped.
            self.0.as_ref()
        }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require writers to mutably
            //   borrow the Carton, and the lifetime of the return value is
            //   elided to the lifetime of the input. This means the borrow
            //   checker will enforce that no one else can access the contents
            //   of the Carton until the mutable reference returned is dropped.
            self.0.as_mut()
        }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr().cast());
        }
    }
}

// A type is Send if it can be sent to another thread. In other words, if ownership of
// a value of that type can be transferred to another thread. For example, Arc<i32>
// is Send, but Rc<i32> is no
//
// Safety: No one besides us has the raw pointer, so we can safely transfer the
// Carton to another thread if T can be safely transferred.
unsafe impl<T> Send for Carton<T> where T: Send {}

// A type is Sync if it can be shared with another thread. In other words, a type T
// is Sync if and only if a shared reference to that type, &T, is Send
//
// Safety: Since there exists a public way to go from a `&Carton<T>` to a `&T`
// in an unsynchronized fashion (such as `Deref`), then `Carton<T>` can't be
// `Sync` if `T` isn't.
// Conversely, `Carton` itself does not use any interior mutability whatsoever:
// all the mutations are performed through an exclusive reference (`&mut`). This
// means it suffices that `T` be `Sync` for `Carton<T>` to be `Sync`:
unsafe impl<T> Sync for Carton<T> where T: Sync {}

fn main() {
    let mut carton = Carton::new(String::from("Hello, world!"));

    std::thread::spawn(move || {
        println!("Carton directions: {:?}", carton);
        println!("Carton contents: {:?}", *carton);
        *carton = String::from("Goodbye, world!");
        println!("Carton contents: {:?}", *carton);
    })
    .join()
    .unwrap();
}
