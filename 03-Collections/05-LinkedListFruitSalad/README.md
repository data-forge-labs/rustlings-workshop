# 🦀 LinkedListFruitSalad — Python to Rust Workshop

*The doubly-linked list: when you actually need O(1) splice/split, and when you don't.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 4 tests pass**.

---

## What Is This Project?

A deep dive into `LinkedList<T>` — when a doubly-linked list is the right choice, and when `Vec` or `VecDeque` is better.

### Python equivalent

```python
from collections import deque

# Python's deque is fast at both ends, but not a true linked list
queue = deque(["Apple", "Banana"])
queue.appendleft("Cherry")  # O(1)
queue.pop()                  # O(1)
```

```rust
use std::collections::LinkedList;

let mut list: LinkedList<&str> = LinkedList::new();
list.push_back("Apple");
list.push_front("Banana");  // O(1) at the front
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Doubly-linked list | `LinkedList<T>` | (no stdlib equivalent) | O(1) splice/split, O(n) traversal |
| 2 | Push/pop ends | `push_front`, `push_back`, `pop_front`, `pop_back` | `deque.append` / `pop` | O(1) at both ends |
| 3 | Cursor API | `.cursor_front_mut()` | N/A | O(1) insertion/remove at any cursor position |
| 4 | Splice / split | `.append(&mut other)`, `.split_off(idx)` | N/A | Move sub-lists in O(1) |
| 5 | Iteration | `for x in &list` | `for x in deque` | Bidirectional, but no random access |
| 6 | No indexing | `list[i]` doesn't exist | `deque[i]` works (O(n)) | Compile-time error forces you to think |
| 7 | Conversion | `Vec::from(list)` / `LinkedList::from(vec)` | `list(deque)` | Convert when you need random access |
| 8 | When *not* to use | Almost always prefer `Vec`/`VecDeque` | `deque` is usually fine | Cache locality matters more than splice |

---

## Table of Contents
1. [Introduction to `LinkedList`](#introduction-to-linkedlist)
2. [Key Rust Concepts Explained](#key-rust-concepts-explained)
   - [What is `LinkedList` and How is it Different from `Vec` or `VecDeque`?](#what-is-linkedlist-and-how-is-it-different-from-vec-or-vecdeque)
   - [When to Prefer `LinkedList` Over Other Data Structures?](#when-to-prefer-linkedlist-over-other-data-structures)
   - [Why Convert `LinkedList` to `Vec` and Back?](#why-convert-linkedlist-to-vec-and-back)
   - [Traits and Types from `rand`](#traits-and-types-from-rand)
3. [Building the Program Step-by-Step](#building-the-program-step-by-step)
   - [Step 1: Basic `LinkedList` Creation and Printing](#step-1-basic-linkedlist-creation-and-printing)
   - [Step 2: Shuffling with Conversion to `Vec`](#step-2-shuffling-with-conversion-to-vec)
   - [Step 3: Double-Ended Operations](#step-3-double-ended-operations)
   - [Step 4: Advanced Features (Error Handling, Middle Insertions)](#step-4-advanced-features-error-handling-middle-insertions)
4. [Additional Challenges](#additional-challenges)
5. [Running the Program](#running-the-program)
6. [Conclusion](#conclusion)

---

## Introduction to `LinkedList`

A **LinkedList** in Rust is a collection from the `std::collections` module that implements a doubly-linked list. Each element (node) contains data and pointers to the next and previous nodes, allowing efficient insertions and removals at any position.

### Key Features of `LinkedList`:
- **Doubly-Linked**: Each node has pointers to both the next and previous nodes, enabling bidirectional traversal.
- **Efficient Insertions/Removals**: O(1) for insertions and removals at known positions (e.g., front, back, or via a cursor).
- **Dynamic Size**: Grows or shrinks as needed, like `Vec` or `VecDeque`.
- **Heap-Allocated**: Nodes are individually allocated on the heap.
- **No Random Access**: No indexing (e.g., `list[3]`), unlike `Vec` or `VecDeque`.

In the provided code, `LinkedList` is used to store fruit names, shuffle them (via conversion to `Vec`), and perform operations at both ends, demonstrating its flexibility for certain use cases.

---

## Key Rust Concepts Explained

### What is `LinkedList` and How is it Different from `Vec` or `VecDeque`?

**`LinkedList`**:
- A doubly-linked list where each node contains data and pointers to the next and previous nodes.
- Ideal for frequent insertions/removals at arbitrary positions, as no element shifting is required.
- Drawbacks include poor cache locality (non-contiguous memory) and high memory overhead (pointers per node).

**Comparison with `Vec` and `VecDeque`**:

| Feature                | `Vec<T>`                          | `VecDeque<T>`                     | `LinkedList<T>`                   |
|------------------------|-----------------------------------|-----------------------------------|-----------------------------------|
| **Structure**          | Contiguous array                 | Ring buffer                      | Doubly-linked list               |
| **Insertion/Removal**  | O(1) at back, O(n) elsewhere     | O(1) at front/back, O(n) middle  | O(1) at known positions          |
| **Random Access**      | O(1) (indexing)                  | O(1) (indexing)                  | O(n) (no indexing)               |
| **Memory Efficiency**  | Compact, contiguous              | Moderate overhead (ring buffer)  | High overhead (node pointers)    |
| **Cache Locality**     | Excellent                        | Good                             | Poor                             |
| **Use Case**           | Dynamic arrays, random access    | Double-ended queues              | Frequent insertions/removals     |

**In the Code**:
- `LinkedList<&str>` stores fruit names, leveraging its ability to:
  - Add elements at the front or back (`push_front`, `push_back`).
  - Support iteration for printing.
- Unlike `Vec`, which is inefficient for front insertions, or `VecDeque`, which is less efficient for middle insertions, `LinkedList` excels in scenarios requiring flexible insertions/removals.

### When to Prefer `LinkedList` Over Other Data Structures?

**Preferred Scenarios**:
1. **Frequent Insertions/Removals at Arbitrary Positions**:
   - Inserting or removing elements in the middle is O(1) if you have a cursor to the position, unlike `Vec` or `VecDeque`, which require shifting (O(n)).
   - Example: Maintaining a playlist where songs are frequently added/removed at specific positions.
2. **Memory Allocation Flexibility**:
   - Each node is allocated separately, avoiding large contiguous memory blocks.
   - Useful in systems with fragmented memory or varying collection sizes.
3. **Element-Wise Processing**:
   - Efficient for traversing and modifying elements based on conditions (e.g., removing nodes matching a criterion).
   - Example: Filtering a list of tasks based on priority.
4. **Specialized Algorithms**:
   - Useful in algorithms requiring node-based structures, like graph traversals or custom queue implementations.
   - Example: Implementing a mergeable list for a divide-and-conquer algorithm.

**Cache Locality Concerns**:
- **Poor Cache Locality**: Nodes are scattered in memory, leading to frequent cache misses on modern CPUs, which optimize for contiguous data access.
- **Memory Overhead**: Each node stores two pointers (next and previous), doubling or tripling memory usage compared to `Vec` for small data types.
- **When to Avoid**:
  - For large datasets with frequent iteration, `Vec` or `VecDeque` are faster due to better cache locality.
  - For random access or indexing, `Vec` or `VecDeque` are required.

**In Practice**:
- Rustâ€™s `LinkedList` is rarely used due to its performance drawbacks. `Vec` or `VecDeque` often suffice for most use cases, but `LinkedList` shines in niche scenarios requiring frequent middle insertions/removals.

### Why Convert `LinkedList` to `Vec` and Back?

The program converts `LinkedList` to `Vec` for shuffling and back to `LinkedList`:
```rust
let mut fruit: Vec<_> = fruit.into_iter().collect();
fruit.shuffle(&mut rng);
let mut fruit: LinkedList<_> = fruit.into_iter().collect();
```

**Why Convert?**:
- **Shuffling Requirement**: The `SliceRandom` traitâ€™s `shuffle` method is implemented for `Vec` and slices, which require contiguous memory for efficient random access. `LinkedList`â€™s non-contiguous structure doesnâ€™t support direct shuffling.
- **Conversion to `Vec`**:
  - `into_iter().collect()` consumes the `LinkedList` into an iterator and builds a `Vec`, enabling `shuffle`.
  - This step is necessary because `LinkedList` lacks indexing, making random swaps inefficient.
- **Shuffling**: `fruit.shuffle(&mut rng)` randomizes the `Vec`â€™s order using the `rand` crate.
- **Conversion Back to `LinkedList`**:
  - Restores the `LinkedList` structure to leverage its efficient insertion/removal capabilities for subsequent operations.
  - Ensures the program can continue using `LinkedList`â€™s features, like `push_front` or potential middle insertions.
- **Trade-Off**: Conversion is O(n) due to copying elements, but itâ€™s a one-time cost for shuffling.
- **Why Not `VecDeque`?**:
  - `VecDeque` also requires conversion to `Vec` for shuffling, as it lacks a direct `shuffle` method.
  - The choice of `LinkedList` may reflect a need for middle insertions/removals or pedagogical purposes to demonstrate its use.

**Note**: The code comments acknowledge that this conversion is uncommon in practice and included for consistency with the `VecDeque` example.

### Traits and Types from `rand`

The program uses the `rand` crate for shuffling:
1. **`rand::seq::SliceRandom`**:
   - **What is it?**: A trait providing methods for random operations on slices and `Vec`, such as `shuffle`.
   - **Key Method**:
     - `shuffle(&mut self, rng: &mut R)`: Randomly reorders elements using a random number generator.
   - **In the Code**:
     ```rust
     fruit.shuffle(&mut rng);
     ```
     - Shuffles the `Vec` of fruits.
   - **Why Use It?**: Efficient, safe randomization for contiguous collections.

2. **`rand::rng`**:
   - **What is it?**: A function returning a thread-local random number generator (`ThreadRng`).
   - **In the Code**:
     ```rust
     let mut rng = rng();
     ```
     - Creates `rng` for shuffling.
   - **Why Use It?**: Convenient, thread-safe randomness source.

**Adding `rand` to Your Project**:
Include in `Cargo.toml`:
```toml
[dependencies]
rand = "0.10"
```

---

## Building the Program Step-by-Step

Weâ€™ll build the fruit salad program incrementally, starting with basic `LinkedList` operations and progressing to advanced features.

### Step 1: Basic `LinkedList` Creation and Printing

**Goal**: Create a `LinkedList`, add fruits, and print them.

```rust
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

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
- **LinkedList Creation**:
  - `LinkedList::new()` creates an empty `LinkedList<&str>`.
  - `push_back` adds elements to the back, similar to `Vec::push`.
