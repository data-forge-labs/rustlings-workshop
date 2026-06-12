//! Reflection Questions:
//! 
//! # What are some potential use cases for using Rust in Jupyter notebooks?
//!
//! Using Rust in Jupyter notebooks can be advantageous for a variety of use cases:
//!
//! - **Educational Purposes**: Teaching Rust concepts interactively, with the 
//!   ability to compile and run code snippets inline.
//! - **Data Processing**: Leveraging Rust's performance for data manipulation and 
//!   analysis tasks.
//! - **Algorithm Development**: Iteratively developing and testing algorithms with 
//!   immediate feedback on correctness and performance.
//! - **Prototyping**: Building and sharing prototypes of Rust applications or
//!   libraries.
//! - **Visualization**: Creating visual representations of data or algorithm 
//!   behavior using plotting libraries compatible with Jupyter.
//! - **Research and Experimentation**: Documenting research findings or conducting 
//!   experiments in a format that combines prose, code, and output.
//!
//! Overall, Jupyter notebooks provide an interactive platform for exploring Rust's 
//! capabilities in a variety of contexts, from education to data-intensive 
//! applications.
//! 
//! # How could displaying custom Rust types as HTML be useful?
//!
//! Displaying custom Rust types as HTML in environments like Jupyter notebooks
//! can be highly beneficial:
//!
//! - **Rich Presentation**: HTML rendering allows for more expressive and
//!   visually appealing presentations of data, which can include tables,
//!   charts, and other graphical elements.
//!
//! - **Interactive Documentation**: Developers can create interactive
//!   documentation with live examples that show the state of custom types in a
//!   more user-friendly format.
//!
//! - **Debugging Aid**: Visualizing complex data structures as HTML can help
//!   with debugging by providing a clearer representation of their current
//!   state.
//!
//! - **Educational Content**: Instructors can use HTML visualizations to teach
//!   Rust data structures and algorithms in a more engaging and intuitive way.
//!
//! - **Data Analysis**: Data scientists and analysts can view and interact with
//!   structured data outputs directly in their analysis workflow.
//!
//! Overall, displaying custom Rust types as HTML enhances the usability and
//! accessibility of information, making it a powerful tool for communication
//! and education.
//! 
//! # What Rust crates seem most interesting to explore with evcxr_jupyter?
//!
//! When using `evcxr_jupyter`, which is a Jupyter kernel for Rust, several crates
//! are particularly interesting to explore due to their utility in an interactive
//! environment:
//!
//! - **plotters**: A data visualization library that can be used to create a wide
//!   range of charts and plots.
//! - **itertools**: Provides extra iterator adaptors and functions, which is
//!   useful for data manipulation and exploration.
//! - **serde**: A framework for serializing and deserializing Rust data structures
//!   efficiently and generically, useful for working with JSON, CSV, or other
//!   data formats.
//! - **pandas**: When used via the `polars` or `rust-dataframe` crates, it allows
//!   for data manipulation akin to Python's pandas library.
//! - **regex**: Allows for text processing and analysis using regular expressions.
//! - **rayon**: Facilitates parallel data processing to speed up computation in
//!   data-heavy tasks.
//! - **ndarray**: A crate for handling large multi-dimensional arrays, which is
//!   essential for numerical computing and machine learning tasks.
//!
//! These crates can significantly enhance the interactive Rust programming
//! experience by providing powerful tools for data visualization, manipulation,
//! and analysis, among other tasks.
//! 
//! # How do notebooks in Rust compare to notebooks in Python?
//!
//! Notebooks in Rust and Python offer different experiences, each with its own
//! advantages and considerations:
//!
//! - **Ease of Use**: Python notebooks are often considered more user-friendly,
//!   especially for beginners, due to Python's simple syntax and the maturity of
//!   its notebook ecosystem.
//!
//! - **Performance**: Rust notebooks can provide better performance, thanks to
//!   Rust's focus on zero-cost abstractions and systems-level efficiency.
//!
//! - **Data Science Libraries**: Python has a well-established set of data science
//!   libraries like NumPy, pandas, and Matplotlib, while Rust is still growing its
//!   ecosystem in this area.
//!
//! - **Type Safety**: Rust's strong type system in notebooks can catch more errors
//!   at compile-time, providing more robust error checking before runtime.
//!
//! - **Concurrency**: Rust's ownership model and support for safe concurrency can
//!   be explored interactively in notebooks, which is beneficial for writing
//!   multi-threaded code.
//!
//! - **Compilation Time**: Rust's compilation step in notebooks can lead to longer
//!   wait times compared to the immediate execution of Python code.
//!
//! - **Interactivity**: Python notebooks have rich interactivity features and wide
//!   integration with web technologies for visualization, which might not be as
//!   readily available or mature in Rust notebooks.
//!
//! Overall, Rust notebooks are well-suited for users who prioritize performance
//! and type safety, while Python notebooks are ideal for those who value ease of
//! use and a vast array of available libraries for data science and visualization.
//! 
//! # What challenges do you anticipate in working with Rust notebooks?
//!
//! Working with Rust notebooks presents several challenges:
//!
//! - **Longer Iteration Times**: Rust's compile-check cycle can lead to longer
//!   iteration times compared to the immediate execution model of languages like
//!   Python.
//!
//! - **Library Ecosystem**: The availability of third-party libraries for tasks
//!   such as data analysis and visualization is more limited compared to Python's
//!   rich ecosystem.
//!
//! - **Learning Curve**: Rust's ownership and borrowing rules, along with its
//!   syntax, can be challenging for those new to the language.
//!
//! - **Error Handling**: Rust's compile-time error checking is strict, which,
//!   while beneficial for catching errors early, can slow down the exploratory
//!   process.
//!
//! - **Interactive Plotting**: Interactive plotting and visualization support in
//!   Rust is still maturing, potentially requiring additional effort to achieve
//!   the same results as in Python.
//!
//! - **Asynchronous Programming**: While Rust supports asynchronous programming,
//!   using it in Jupyter notebooks may not be as straightforward as in Python.
//!
//! - **Limited Community Examples**: There are fewer community-contributed
//!   notebooks for Rust, which can serve as learning resources or starting points
//!   for new projects.
//!
//! Despite these challenges, Rust notebooks can still be a powerful tool for those
//! who require the performance and safety guarantees that Rust provides.
//! 

fn main() {
    // Matrix example
    let m = rust_jupyter_notebook::Matrix::new(vec![1, 2, 3, 4], 2);
    println!("Matrix (2x2):");
    println!("{}", m.to_html());
    println!("num_rows: {}, num_cols: {}", m.num_rows(), m.num_cols());

    // SimpleDataFrame example
    let df = rust_jupyter_notebook::SimpleDataFrame::new(
        vec!["name".to_string(), "age".to_string()],
        vec![
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ],
    );
    println!("\nDataFrame:");
    println!("{}", df.to_html());

    // range_f64 example
    let r = rust_jupyter_notebook::range_f64(0.0, 5.0, 1.5);
    println!("\nrange_f64(0.0, 5.0, 1.5): {:?}", r);

    // Interactive crates
    println!(
        "\nInteractive crates: {:?}",
        rust_jupyter_notebook::list_interactive_crates()
    );

    // Use cases
    println!("\nUse cases:");
    for case in rust_jupyter_notebook::rust_notebook_use_cases() {
        println!("  - {}", case);
    }
}
