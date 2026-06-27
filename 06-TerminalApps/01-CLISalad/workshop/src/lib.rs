//! # Reflection Questions:
//!
//! # How does the clap library help in creating a command-line interface (CLI) in Rust?
//!
//! `clap` is a powerful and widely-used library in Rust for parsing command-line arguments and
//! subcommands. It simplifies the process of building sophisticated command-line interfaces
//! (CLIs) with various features and customizations. Here's how `clap` aids in CLI development:
//!
//! 1. **Declarative Syntax**: `clap` allows developers to use a declarative macro system or a
//! more verbose, but flexible, builder pattern to define possible command-line arguments,
//! flags, and subcommands. This makes the code more readable and easier to maintain.
//!
//! 2. **Automatic Help and Version**: By default, `clap` automatically generates help messages
//! and version information for your CLI, including detailed descriptions for each argument and
//! subcommand. This helps users understand how to use your application without additional
//! effort on your part.
//!
//! 3. **Argument Validation**: `clap` provides built-in validations and can enforce specific
//! types, value ranges, or patterns for the arguments passed to the CLI. This feature reduces
//! boilerplate code for argument checking and parsing, ensuring inputs meet the expected
//! criteria before execution.
//!
//! 4. **Complex CLI Structures**: With `clap`, developers can easily create complex CLI
//! applications that include subcommands (similar to `git push`, `git pull`), each with its own
//! set of arguments and flags. This allows for the development of rich and user-friendly
//! command-line applications.
//!
//! 5. **Customization and Flexibility**: `clap` offers extensive customization options for
//! error messages, help messages, argument behaviors (e.g., multiple occurrences, optional
//! values), and much more. This level of control enables developers to tailor the CLI
//! experience to their application's needs.
//!
//! 6. **Environment Variable Support**: It also supports defining arguments that can be set
//! via environment variables, providing flexibility for users to interact with the CLI
//! application in different contexts.
//!
//! Overall, `clap` streamlines the creation of command-line interfaces by handling many common
//! and advanced scenarios out of the box, allowing developers to focus on the unique aspects of
//! their CLI applications.
//!
//! # How is the Vec fruits shuffled in the create_fruit_salad function?
//!
//! The shuffling of the `Vec` containing fruits in the `create_fruit_salad` function is achieved
//! through the use of the `shuffle` method provided by the `rand` crate, specifically through its
//! `SliceRandom` trait. This method takes a mutable reference to a slice (which the `Vec` can be
//! coerced into) and a mutable reference to a random number generator (RNG) and randomly
//! reorders the elements within the slice in-place.
//!
//! Here's a breakdown of the process:
//!
//! 1. **RNG Initialization**: First, a random number generator is initialized by calling
//! `thread_rng()`, which provides a thread-local RNG seeded by the system.
//!
//! 2. **Shuffling**: The `shuffle` method is then called on the `fruits` vector with the
//! `&mut rng` passed as an argument. This method mutates the vector, randomly permuting the
//! elements it contains.
//!
//! 3. **Selecting a Subset**: After shuffling, the function selects a subset of the shuffled
//! fruits using `into_iter().take(num_fruits).collect()`. This takes the first `num_fruits`
//! elements from the shuffled list and collects them into a new `Vec<String>`, which is then
//! returned.
//!
//! This approach allows for the creation of a random assortment of fruits from the predefined
//! list, with the number of fruits in the final salad determined by the `num_fruits` parameter.
//! The use of `SliceRandom::shuffle` ensures that the selection is varied and unpredictable,
//! making the function versatile for generating different combinations of fruit salads.
//!
//! # Why is there a need to convert the fruits Vec into an iterator and then take only a specific
//! number of fruits?
//!
//! The conversion of the `fruits` Vec into an iterator followed by taking only a specific number
//! of fruits is a method to efficiently create a subset of the original list based on the
//! `num_fruits` parameter. This technique serves several purposes in the context of the
//! `create_fruit_salad` function:
//!
//! 1. **Dynamic Subset Selection**: By converting the `Vec` to an iterator, we can use the `take`
//! method to easily specify how many elements (fruits in this case) we want to include in the
//! final Vec. This allows for flexible control over the size of the resulting fruit salad,
//! enabling the function to return a variable number of fruits based on the `num_fruits`
//! argument.
//!
//! 2. **Efficiency**: This approach is efficient in terms of both memory usage and performance.
//! Converting to an iterator and then using `take` does not require copying the entire Vec or
//! manually iterating through the Vec to select a certain number of elements. Instead, it
//! leverages iterator laziness, only processing items up to the limit specified by `take`.
//!
//! 3. **Simplicity and Readability**: Using iterator methods like `into_iter()` and `take()`
//! makes the code concise and easy to understand. It clearly expresses the intent to transform
//! the collection into a sequence of elements, from which only a specified number are needed.
//!
//! 4. **Flexibility for Further Transformations**: If needed, additional iterator methods can be
//! chained after `take` to perform further transformations on the selected subset of fruits.
//! This is useful in scenarios where further processing is required, such as filtering or
//! mapping, before finally collecting the results into a Vec.
//!
//! In summary, converting the `fruits` Vec into an iterator and then taking only a specific
//! number of fruits is a streamlined and versatile technique for generating a customizable and
//! dynamic subset of the original collection, perfectly suited for the `create_fruit_salad`
//! function's requirements.