- **Printing**:
  - `fruit.iter().enumerate()` iterates over references to elements, pairing with indices.
  - Comma-separated formatting omits the comma for the last item.
- **Concepts**:
  - `LinkedList` initialization and back insertion.
  - Immutable iteration with `.iter()`.
  - No random access, relying on sequential traversal.

**Output**:
```
Fruit Salad:
Arbutus, Loquat, Strawberry Tree Berry
```

### Step 2: Shuffling with Conversion to `Vec`

**Goal**: Shuffle the fruits by converting `LinkedList` to `Vec`.

```rust
use rand::seq::SliceRandom;
use rand::rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

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
  - `fruit.shuffle(&mut rng)` randomizes the `Vec`â€™s order.
- **Conversion**:
  - `into_iter().collect()` converts `LinkedList` to `Vec` and back.
  - `Vec<_>` uses type inference for `Vec<&str>`.
- **Traits**:
  - `SliceRandom` for `shuffle`.
- **Ownership**:
  - `into_iter()` consumes the `LinkedList`, requiring reassignment.
  - `shuffle` modifies the `Vec` in place.

**Output** (example):
```
Fruit Salad:
Loquat, Arbutus, Strawberry Tree Berry
```

### Step 3: Double-Ended Operations

**Goal**: Add fruits to both ends, showcasing `LinkedList`â€™s flexibility.

```rust
use rand::seq::SliceRandom;
use rand::rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to both ends
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

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
- **Double-Ended Operations**:
  - `push_front` and `push_back` add elements efficiently (O(1)).
  - Demonstrates `LinkedList`â€™s ability to modify both ends, similar to `VecDeque` but with potential for middle operations.
