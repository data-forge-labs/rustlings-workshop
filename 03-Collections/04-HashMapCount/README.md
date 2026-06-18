# 🦀 HashMapCount — Python to Rust Workshop

*Frequency counting with `HashMap`, the `entry` API, and `BTreeMap` for sorted output.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 7 tests pass**.

---

## What Is This Project?

Word frequency counting with `HashMap`, the `entry` API, and `BTreeMap` for sorted output.

### Python equivalent

```python
from collections import Counter

text = "the cat sat on the mat"
counts = Counter(text.split())
print(counts.most_common(3))  # [('the', 2), ('cat', 1), ('sat', 1)]
```

```rust
// Rust — single hash lookup
for word in text.split_whitespace() {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
```

```python
# Python — three lookups
for word in text.split():
    counts[word] = counts.get(word, 0) + 1
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `HashMap` | `HashMap<K, V>` | `dict` | Hash-based key-value storage with typed K, V |
| 2 | Insert | `.insert(k, v)` | `d[k] = v` | Returns `Option<V>` (old value) |
| 3 | Safe get | `.get(k)` → `Option<&V>` | `d.get(k)` → `None` or value | No `KeyError` — the type forces a check |
| 4 | Entry API | `.entry(k).or_insert(0)` | `d.setdefault(k, 0)` | Insert-if-missing in a single lookup |
| 5 | Mutable entry | `.entry(k).or_insert_with(default)` | N/A | Lazy default — only computed if missing |
| 6 | Iteration | `for (k, v) in &map` | `for k, v in d.items()` | Borrowed iteration, no moves |
| 7 | Sorting by value | Convert to `Vec<(K, V)>` then `sort_by` | `sorted(d.items(), key=...)` | Rust's `sort_by_key` / `sort_by` |
| 8 | `BTreeMap` | `BTreeMap<K, V>` | `sortedcontainers.SortedDict` | Sorted iteration, same API as `HashMap` |

---

## Table of Contents
1. [Introduction to `HashMap`](#introduction-to-hashmap)
2. [Key Rust Concepts Explained](#key-rust-concepts-explained)
   - [What is `HashMap` and How is it Used?](#what-is-hashmap-and-how-is-it-used)
   - [The `entry` Method and `or_insert`](#the-entry-method-and-or_insert)
   - [Why `or_insert(0)`?](#why-or_insert0)
   - [Ensuring Correct Pairing](#ensuring-correct-pairing)
   - [Using `BTreeMap` for Sorting](#using-btreemap-for-sorting)
3. [Building the Program Step-by-Step](#building-the-program-step-by-step)
   - [Step 1: Basic `HashMap` for Frequency Counting](#step-1-basic-hashmap-for-frequency-counting)
   - [Step 2: Printing the Results](#step-2-printing-the-results)
   - [Step 3: Sorting by Number with `BTreeMap`](#step-3-sorting-by-number-with-btreemap)
   - [Step 4: Advanced Sorting by Frequency](#step-4-advanced-sorting-by-frequency)
4. [Challenge Explained: Sorting by Frequency](#challenge-explained-sorting-by-frequency)
5. [Additional Challenges](#additional-challenges)
6. [Running the Program](#running-the-program)
7. [Conclusion](#conclusion)

---

## Introduction to `HashMap`

A **HashMap** in Rust is a collection from the `std::collections` module that stores key-value pairs, allowing efficient lookup, insertion, and deletion based on keys. It uses a hash table internally, making operations like retrieving a value by its key average O(1) time complexity.

### Key Features of `HashMap`:
- **Key-Value Storage**: Maps keys of type `K` to values of type `V` (e.g., `HashMap<i32, u32>` maps `i32` numbers to their `u32` frequencies).
- **Dynamic Size**: Grows or shrinks as entries are added or removed.
- **Unordered**: Does not maintain insertion order (unlike `BTreeMap`).
- **Hashing**: Keys must implement the `Hash` and `Eq` traits for hashing and equality comparison.
- **Heap-Allocated**: Stores data on the heap, like `Vec`.

In the provided code, `HashMap` is used to count the frequency of each number in a vector, with numbers (`i32`) as keys and their frequencies (`u32`) as values.

---

## Key Rust Concepts Explained

### What is `HashMap` and How is it Used?

The `HashMap` in the program is defined as `HashMap<i32, u32>`:
- **Keys**: `i32` (signed integers, the numbers from the input vector).
- **Values**: `u32` (unsigned integers, the frequency of each number).

**How it’s Used**:
- **Initialization**: Created with `HashMap::new()` as an empty map.
- **Insertion/Update**: Iterates through the input vector, using the `entry` API to either:
  - Insert a new key with a frequency of 1 (if the number is new).
  - Increment the frequency (if the number already exists).
- **Purpose**: Tracks how many times each number appears in the vector.

**In the Code**:
```rust
let mut frequencies = HashMap::new();
for number in numbers {
    let frequency = frequencies.entry(number).or_insert(0);
    *frequency += 1;
}
```
- `frequencies` maps each number to its count.
- The loop processes each number, updating its frequency in the map.

### The `entry` Method and `or_insert`

The `entry` method is part of the `HashMap` API and provides a way to handle key-value pairs efficiently.

- **Definition**: `entry(key: K) -> Entry<K, V>`.
  - Takes a key and returns an `Entry` enum, which represents either:
    - `OccupiedEntry`: The key exists, with access to its value.
    - `VacantEntry`: The key does not exist, allowing insertion.
- **Purpose**: Simplifies the logic of checking if a key exists, inserting, or updating.

The `or_insert` method is called on an `Entry`:
- **Definition**: `or_insert(self, default: V) -> &mut V`.
  - If the key exists (`OccupiedEntry`), returns a mutable reference to the existing value.
  - If the key doesn’t exist (`VacantEntry`), inserts the `default` value and returns a mutable reference to it.

**In the Code**:
```rust
let frequency = frequencies.entry(number).or_insert(0);
*frequency += 1;
```
- `entry(number)` gets the `Entry` for `number`.
- `or_insert(0)`:
  - If `number` is not in the map, inserts `number -> 0` and returns a mutable reference to `0`.
  - If `number` exists, returns a mutable reference to its current frequency.
- `*frequency += 1` increments the frequency by 1.

### Why `or_insert(0)`?

The `or_insert(0)` method initializes the frequency to `0` for new keys, allowing the subsequent `+= 1` to set the correct count.

- **Why `0`?**:
  - When a number is encountered for the first time, its frequency should start at 0, then be incremented to 1.
  - This ensures the first occurrence sets the frequency to 1, the second to 2, and so on.
- **Alternative**:
  - Without `or_insert`, you’d need explicit checks:
    ```rust
    if frequencies.contains_key(&number) {
        *frequencies.get_mut(&number).unwrap() += 1;
    } else {
        frequencies.insert(number, 1);
    }
    ```
    This is verbose and less efficient, as it requires multiple lookups.

**Effect**:
- Creates a `HashMap` where each key (number) maps to its exact frequency in the input vector.

### Ensuring Correct Pairing

The program ensures each number and its frequency are correctly paired through the `HashMap`’s key-value structure and the `entry` API:
- **Key-Value Association**: `HashMap` guarantees that each key is unique and maps to exactly one value.
- **Entry API**: The `entry` method ensures atomic updates:
  - If the key exists, it updates the existing value.
  - If the key doesn’t exist, it inserts a new key-value pair.
- **Increment Logic**: `*frequency += 1` accurately increments the count for each occurrence, maintaining the correct frequency.

This approach avoids errors like duplicate keys or incorrect counts, ensuring the final `HashMap` accurately represents the input vector’s frequencies.

### Using `BTreeMap` for Sorting

The code converts the `HashMap` to a `BTreeMap` to sort the output by key (number):
```rust
let result: BTreeMap<&i32, &u32> = result.iter().collect();
```

- **What is `BTreeMap`?**:
  - A collection from `std::collections` that stores key-value pairs in a balanced binary tree.
  - Unlike `HashMap`, it maintains keys in sorted order (requires keys to implement `Ord`).
- **Why Use It?**:
  - `HashMap` is unordered, so its iteration order is unpredictable.
  - `BTreeMap` sorts keys (here, `&i32`), making the output consistent and sorted by number.
- **In the Code**:
  - `result.iter()` produces an iterator of `(&i32, &u32)` (references to the `HashMap`’s keys and values).
  - `collect()` builds a `BTreeMap<&i32, &u32>` from the iterator, sorting keys automatically.
- **Output Effect**:
  - Numbers are printed in ascending order (e.g., `1, 2, 3, ...`).

---

## Building the Program Step-by-Step

We’ll build the frequency-counting program incrementally, starting with a basic `HashMap` and progressing to advanced sorting and error handling.

### Step 1: Basic `HashMap` for Frequency Counting

**Goal**: Count the frequency of numbers in a vector using a `HashMap`.

**Code**:
```rust
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> HashMap<i32, u32> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frequency = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }

    frequencies
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 1];
    let result = logic(numbers);
    println!("Frequencies: {:?}", result);
}
```

**Explanation**:
- **HashMap Creation**:
  - `HashMap::new()` creates an empty `HashMap<i32, u32>`.
- **Frequency Counting**:
  - Iterates through `numbers`.
  - Uses `entry(number).or_insert(0)` to get a mutable reference to the frequency, initializing to `0` if new.
  - Increments the frequency with `*frequency += 1`.
- **Output**:
  - Prints the `HashMap` using the `Debug` formatter (`{:?}`).
- **Concepts**:
  - `HashMap` initialization and key-value insertion.
  - `entry` and `or_insert` for efficient updates.
  - Dereferencing (`*`) to modify the value.

**Output**:
```
Frequencies: {1: 2, 2: 2, 3: 1}
```

### Step 2: Printing the Results

**Goal**: Improve the output by printing each number and its frequency in a readable format.

**Code**:
```rust
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> HashMap<i32, u32> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frequency = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }

    frequencies
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 1];
    let result = logic(numbers);

    println!("Number frequencies:");
    for (number, frequency) in &result {
        println!("Number {}: {} times", number, frequency);
    }
}
```

**New Concepts**:
- **Iteration**:
  - `&result` borrows the `HashMap`, allowing iteration without consuming it.
  - `for (number, frequency) in &result` iterates over `(&i32, &u32)` pairs.
- **Formatted Output**:
  - Uses `println!` with placeholders for a clear, human-readable format.
- **Borrowing**:
  - Iterating over `&result` avoids moving the `HashMap`, preserving it for further use.

**Output**:
```
Number frequencies:
Number 1: 2 times
Number 2: 2 times
Number 3: 1 times
```
(Note: Order may vary due to `HashMap`’s unordered nature.)

### Step 3: Sorting by Number with `BTreeMap`

**Goal**: Sort the output by number using `BTreeMap`.

**Code**:
```rust
use std::collections::{BTreeMap, HashMap};

