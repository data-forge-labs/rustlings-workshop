# 🦀 ArrayFruitSalad — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 7 tests pass**.

## Why Fixed-Size Arrays?

Ownership note: In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.


---

## Why Fixed-Size Arrays?

Ownership note: In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.


**Python pain:** Python lists are always dynamic — there's no way to say "this collection has *exactly* 10 elements and never changes." You pay a hidden cost for every check (`len(self) == expected`), and off-by-one bugs hide in append/remove logic.

**Rust fix:** `[T; N]` makes the size *part of the type*. The compiler knows the array is always 10 elements, allocates it on the stack (no heap), and rejects any attempt to add or remove elements.

```rust
// Rust — size is part of the type
let scores: [i32; 5] = [90, 80, 85, 95, 88];  // exactly 5 elements
let rgb: [u8; 3] = [255, 128, 0];              // 3 bytes, stack-only
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Fixed-size arrays | `[T; N]` | `array.array` | Stack-allocated, compile-time-known length |
| 2 | Stack vs heap | Stack allocation | All on heap | Arrays live on the stack — no GC, no allocator |
| 3 | `Copy` trait | `T: Copy` | Always copies | Arrays of `Copy` types are themselves `Copy` |
| 4 | `vec![]` macro | `vec![a, b, c]` | `[a, b, c]` | Convert fixed array to dynamic Vec in one call |
| 5 | `from_fn` / `map` | `[0; 5]` or `arr.map(\|x\| x*2)` | list comprehension | Build arrays from expressions |
| 6 | Array indexing | `arr[i]` | `arr[i]` | O(1) access, panics on out-of-bounds |
| 7 | Array vs Vec | `Vec<T>` (dynamic) | `list` (always dynamic) | Choose stack `[T;N]` for fixed, heap `Vec` for growth |
| 8 | `SliceRandom` | `fruits.shuffle(&mut rng)` | `random.shuffle(fruits)` | Shuffle via the `rand` crate trait |

---

## Table of Contents
1. [Introduction to Arrays](#introduction-to-arrays)
2. [Reflection Questions](#reflection-questions)
   - [What is an Array in Rust and How is it Different from `Vec`, `VecDeque`, or `LinkedList`?](#what-is-an-array-in-rust-and-how-is-it-different-from-vec-vecdeque-or-linkedlist)
   - [When to Prefer Arrays Over Other Data Structures?](#when-to-prefer-arrays-over-other-data-structures)
   - [Why Convert Array to `Vec` for Shuffling in This Program?](#why-convert-array-to-vec-for-shuffling-in-this-program)
3. [Key Rust Concepts Explained](#key-rust-concepts-explained)
   - [Traits and Types from `rand`](#traits-and-types-from-rand)
   - [Array Indexing and Iteration](#array-indexing-and-iteration)
4. [Building the Program Step-by-Step](#building-the-program-step-by-step)
   - [Step 1: Basic Array Creation and Printing](#step-1-basic-array-creation-and-printing)
   - [Step 2: Shuffling with Conversion to `Vec`](#step-2-shuffling-with-conversion-to-vec)
   - [Step 3: Adding Fruits via `Vec` and Converting Back to Array](#step-3-adding-fruits-via-vec-and-converting-back-to-array)
   - [Step 4: Advanced Features (Error Handling, Random Selection)](#step-4-advanced-features-error-handling-random-selection)
5. [Additional Challenges](#additional-challenges)
6. [Running the Program](#running-the-program)
7. [Conclusion](#conclusion)

---

## Introduction to Arrays

An **array** in Rust is a fixed-size, contiguous collection of elements of the same type, represented as `[T; N]`, where `T` is the element type and `N` is the length, known at compile time. Arrays are stack-allocated (unless boxed) and provide fast, O(1) random access to elements via indexing.

### Key Features of Arrays:
- **Fixed Size**: The length is set at compile time and cannot change.
- **Contiguous Memory**: Elements are stored sequentially, ensuring excellent cache locality.
- **Random Access**: O(1) access via indexing (e.g., `array[0]`).
- **Immutable by Default**: Elements can be modified only if the array is mutable.
- **Type Safety**: All elements must be of type `T`.

In this tutorial, weâ€™ll use an array to store a fixed-size list of fruit names, shuffle them (via conversion to `Vec`), and perform operations like adding or removing fruits, demonstrating array usage in a constrained context.

---

## Reflection Questions

### What is an Array in Rust and How is it Different from `Vec`, `VecDeque`, or `LinkedList`?

**Array (`[T; N]`)**:
- A fixed-size, stack-allocated collection with a compile-time-known length.
- Ideal for scenarios with a known, unchanging number of elements and a need for fast random access.
- Operations like insertion or removal are not directly supported, as the size is fixed.

**Comparison with `Vec`, `VecDeque`, and `LinkedList`**:

| Feature                | `Array [T; N]`                   | `Vec<T>`                          | `VecDeque<T>`                     | `LinkedList<T>`                   |
|------------------------|-----------------------------------|-----------------------------------|-----------------------------------|-----------------------------------|
| **Structure**          | Fixed-size contiguous array      | Resizable contiguous array       | Ring buffer                      | Doubly-linked list               |
| **Size**               | Fixed at compile time            | Dynamic, resizable               | Dynamic, resizable               | Dynamic, resizable               |
| **Insertion/Removal**  | Not supported (fixed size)       | O(1) at back, O(n) elsewhere     | O(1) at front/back, O(n) middle  | O(1) at known positions          |
| **Random Access**      | O(1) (indexing)                  | O(1) (indexing)                  | O(1) (indexing)                  | O(n) (no indexing)               |
| **Memory Efficiency**  | Highly compact, no overhead      | Compact, minimal overhead        | Moderate overhead (ring buffer)  | High overhead (node pointers)    |
| **Cache Locality**     | Excellent                        | Excellent                        | Good                             | Poor                             |
| **Use Case**           | Fixed-size data, random access   | Dynamic arrays, random access    | Double-ended queues              | Frequent insertions/removals     |

**In the Program**:
- An array `[&str; 6]` is used to store a fixed list of fruit names.
- Unlike `Vec` (resizable), `VecDeque` (double-ended), or `LinkedList` (flexible insertions), arrays are rigid but efficient for static data.

### When to Prefer Arrays Over Other Data Structures?

**Preferred Scenarios**:
1. **Known, Fixed Size**:
   - When the number of elements is known at compile time and wonâ€™t change.
   - Example: Storing the days of the week (`["Mon", "Tue", ..., "Sun"]`).
2. **Performance-Critical Random Access**:
   - Arrays offer O(1) indexing with excellent cache locality, ideal for frequent element access.
   - Example: Lookup tables or configuration data.
3. **Stack Allocation**:
   - Arrays are stack-allocated, avoiding heap allocation overhead.
   - Example: Small, fixed-size buffers in performance-sensitive code.
4. **Compile-Time Guarantees**:
   - The fixed size ensures safety and predictability, enforced by the compiler.
   - Example: Representing a 3D point `[f32; 3]` with exactly three coordinates.

**Limitations**:
- **No Resizing**: Cannot add or remove elements, unlike `Vec`, `VecDeque`, or `LinkedList`.
- **Fixed Memory**: Large arrays on the stack can cause stack overflows; use `Box<[T; N]>` for heap allocation.
- **When to Avoid**:
  - For dynamic sizes, use `Vec` or `VecDeque`.
  - For frequent insertions/removals, use `VecDeque` or `LinkedList`.

**In Practice**:
- Arrays are used for small, fixed-size data where performance and simplicity are priorities. For dynamic collections, other structures are typically preferred.

### Why Convert Array to `Vec` for Shuffling in This Program?

The program converts the array to a `Vec` for shuffling:
```rust
let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
fruit_vec.shuffle(&mut rng);
```

**Why Convert?**:
- **Shuffling Requirement**: The `SliceRandom` traitâ€™s `shuffle` method is implemented for `Vec` and slices, which support efficient random access. Arrays support indexing but lack a direct `shuffle` method.
- **Conversion to `Vec`**:
  - `into_iter().collect()` converts the array to a `Vec`, enabling `shuffle`.
  - Arrays are fixed-size, so shuffling requires a resizable structure like `Vec` to handle temporary transformations.
- **Shuffling**: `fruit_vec.shuffle(&mut rng)` randomizes the `Vec`â€™s order using the `rand` crate.
- **Conversion Back to Array**:
  - The program converts back to an array to maintain a fixed-size structure, assuming the final size matches the arrayâ€™s capacity (e.g., `[&str; 6]`).
  - This is less common in practice, as `Vec` is often preferred for dynamic operations, but it demonstrates array usage in a constrained context.
- **Trade-Off**: Conversion is O(n) due to copying, but itâ€™s necessary for shuffling.
- **Alternative**: Shuffle the array in place using index swaps, but `Vec`â€™s `shuffle` is more convenient and optimized.

---

## Key Rust Concepts Explained

### Traits and Types from `rand`

The program uses the `rand` crate for shuffling and random selection:
1. **`rand::seq::SliceRandom`**:
   - **What is it?**: A trait providing methods for random operations on slices and `Vec`, such as `shuffle` and `choose`.
   - **Key Methods**:
     - `shuffle(&mut self, rng: &mut R)`: Randomly reorders elements.
     - `choose(&self, rng: &mut R) -> Option<&T>`: Selects a random element.
   - **In the Code**:
     ```rust
     fruit_vec.shuffle(&mut rng);
     fruit_vec.choose(&mut rng);
     ```
     - Shuffles the `Vec` and selects random fruits.
   - **Why Use It?**: Efficient, safe randomization for contiguous collections.

2. **`rand::rng`**:
   - **What is it?**: A function returning a thread-local random number generator (`ThreadRng`).
   - **In the Code**:
     ```rust
     let mut rng = rng();
     ```
     - Creates `rng` for shuffling and selection.
   - **Why Use It?**: Convenient, thread-safe randomness source.

**Adding `rand` to Your Project**:
Include in `Cargo.toml`:
```toml
[dependencies]
rand = "0.10"
```

### Array Indexing and Iteration

- **Indexing**:
  - Arrays support O(1) access via `array[index]`, e.g., `fruit[0]`.
  - Bounds are checked at runtime, panicking on out-of-bounds access.
  - Example: `fruit[0] = "New Fruit"` (if mutable).
- **Iteration**:
  - `iter()` yields references (`&T`) to elements, e.g., `fruit.iter()`.
  - `into_iter()` consumes the array, yielding elements by value.
  - `enumerate()` pairs indices with elements, used for printing.
- **In the Code**:
  - Iteration is used for printing (`fruit.iter().enumerate()`).
  - Conversion to `Vec` uses `into_iter()`.

---

## Building the Program Step-by-Step

Weâ€™ll build a fruit salad program using an array, starting with basic operations and progressing to advanced features. The array will have a fixed size (`[&str; 6]`), and weâ€™ll handle size constraints explicitly.

### Step 1: Basic Array Creation and Printing

**Goal**: Create an array of fruits and print them.

```rust
fn main() {
    let fruit: [&str; 3] = ["Arbutus", "Loquat", "Strawberry Tree Berry"];

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

**Explanation**:
- **Array Creation**:
  - `let fruit: [&str; 3]` defines a fixed-size array of 3 string slices.
  - Initialized with three fruits.
- **Printing**:
  - `fruit.iter().enumerate()` iterates over references to elements, pairing with indices.
  - Comma-separated formatting omits the comma for the last item.
- **Concepts**:
  - Array initialization with fixed size.
  - Immutable iteration with `.iter()`.
  - Stack allocation for efficiency.

**Output**:
```
Fruit Salad:
Arbutus, Loquat, Strawberry Tree Berry
```

### Step 2: Shuffling with Conversion to `Vec`

**Goal**: Shuffle the fruits by converting the array to a `Vec`.

```rust
use rand::seq::SliceRandom;
use rand::rng;

fn main() {
    let fruit: [&str; 3] = ["Arbutus", "Loquat", "Strawberry Tree Berry"];

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert back to array
    let fruit: [&str; 3] = fruit_vec.try_into().unwrap();

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

**New Concepts**:
- **Randomization**:
  - `rng()` creates a random number generator.
  - `fruit_vec.shuffle(&mut rng)` randomizes the `Vec`â€™s order.
- **Conversion**:
  - `into_iter().collect()` converts the array to a `Vec<&str>`.
  - `try_into().unwrap()` converts the `Vec` back to `[&str; 3]`, assuming the size matches.
- **Traits**:
  - `SliceRandom` for `shuffle`.
- **Ownership**:
  - `into_iter()` consumes the array, yielding elements.
  - `try_into` ensures the `Vec` has exactly 3 elements.

**Output** (example):
```
Fruit Salad:
Loquat, Arbutus, Strawberry Tree Berry
```

### Step 3: Adding Fruits via `Vec` and Converting Back to Array

**Goal**: Add fruits to create a larger array, using `Vec` as an intermediate step.

```rust
use rand::seq::SliceRandom;
use rand::rng;

fn main() {
    let fruit: [&str; 3] = ["Arbutus", "Loquat", "Strawberry Tree Berry"];

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Add fruits
    fruit_vec.insert(0, "Pomegranate");
    fruit_vec.push("Fig");
    fruit_vec.push("Cherry");

    // Convert back to array (fixed size 6)
    let fruit: [&str; 6] = fruit_vec.try_into().unwrap();

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

**New Concepts**:
- **Adding Elements**:
  - `fruit_vec.insert(0, "Pomegranate")` adds at the front (O(n) due to shifting).
  - `fruit_vec.push("Fig")` and `push("Cherry")` add at the back (O(1) amortized).
  - Arrays canâ€™t resize, so `Vec` is used for flexibility.
- **Conversion Back**:
  - `try_into().unwrap()` converts the `Vec` to `[&str; 6]`, matching the new size.
  - Panics if the `Vec` size doesnâ€™t match (addressed in Step 4).
- **Purpose**:
  - Mimics adding fruits to the salad, constrained by the arrayâ€™s fixed size.

**Output** (example):
```
Fruit Salad:
Pomegranate, Loquat, Arbutus, Strawberry Tree Berry, Fig, Cherry
```

### Step 4: Advanced Features (Error Handling, Random Selection)

**Goal**: Add error handling, random selection, and a separate printing function.

```rust
use rand::seq::SliceRandom;
use rand::rng;

fn main() -> Result<(), String> {
    let fruit: [&str; 3] = ["Arbutus", "Loquat", "Strawberry Tree Berry"];

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Add fruits
    fruit_vec.insert(0, "Pomegranate");
    fruit_vec.push("Fig");
    fruit_vec.push("Cherry");

    // Check size before conversion
    if fruit_vec.len() != 6 {
        return Err(format!("Expected 6 fruits, got {}", fruit_vec.len()));
    }
    let fruit: [&str; 6] = fruit_vec.try_into().map_err(|_| "Conversion to array failed")?;

    print_fruit_salad(&fruit)?;

    // Randomly select a fruit
    let random_fruit = fruit.choose(&mut rng).ok_or("Array is empty")?;
    println!("Random fruit: {}", random_fruit);

    // Create a new array excluding the last fruit
    let mut new_fruit: [&str; 5] = Default::default();
    new_fruit.copy_from_slice(&fruit[..5]);

    println!("Fruit salad after removing last fruit:");
    print_fruit_salad(&new_fruit)?;

    Ok(())
}

fn print_fruit_salad(fruit: &[&str]) -> Result<(), String> {
    if fruit.is_empty() {
        return Err("Fruit salad is empty".to_string());
    }
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    Ok(())
}
```

**New Concepts**:
- **Error Handling**:
  - `main` returns `Result<(), String>` for error propagation.
  - `print_fruit_salad` takes a slice (`&[&str]`) and returns `Result` to handle empty arrays.
  - Size check before `try_into` prevents panics.
  - `try_into().map_err()` handles conversion errors.
- **Random Selection**:
  - `fruit.choose(&mut rng)` picks a random element from the array (as a slice).
  - `.ok_or()` handles the empty case (though impossible for a fixed-size array).
- **Removing Elements**:
  - Arrays canâ€™t resize, so a new array `[&str; 5]` is created, copying the first 5 elements.
  - `copy_from_slice` copies data from a slice to the array.
- **Slice Usage**:
  - `print_fruit_salad` uses a slice for flexibility, working with arrays or `Vec`s.
  - `&fruit[..5]` creates a slice for copying.
- **Default Initialization**:
  - `Default::default()` initializes the new array with null pointers (safe for `&str` after copying).

**Output** (example):
```
Fruit Salad:
Pomegranate, Loquat, Arbutus, Strawberry Tree Berry, Fig, Cherry
Random fruit: Arbutus
Fruit salad after removing last fruit:
Fruit Salad:
Pomegranate, Loquat, Arbutus, Strawberry Tree Berry, Fig
```

**Error Case** (wrong size):
Modify `fruit_vec` to have 5 elements:
```rust
fruit_vec.pop(); // Remove one fruit
```
Output:
```
Error: Expected 6 fruits, got 5
```

---

## Additional Challenges

To further explore arrays, randomization, and Rust collections, try these challenges:

1. **In-Place Shuffle**:
   Implement a shuffle function for the array without converting to `Vec`.
   **Hint**: Use index swaps with `rng.random_range`.

2. **Replace Fruit**:
   Replace a specific fruit in the array with another (e.g., replace "Loquat" with "Mango").
   **Hint**: Iterate and update via indexing.

3. **Sort Fruits**:
   Sort the array by fruit name length before printing.
   **Hint**: Convert to `Vec`, sort, and convert back.

4. **Interactive Array**:
   Allow users to specify initial fruits via the command line, ensuring exactly 3 inputs.
   **Hint**: Use `std::io` and validate input length.

5. **Reverse Array**:
   Create a new array with fruits in reverse order.
   **Hint**: Use a loop or `into_iter().rev()`.

6. **Serialize Array**:
   Save the array to a JSON file and load it later.
   **Hint**: Use `serde` and `serde_json`.

---

## Running the Program

For any step:
1. Create a new Rust project:
   ```bash
   cargo new fruit_salad_array
   cd fruit_salad_array
   ```
2. Update `Cargo.toml` (for Steps 2â€“4):
   ```toml
   [dependencies]
   rand = "0.10"
   ```
3. Copy the code for the desired step into `workshop/src/main.rs`.
4. Run:
   ```bash
   cd workshop && cargo run
   ```

---

## Conclusion

This tutorial built a fruit salad program using arrays, from basic creation to advanced features like error handling and random selection. We covered:
- **Arrays**: Fixed-size, contiguous collections with fast random access.
- **Comparison**: Differences from `Vec` (resizable), `VecDeque` (double-ended), and `LinkedList` (flexible insertions).
- **Conversions**: Converting to `Vec` for shuffling and back to an array for fixed-size storage.
- **Use Cases**: Scenarios where arrays excel (fixed-size, performance-critical) and their limitations (no resizing).
- **Randomization**: Using `rand`â€™s `SliceRandom` for shuffling and selection.
- **Advanced Features**: Error handling, slice operations, and array manipulation.


## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

