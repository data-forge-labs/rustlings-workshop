//! # How does the calculate_weights function determine the weight of each language?
//! 
//! The `calculate_weights` function determines the weight of each language by normalizing the years of
//! creation, calculating a weight based on the normalized year, and then inserting the language and its
//! corresponding weight into a new HashMap.
//!
//! The normalization is performed by first calculating the range of years, then transforming each year into
//! a normalized value between 0 and 1. The weight is then calculated based on the normalized year and
//! inserted into the HashMap along with the language name.
//!
//! The specific steps are:
//! 1. Calculate normalized year for each language based on the range of years.
//! 2. Calculate weight based on the normalized year.
//! 3. Insert the language and its corresponding weight into the new HashMap.
//! 
//! # How does the code ensure that the weights are normalized between 1 and 100?
//!
//! The `normalize` function ensures that the weights are normalized between 1 and 100 by performing the
//! following steps:
//! 1. Calculating the range of years by finding the minimum and maximum year values in the `languages`
//! map.
//! 2. Calculating the normalized year for each language based on the range of years.
//! 3. Calculating the weight based on the normalized year, ensuring it falls between 1 and 100.
//! 4. Inserting the language and its corresponding weight into the `weights` HashMap.
//!
//! The code uses a formula to calculate the weight: `weight = (normalized_year * 99.0) as u32 + 1`, which
//! ensures that the weights are normalized between 1 and 100.
//!
//! # Why does the code use values_mut when updating the years in the calculate_weights function?
//!
//! The code uses `values_mut` when updating the years in the `normalize` function because it needs mutable
//! references to the values in the hashmap in order to update them. This is necessary because the years are
//! being modified in place, and `values_mut` provides a way to get mutable references to the values in the
//! hashmap. This allows the code to directly modify the values in the original hashmap rather than creating
//! a new hashmap with the updated values.

use std::collections::HashMap;

/// Return a hashmap with the name of 15 the programming language and its year of creation.
fn languages() -> HashMap<String, u32> {
    let mut languages = HashMap::new();

    languages.insert("Rust".to_string(), 2010);
    languages.insert("Python".to_string(), 1991);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C".to_string(), 1972);
    languages.insert("C++".to_string(), 1983);
    languages.insert("C#".to_string(), 2000);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("Go".to_string(), 2009);
    languages.insert("Swift".to_string(), 2014);
    languages.insert("Kotlin".to_string(), 2011);
    languages.insert("R".to_string(), 1993);
    languages.insert("Ruby".to_string(), 1995);
    languages.insert("Scala".to_string(), 2003);
    languages.insert("Dart".to_string(), 2011);

    languages
}

/// Normalize the years in the given languages map and calculate weights.
fn normalize(languages: &mut HashMap<String, u32>) -> HashMap<String, u32> {
    for year in languages.values_mut() {
       *year = 2024 - *year; 
    }

    let min_year = languages.values().min().unwrap_or(&0);
    let max_year = languages.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, year) in languages.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as u32 + 1;
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    // Print the normalized weights.
    let weights = normalize(&mut languages());
    for (name, weight) in weights {
        println!("{}: {}", name, weight);
    }
}