fn logic(numbers: Vec<i32>) -> HashMap<i32, u32> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frequency = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }

    frequencies
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 1];
    let result = logic(numbers);

    // Convert to BTreeMap for sorted output
    let sorted_result: BTreeMap<&i32, &u32> = result.iter().collect();

    println!("Number frequencies (sorted by number):");
    for (number, frequency) in &sorted_result {
        println!("Number {}: {} times", number, frequency);
    }
}
```

**New Concepts**:
- **BTreeMap**:
  - `BTreeMap<&i32, &u32>` stores references to the `HashMap`’s keys and values, sorted by key.
  - `result.iter().collect()` builds the `BTreeMap` from the `HashMap`’s iterator.
- **Sorted Iteration**:
  - Iterating over `&sorted_result` yields pairs in ascending key order.
- **Ownership**:
  - `iter()` yields references, avoiding ownership transfer.
  - `sorted_result` borrows from `result`, which remains usable.

**Output**:
```
Number frequencies (sorted by number):
Number 1: 2 times
Number 2: 2 times
Number 3: 1 times
```

### Step 4: Advanced Sorting by Frequency

**Goal**: Sort the output by frequency (descending) and then by number (ascending) for equal frequencies, addressing Challenge 3.

**Code**:
```rust
use std::collections::{BTreeMap, HashMap};

