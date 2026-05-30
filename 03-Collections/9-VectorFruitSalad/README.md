# Rust for Python Data Engineers — Vector Fruit Salad

*Your first data-engineering style project in Rust: select, shuffle, and serve random fruit combinations using vectors — the Rust equivalent of Python lists.*

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)
4. [Concept: Vec Recap — Your Dynamic Collection](#4-concept-vec-recap--your-dynamic-collection)
5. [Concept: The `rand` Crate — Random Numbers](#5-concept-the-rand-crate--random-numbers)
6. [Concept: Working with External Crates (Cargo.toml)](#6-concept-working-with-external-crates-cargotoml)
7. [Concept: SliceRandom — Shuffling and Choosing](#7-concept-slicerandom--shuffling-and-choosing)
8. [Building Step by Step](#8-building-step-by-step)
9. [Complete Code](#9-complete-code)
10. [Exercises](#10-exercises)
11. [Summary](#11-summary)

---

## 1. Project Overview

We'll build a program that:
1. Stores a list of fruits
2. Randomly selects a subset
3. Shuffles the selection
4. Prints a "fruit salad" — a comma-separated list

### Python Comparison

```python
# Python version of what we're building
import random

FRUITS = ["Orange", "Apple", "Banana", "Pear", "Grape",
          "Watermelon", "Strawberry", "Cherry", "Plum", "Peach"]

def make_salad():
    count = random.randint(1, len(FRUITS))
    selected = random.choices(FRUITS, k=count)
    random.shuffle(selected)
    print("Fruit salad:", ", ".join(selected))

make_salad()
# Output: Fruit salad: Grape, Banana, Peach, Apple
```

### What You'll Learn

| Rust Concept | Python Equivalent | Data Engineering Use |
|---|---|---|
| `Vec<&str>` | `list[str]` | Dynamic data collections |
| `rand` crate | `random` module | Sampling, shuffling data |
| `SliceRandom` trait | `random.shuffle()` | Randomizing data order |
| Cargo dependencies | `requirements.txt` | Managing external packages |
| `for` + `.iter()` + `.enumerate()` | `for i, x in enumerate(list)` | Iterating with indices |

---

## 2. Prerequisites

- Completed [Basic Calculator](../01-Foundations/1-BasicCalculator/README.md)
- Familiar with `Vec<T>` from [TicketManagement](../03-Collections/6-TicketManagement/README.md)
- Understand `for` loops and `if/else`

---

## 3. Running the Python Version

```python
# project.py — run to see expected output
python project.py
# Sample output: Fruit salad: Plum, Apple, Grape, Cherry
```

---

## 4. Concept: Vec Recap — Your Dynamic Collection

### Creating a Vec

```rust
// Method 1: Vec::new()
let mut fruits: Vec<&str> = Vec::new();
fruits.push("Apple");
fruits.push("Banana");

// Method 2: vec! macro
let fruits = vec!["Apple", "Banana", "Cherry"];

// Method 3: From an array
let arr = ["a", "b", "c"];
let vec_from_arr: Vec<&str> = arr.to_vec();
```

### Vec Methods for Data Engineers

```rust
let mut data: Vec<f64> = vec![1.5, 2.3, 4.7, 0.5];

data.push(3.2);                 // Append
let last = data.pop();           // Remove last → Some(3.2)
let first = data.first();        // First element → Option<&f64>
let count = data.len();          // Number of elements
let is_empty = data.is_empty();  // Check if empty
data.sort();                     // Sort in place
data.reverse();                  // Reverse order
data.dedup();                    // Remove consecutive duplicates
data.clear();                    // Remove all elements

// Convert back to array (if size known at compile time)
let array: [f64; 4] = data.try_into().unwrap();
```

### Vec vs Python List

| Operation | Python `list` | Rust `Vec<T>` |
|---|---|---|
| Create empty | `items = []` | `let items: Vec<T> = Vec::new();` |
| Create with values | `items = [1, 2, 3]` | `let items = vec![1, 2, 3];` |
| Add one | `items.append(x)` | `items.push(x)` |
| Remove last | `items.pop()` | `items.pop()` |
| Access by index | `items[i]` | `items[i]` (panics if out of bounds) |
| Safe access | N/A | `items.get(i)` → `Option<&T>` |
| Length | `len(items)` | `items.len()` |
| Slice | `items[start:end]` | `&items[start..end]` |
| Iterate | `for x in items:` | `for x in &items { }` |

---

## 5. Concept: The `rand` Crate — Random Numbers

### Adding to Cargo.toml

```toml
[dependencies]
rand = "0.8.5"   # Use this version
```

### Generating Random Numbers

```rust
use rand::Rng;  // Import the Rng trait

let mut rng = rand::thread_rng();  // Create a random number generator

let x: u32 = rng.gen();          // Random u32 (0 to u32::MAX)
let y = rng.gen_range(1..=10);   // Random number between 1 and 10
let z: f64 = rng.gen();          // Random float 0.0 to 1.0
let b: bool = rng.gen();         // Random bool (true/false)
```

### Python vs rand

```python
import random

x = random.randint(1, 10)    # Random int 1-10
y = random.random()           # Random float 0.0-1.0
z = random.choice(items)      # Random element
items_shuffled = random.sample(items, k=count)  # Random subset
random.shuffle(items)         # Shuffle in place
```

```rust
use rand::Rng;

let mut rng = rand::thread_rng();
let x: u32 = rng.gen_range(1..=10);
let y: f64 = rng.gen();
let z = fruits.choose(&mut rng);  // Random element
// Random subset = manual selection
// fruits.shuffle(&mut rng);  // Shuffle in place
```

---

## 6. Concept: Working with External Crates (Cargo.toml)

### Adding Dependencies

```toml
# Cargo.toml
[package]
name = "vector-fruit-salad"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

### How Cargo Resolves Versions

| Version spec | Meaning |
|---|---|
| `"0.8"` | `>=0.8.0` and `<0.9.0` (compatible with 0.8.x) |
| `"0.8.5"` | Exactly `0.8.5` |
| `"^0.8.5"` | Same as `"0.8.5"` — any 0.8.x >= 0.8.5 |
| `">=1.0"` | Any version 1.0 or higher |
| `"*"` | Any version (not recommended) |

### Python vs Cargo

| Python | Rust |
|---|---|
| `pip install pandas` | Add to `[dependencies]` in Cargo.toml |
| `requirements.txt` | `Cargo.toml` |
| `pip freeze` | `Cargo.lock` (auto-generated) |
| `import pandas` | `use some_crate::SomeType;` |
| Virtual environments | Each project is isolated |

---

## 7. Concept: SliceRandom — Shuffling and Choosing

### The SliceRandom Trait

`SliceRandom` is a trait from `rand` that adds methods to slices (`&[T]`):

```rust
use rand::seq::SliceRandom;
use rand::thread_rng;

let mut fruits = vec!["Apple", "Banana", "Cherry", "Date"];
let mut rng = thread_rng();

// Choose one random element
let pick = fruits.choose(&mut rng);  // Option<&&str>

// Shuffle in place
fruits.shuffle(&mut rng);  // Randomizes order

// Partial shuffle (first k elements randomized)
fruits.partial_shuffle(&mut rng, 2);
```

### Why `SliceRandom` Is a Trait

In Rust, methods like `.shuffle()` and `.choose()` aren't built into `Vec` — they're added via a **trait** that you import:

```rust
// Without the import, this won't compile:
// fruits.shuffle(&mut rng);
// ERROR: no method named `shuffle`

// With the import:
use rand::seq::SliceRandom;
// Now `.shuffle()` is available on any slice
```

### Python Equivalent

```python
import random

# choose() is like random.choice()
pick = random.choice(fruits)

# shuffle() is like random.shuffle()
random.shuffle(fruits)
```

---

## 8. Building Step by Step

### Step 1: Create the Project

```bash
cargo new vector-fruit-salad
cd vector-fruit-salad
```

### Step 2: Add rand Dependency

Edit `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

### Step 3: Define the Fruit List

```rust
// A constant array of fruit names — fixed at compile time
const FRUITS: [&str; 10] = [
    "Orange", "Apple", "Banana", "Pear", "Grape",
    "Watermelon", "Strawberry", "Cherry", "Plum", "Peach",
];
```

### Step 4: Import rand Types

```rust
use rand::Rng;                // For gen_range()
use rand::seq::SliceRandom;  // For shuffle(), choose()
use rand::thread_rng;        // Get a random number generator
```

### Step 5: Select Random Fruits

```rust
fn select_random_fruits(count: usize, fruits: &[&str], rng: &mut impl Rng) -> Vec<&str> {
    let mut selected = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..fruits.len());
        selected.push(fruits[idx]);
    }
    selected
}
```

### Step 6: Main Function

```rust
fn main() {
    let mut rng = thread_rng();

    // Pick a random number of fruits to include
    let fruit_count = rng.gen_range(1..=FRUITS.len());

    // Select that many random fruits
    let mut fruit_salad = select_random_fruits(fruit_count, &FRUITS, &mut rng);

    // Pick one random fruit to highlight
    let random_fruit = fruit_salad.choose(&mut rng);
    if let Some(fruit) = random_fruit {
        println!("Random fruit: {}", fruit);
    }

    // Shuffle the salad
    fruit_salad.shuffle(&mut rng);

    // Print the salad
    println!("Fruit salad:");
    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

---

## 9. Complete Code

```rust
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// The master list of available fruits — a fixed-size array
const FRUITS: [&str; 10] = [
    "Orange", "Apple", "Banana", "Pear", "Grape",
    "Watermelon", "Strawberry", "Cherry", "Plum", "Peach",
];

/// Select `count` random fruits from the given slice
fn select_random_fruits(count: usize, fruits: &[&str], rng: &mut ThreadRng) -> Vec<&str> {
    let mut selected = Vec::new();
    for _ in 0..count {
        let idx = rng.gen_range(0..fruits.len());
        selected.push(fruits[idx]);
    }
    selected
}

fn main() {
    let mut rng = thread_rng();

    // How many fruits in this salad?
    let fruit_count = rng.gen_range(1..=FRUITS.len());

    // Select the fruits
    let mut salad = select_random_fruits(fruit_count, &FRUITS, &mut rng);

    // Highlight one random fruit
    if let Some(fruit) = salad.choose(&mut rng) {
        println!("Random fruit: {}", fruit);
    }

    // Shuffle
    salad.shuffle(&mut rng);

    // Print the salad
    println!("Fruit salad:");
    for (i, item) in salad.iter().enumerate() {
        if i != salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

---

## 10. Exercises

### Exercise 1: No Duplicates

Modify `select_random_fruits` so it never selects the same fruit twice:

```rust
fn select_unique_fruits(count: usize, fruits: &[&str], rng: &mut ThreadRng) -> Vec<&str> {
    // Your code here — hint: use a loop that checks for duplicates
    // Or: shuffle a copy of fruits and take the first `count`
}
```

<details>
<summary>Solution</summary>

```rust
fn select_unique_fruits(count: usize, fruits: &[&str], rng: &mut ThreadRng) -> Vec<&str> {
    let mut available = fruits.to_vec();
    available.shuffle(rng);
    available.into_iter().take(count).collect()
}
```
</details>

### Exercise 2: Weighted Selection

Some fruits should appear more often. Add weights:

```rust
fn select_weighted(fruits: &[&str], weights: &[f64], count: usize, rng: &mut ThreadRng) -> Vec<&str> {
    // Use rng.gen_bool() or rand::distributions::WeightedIndex
}
```

### Exercise 3: Salad Statistics

After creating the salad, print statistics:

```
Fruit salad: Grape, Apple, Banana
Stats: 3 fruits, 2 unique types
```

---

## 11. Summary

| Concept | How Used | Data Engineering Analog |
|---|---|---|
| `Vec::new()` | Create empty fruit list | Create empty data collection |
| `vec![]` macro | Pre-populate fruits | Initialize data batch |
| `.push()` | Add selected fruit | Append to dataset |
| `.choose()` | Pick random fruit | Sample one row |
| `.shuffle()` | Randomize order | Randomize data for training |
| `.iter().enumerate()` | Print with indices | Iterate with position |
| `rng.gen_range()` | Random count | Random partitioning |

### Key Takeaway

Vectors in Rust = Lists in Python. The core operations are the same, but Rust gives you:
- **Type safety** — `Vec<&str>` can only hold string slices
- **Explicit cloning** — no accidental duplicate of large data
- **Trait-based extensions** — `.shuffle()` comes from importing `SliceRandom`, not built into Vec

### Next Project

Proceed to [10-ArrayFruitSalad](../03-Collections/10-ArrayFruitSalad/README.md) to compare **arrays** vs **vectors** in Rust.