- **Purpose**:
  - Adding `"Pomegranate"` (front) and `"Fig"`, `"Cherry"` (back) shows flexibility.
- **Comparison**:
  - Unlike `Vec`, which is slow for front insertions, `LinkedList` handles both ends efficiently.
  - Like `VecDeque`, but `LinkedList` supports middle insertions better (with a cursor).

**Output** (example):
```
Fruit Salad:
Pomegranate, Loquat, Arbutus, Strawberry Tree Berry, Fig, Cherry
```

### Step 4: Advanced Features (Error Handling, Middle Insertions)

**Goal**: Add error handling, middle insertions, and random selection to leverage `LinkedList`â€™s strengths.

```rust
use rand::seq::SliceRandom;
use rand::rng;
use std::collections::LinkedList;

fn main() -> Result<(), String> {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to both ends
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    print_fruit_salad(&fruit)?;

    // Insert a fruit in the middle
    let mut cursor = fruit.cursor_front_mut();
    for _ in 0..(fruit.len() / 2) {
        cursor.move_next();
    }
    cursor.insert_after("Mango");

    println!("After inserting Mango in the middle:");
    print_fruit_salad(&fruit)?;

    // Randomly select a fruit (via Vec conversion for simplicity)
    let random_fruit = {
        let fruit_vec: Vec<_> = fruit.iter().collect();
        fruit_vec.choose(&mut rng).ok_or("List is empty")?
    };
    println!("Random fruit: {}", random_fruit);

    // Remove from both ends
    let last_item = fruit.pop_back().ok_or("Cannot pop from empty list")?;
    println!("Last item removed: {}", last_item);
    let first_item = fruit.pop_front().ok_or("Cannot pop from empty list")?;
    println!("First item removed: {}", first_item);

    println!("Final fruit salad:");
    print_fruit_salad(&fruit)?;

    Ok(())
}

fn print_fruit_salad(fruit: &LinkedList<&str>) -> Result<(), String> {
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
  - `print_fruit_salad` returns `Result` to handle empty lists.
  - `pop_back` and `pop_front` use `.ok_or()` to convert `None` to errors.
- **Middle Insertion**:
  - Uses `cursor_front_mut()` to create a mutable cursor starting at the front.
  - `move_next()` advances to the middle (approximately `len/2` steps).
  - `insert_after` inserts `"Mango"` after the cursorâ€™s position (O(1)).
  - Demonstrates `LinkedList`â€™s strength for middle insertions, unlike `Vec` or `VecDeque`.
- **Random Selection**:
  - Converts to `Vec` temporarily for `SliceRandom::choose`, as `LinkedList` lacks direct random access.
  - `fruit_vec.choose(&mut rng)` picks a random element.
- **Cursor API**:
  - `LinkedList`â€™s cursor API allows precise navigation and modification, ideal for complex list manipulations.
- **Robustness**:
  - Handles empty lists to prevent panics.
  - `?` operator propagates errors cleanly.

**Output** (example):
```
Fruit Salad:
Pomegranate, Loquat, Arbutus, Strawberry Tree Berry, Fig, Cherry
After inserting Mango in the middle:
Fruit Salad:
Pomegranate, Loquat, Arbutus, Mango, Strawberry Tree Berry, Fig, Cherry
Random fruit: Arbutus
Last item removed: Cherry
First item removed: Pomegranate
Final fruit salad:
Fruit Salad:
Loquat, Arbutus, Mango, Strawberry Tree Berry, Fig
```

**Error Case** (empty list):
```rust
let fruit: LinkedList<&str> = LinkedList::new();
print_fruit_salad(&fruit)?;
```
Output:
```
Error: Fruit salad is empty
```

---

## Additional Challenges

To further explore `LinkedList`, randomization, and Rust collections, try these challenges:

1. **Custom Shuffle for `LinkedList`**:
   Implement a shuffle function for `LinkedList` without converting to `Vec`.
   **Hint**: Use a cursor to swap random nodes.

2. **Middle Removal**:
   Remove an element from the middle of the `LinkedList` using a cursor.
   **Hint**: Use `cursor_front_mut` and `remove_current`.

3. **Filter Fruits**:
   Remove fruits with names longer than a certain length (e.g., 7 characters).
   **Hint**: Use `cursor_front_mut` and `remove_current` while traversing.

4. **Interactive List**:
   Allow users to add or remove fruits interactively via the command line, specifying positions (front, back, or middle).
   **Hint**: Use `std::io` and the cursor API.

5. **Merge Lists**:
   Create two `LinkedList`s and merge them into one, preserving order or sorting by name.
   **Hint**: Use `append` or iterate with cursors.

6. **Serialize List**:
   Save the `LinkedList` to a JSON file and load it later.
   **Hint**: Use `serde` and `serde_json`.

---

## Running the Program

For any step:
1. Create a new Rust project:
   ```bash
   cargo new fruit_salad_linkedlist
   cd fruit_salad_linkedlist
   ```
2. Update `Cargo.toml`:
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

This tutorial built a fruit salad program using `LinkedList`, from basic list operations to advanced features like middle insertions and error handling. We covered:
- **LinkedList**: A doubly-linked list with efficient insertions/removals at any position.
- **Comparison**: Differences from `Vec` (random access, cache-friendly) and `VecDeque` (double-ended, contiguous).
- **Conversions**: Converting to `Vec` for shuffling and back to `LinkedList` for list operations.
- **Use Cases**: Scenarios where `LinkedList` excels (e.g., middle insertions) and its cache locality drawbacks.
- **Randomization**: Using `rand`â€™s `SliceRandom` for shuffling.
- **Advanced Features**: Cursor-based middle insertions and robust error handling.
- **Error Handling**: Robust checks for empty lists and invalid indices.
- **Interactive Input**: Using `std::io` for user interaction.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