/// Return the hard-coded list of available fruits.
pub fn list_fruits() -> Vec<String> {
    vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ]
}

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    use rand::seq::SliceRandom;
    let mut fruits = list_fruits();
    let mut rng = rand::rng();
    fruits.shuffle(&mut rng);
    fruits.into_iter().take(num_fruits).collect()
}

pub fn fruit_salad_cli(args: Vec<String>) -> Result<String, String> {
    let num = args.get(1).ok_or_else(|| "missing argument".to_string())?;
    let n: usize = num.parse().map_err(|e| format!("invalid number: {}", e))?;
    let salad = create_fruit_salad(n);
    Ok(format!("Fruit salad with {} fruits: {:?}", n, salad))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_fruit_list {
        use super::*;

        #[test]
        fn test_contains_expected_fruits() {
            let fruits = list_fruits();
            assert!(fruits.contains(&"Apple".to_string()));
            assert!(fruits.contains(&"Fig".to_string()));
            assert!(fruits.contains(&"Cherry".to_string()));
        }

        #[test]
        fn test_list_length() {
            assert_eq!(list_fruits().len(), 10);
        }

        #[test]
        fn test_consistent_order() {
            let fruits = list_fruits();
            assert_eq!(fruits[0], "Arbutus");
            assert_eq!(fruits[9], "Apple");
        }
    }

    mod step_02_fruit_salad {
        use super::*;

        #[test]
        fn test_returns_correct_count() {
            let salad = create_fruit_salad(3);
            assert_eq!(salad.len(), 3);
        }

        #[test]
        fn test_returns_subset_of_fruits() {
            let fruits = list_fruits();
            let salad = create_fruit_salad(5);
            assert_eq!(salad.len(), 5);
            for fruit in &salad {
                assert!(fruits.contains(fruit), "{} is not in the fruit list", fruit);
            }
        }

        #[test]
        fn test_handles_zero() {
            let salad = create_fruit_salad(0);
            assert!(salad.is_empty());
        }

        #[test]
        fn test_handles_overflow() {
            let salad = create_fruit_salad(20);
            assert_eq!(salad.len(), 10);
        }
    }

    mod step_03_cli {
        use super::*;

        #[test]
        fn test_cli_valid_number() {
            let result = fruit_salad_cli(vec![
                "cli-salad".to_string(),
                "3".to_string(),
            ]);
            assert!(result.is_ok());
            let output = result.unwrap();
            assert!(output.contains("Fruit salad with 3 fruits"));
        }

        #[test]
        fn test_cli_invalid_input() {
            let result = fruit_salad_cli(vec![
                "cli-salad".to_string(),
                "abc".to_string(),
            ]);
            assert!(result.is_err());
        }

        #[test]
        fn test_cli_missing_arg() {
            let result = fruit_salad_cli(vec!["cli-salad".to_string()]);
            assert!(result.is_err());
        }
    }
}