fn logic(numbers: Vec<i32>) -> Result<HashMap<i32, u32>, String> {
    if numbers.is_empty() {
        return Err("Input vector is empty".to_string());
    }

    let mut frequencies = HashMap::new();

    for number in numbers {
        let frequency = frequencies.entry(number).or_insert(0);
        *frequency += 1;
    }

    Ok(frequencies)
}

fn main() -> Result<(), String> {
    let numbers = vec![1, 2, 3, 4, 7, 7, 5, 6, 1, 7, 1, 8, 2, 2, 2, 9, 10];
    let result = logic(numbers)?;

    // Sort by frequency (descending) and number (ascending)
    let mut sorted_by_freq: Vec<(&i32, &u32)> = result.iter().collect();
    sorted_by_freq.sort_by(|a, b| {
        b.1.cmp(a.1) // Sort by frequency (descending)
            .then(a.0.cmp(b.0)) // Then by number (ascending)
    });

    println!("Number frequencies (sorted by frequency, then number):");
    for (number, frequency) in sorted_by_freq {
        println!("Number {}: {} times", number, frequency);
    }

    // Also print sorted by number using BTreeMap
    let sorted_by_number: BTreeMap<&i32, &u32> = result.iter().collect();
    println!("\nNumber frequencies (sorted by number):");
    for (number, frequency) in &sorted_by_number {
        println!("Number {}: {} times", number, frequency);
    }

    Ok(())
}
```

**New Concepts**:
- **Error Handling**:
  - `logic` returns `Result<HashMap<i32, u32>, String>` to handle empty input.
  - `main` returns `Result<(), String>` for error propagation.
  - Uses `?` operator to propagate errors.
- **Custom Sorting**:
  - Converts `HashMap` to a `Vec<(&i32, &u32)>` for sorting.
  - `sort_by` with a closure:
    - `b.1.cmp(a.1)` sorts by frequency in descending order (`b` before `a` for higher frequencies).
    - `.then(a.0.cmp(b.0))` breaks ties by number in ascending order.
- **Multiple Outputs**:
  - Shows both frequency-sorted and number-sorted results.
  - Reuses `BTreeMap` for number-sorted output.
- **Ownership and Borrowing**:
  - `iter()` and `collect()` work with references to avoid moving `result`.
  - `sorted_by_freq` owns its `Vec`, but its elements are references to `result`.

**Output** (example):
```
Number frequencies (sorted by frequency, then number):
Number 2: 5 times
Number 1: 3 times
Number 7: 3 times
Number 3: 1 times
Number 4: 1 times
Number 5: 1 times
Number 6: 1 times
Number 8: 1 times
Number 9: 1 times
Number 10: 1 times

Number frequencies (sorted by number):
Number 1: 3 times
Number 2: 5 times
Number 3: 1 times
Number 4: 1 times
Number 5: 1 times
Number 6: 1 times
Number 7: 3 times
Number 8: 1 times
Number 9: 1 times
Number 10: 1 times
```

---

## Challenge Explained: Sorting by Frequency

**Challenge**: Modify the program to sort the final result by frequency.

**Solution** (Implemented in Step 4):
- **Approach**:
  - Convert the `HashMap` to a `Vec<(&i32, &u32)>` using `iter().collect()`.
  - Use `sort_by` with a custom comparator:
    - Primary sort: Frequency (`u32`) in descending order (`b.1.cmp(a.1)`).
    - Secondary sort: Number (`i32`) in ascending order for equal frequencies (`.then(a.0.cmp(b.0))`).
- **Why Not `BTreeMap`?**:
  - `BTreeMap` sorts by keys, but we want to sort by values (frequencies).
  - A `Vec` with `sort_by` allows flexible sorting on any field.
- **Implementation**:
  ```rust
  let mut sorted_by_freq: Vec<(&i32, &u32)> = result.iter().collect();
  sorted_by_freq.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
  ```
- **Effect**:
  - Numbers with higher frequencies appear first.
  - For equal frequencies, numbers are sorted ascending (e.g., `1` before `7` for frequency `3`).

---

## Additional Challenges

To further explore `HashMap`, sorting, and Rust collections, try these challenges:

1. **Filter by Frequency**:
   Only include numbers with a frequency above a user-specified threshold in the output.
   **Hint**: Use `filter` on the `Vec` before sorting.

2. **Group by Frequency**:
   Group numbers by their frequency (e.g., `{3: [1, 7], 5: [2], 1: [3, 4, ...]}`).
   **Hint**: Use a `HashMap<u32, Vec<i32>>`.

3. **Input Validation**:
   Add checks for invalid input (e.g., negative numbers) and return an error.
   **Hint**: Modify `logic` to validate each number.

4. **Interactive Input**:
   Allow users to input numbers via the command line.
   **Hint**: Use `std::io` to read and parse input.

5. **Most/Least Frequent**:
   Print only the number(s) with the highest and lowest frequencies.
   **Hint**: Track max/min frequencies during iteration or after sorting.

6. **Serialize Output**:
   Save the frequency map to a JSON file.
   **Hint**: Use the `serde` and `serde_json` crates.

---

## Running the Program

For any step:
1. Create a new Rust project:
   ```bash
   cargo new frequency_counter
   cd frequency_counter
   ```
2. Update `Cargo.toml` (Step 4 requires no additional dependencies):
   ```toml
   [dependencies]
   ```
3. Copy the code for the desired step into `workshop/src/main.rs`.
4. Run:
   ```bash
   cd workshop && cargo run
   ```

---

## Conclusion

This tutorial built a frequency-counting program from a basic `HashMap` implementation to an advanced version with sorting and error handling. We covered:
- **HashMap**: Key-value storage for efficient frequency counting.
- **Entry API**: `entry` and `or_insert` for atomic updates.
- **BTreeMap**: Sorted output by key.
- **Custom Sorting**: Sorting by frequency using `Vec` and `sort_by`.
- **Error Handling**: Using `Result` for robust code.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

