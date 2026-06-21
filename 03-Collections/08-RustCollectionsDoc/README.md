# 🦀 RustCollectionsDoc — Rust Reference

*A side-by-side comparison of every standard collection type in Rust, with fruit-salad examples for each.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 8 tests pass**.

---

## What Is This Reference?

A side-by-side comparison of every standard collection type in Rust, with fruit-salad examples for each.

### Python equivalent

```python
# Python has list, dict, set, deque, OrderedDict — different APIs each
fruits = ["Orange", "Apple", "Banana"]
fruit_set = set(fruits)
fruit_counts = {"Orange": 1, "Apple": 2}
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Constant array | Compile-time fixed, stack-allocated |
| 2 | `Vec<T>` | Growable, contiguous, type-safe |
| 3 | `Vec<String>` | Mutable, owned growable text |
| 4 | Slices `&[T]` | Borrowed view of any contiguous sequence |
| 5 | `HashMap` | Hash-based key-value lookup |
| 6 | `BinaryHeap` | Priority queue (max-heap) |
| 7 | Sorting | Comparator-based sort |

---

Here are several ways to define a collection of fruits in Rust, mirroring the provided list:

#### 1. **Using a Constant Array (`[&str; 10]`)**

This is the closest match to a fixed-size list of string literals, commonly used when the size is known and wonâ€™t change.

```rust
const FRUITS: [&str; 10] = [
    "Orange",
    "Apple",
    "Banana",
    "Pear",
    "Grape",
    "Watermelon",
    "Strawberry",
    "Cherry",
    "Plum",
    "Peach",
];
```

- **Explanation**:
  - `const FRUITS`: Declares a compile-time constant.
  - `[&str; 10]`: An array of 10 string slices (`&str`).
  - String literals have a `'static` lifetime, so the type is technically `[&'static str; 10]`, but `'static` is often omitted for brevity.
- **Use Case**: Ideal for fixed, unchanging lists with compile-time size.

#### 2. **Using a `Vec<&str>`**

If you need a dynamic, resizable list, use a `Vec`:

```rust
let FRUITS: Vec<&str> = vec![
    "Orange",
    "Apple",
    "Banana",
    "Pear",
    "Grape",
    "Watermelon",
    "Strawberry",
    "Cherry",
    "Plum",
    "Peach",
];
```

- **Explanation**:
  - `let FRUITS`: Declares a variable (mutable with `mut` if needed, e.g., `let mut FRUITS`).
  - `Vec<&str>`: A dynamic vector of string slices.
  - `vec!` macro initializes the vector.
- **Use Case**: Suitable for lists that may grow, shrink, or be modified at runtime.

#### 3. **Using a `Vec<String>`**

If you need owned strings (e.g., for modification or ownership), use `String`:

```rust
let FRUITS: Vec<String> = vec![
    "Orange".to_string(),
    "Apple".to_string(),
    "Banana".to_string(),
    "Pear".to_string(),
    "Grape".to_string(),
    "Watermelon".to_string(),
    "Strawberry".to_string(),
    "Cherry".to_string(),
    "Plum".to_string(),
    "Peach".to_string(),
];
```

- **Explanation**:
  - `Vec<String>`: A vector of owned strings.
  - `.to_string()` converts `&str` literals to `String`.
  - Alternatively, use `String::from("Orange")` or the `vec!` macro with `into` (e.g., `vec!["Orange".into(), ...]`).
- **Use Case**: When strings need to be owned or modified (e.g., appending characters).

#### 4. **Using `VecDeque<&str>` or `LinkedList<&str>`**

For specialized use cases (double-ended queue or linked list):

```rust
use std::collections::VecDeque;

let FRUITS: VecDeque<&str> = VecDeque::from([
    "Orange",
    "Apple",
    "Banana",
    "Pear",
    "Grape",
    "Watermelon",
    "Strawberry",
    "Cherry",
    "Plum",
    "Peach",
]);
```

```rust
use std::collections::LinkedList;

let FRUITS: LinkedList<&str> = LinkedList::from([
    "Orange",
    "Apple",
    "Banana",
    "Pear",
    "Grape",
    "Watermelon",
    "Strawberry",
    "Cherry",
    "Plum",
    "Peach",
]);
```

- **Explanation**:
  - `VecDeque` and `LinkedList` are less common but valid for specific needs (e.g., double-ended operations or frequent middle insertions).
  - `from` converts an array or slice to the collection.
- **Use Case**: Rare for simple lists; used in previous tutorials for queue or list operations.

---

### Choosing the Right Collection

Based on the context of the previous tutorials (e.g., fruit salad programs), the array `[&str; 10]` is likely the intended structure, as it matches the fixed-size, constant nature of the fruit list in earlier examples (e.g., `const FRUITS: [&str; 10]`). However, if the list needs to be dynamic or support operations like shuffling, a `Vec<&str>` is more practical, as arrays donâ€™t support resizing or direct shuffling.

Hereâ€™s a quick decision guide:
- **Use an Array**: If the list is fixed, known at compile time, and wonâ€™t change (e.g., a constant list of 10 fruits).
- **Use a `Vec`**: If the list needs to grow, shrink, or be shuffled/modified.
- **Use `VecDeque` or `LinkedList`**: For specific queue or list operations (less likely here).

---

### Example in Context

To align with the previous fruit salad tutorials, letâ€™s assume the intent is to define a constant array similar to the vector example. Hereâ€™s how it would look in a program:

```rust
use rand::seq::SliceRandom;
use rand::rng;

const FRUITS: [&str; 10] = [
    "Orange",
    "Apple",
    "Banana",
    "Pear",
    "Grape",
    "Watermelon",
    "Strawberry",
    "Cherry",
    "Plum",
    "Peach",
];

fn main() {
    let mut rng = rng();
    let mut fruit_vec: Vec<&str> = FRUITS.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    println!("Fruit Salad:");
    for (i, item) in fruit_vec.iter().enumerate() {
        if i != fruit_vec.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
```

- **Explanation**:
  - `FRUITS` is a constant array, matching the provided list.
  - Converted to `Vec` for shuffling, as arrays donâ€™t support direct shuffling.
  - Prints the shuffled fruits, similar to previous tutorials.

**Output** (example):
```
Fruit Salad:
Peach, Apple, Cherry, Watermelon, Grape, Banana, Plum, Strawberry, Orange, Pear
```

---

### Addressing the Original Syntax

To make the Python-like syntax valid in Rust, youâ€™d rewrite:
```python
FRUITS: List[str] = ["Orange", "Apple", "Banana", "Pear", "Grape", "Watermelon", "Strawberry", "Cherry", "Plum", "Peach"]
```

As one of the Rust equivalents above, with the array being the most direct translation for a fixed list:

```rust
const FRUITS: [&str; 10] = ["Orange", "Apple", "Banana", "Pear", "Grape", "Watermelon", "Strawberry", "Cherry", "Plum", "Peach"];
```

If you intended a different collection (e.g., `Vec`), please clarify, and I can tailor the example further!

---

### Conclusion

The syntax `FRUITS: List[str] = [...]` is invalid in Rust due to its Python-style type hint and non-existent `List` type. Rust uses arrays (`[&str; N]`), `Vec<&str>`, or other collections for lists. The most appropriate translation for a fixed list of 10 fruits is a constant array (`[&str; 10]`), but `Vec<&str>` is better for dynamic operations like shuffling. The provided example shows how to integrate this into a fruit salad program, consistent with previous tutorials.

### Why the Syntax: `FRUITS: List[str] = [...]` is Invalid in Rust

1. **`List[str]` Type**:
   - Rust does not have a `List` type in its standard library, nor does it use Python-style type hints like `[str]`.
   - The equivalent Rust types for a collection of strings are:
     - Array: `[&str; N]` (fixed-size, compile-time length).
     - Vector: `Vec<&str>` or `Vec<String>` (dynamic, resizable).
     - Other collections: `VecDeque<&str>` or `LinkedList<&str>` (specialized use cases).
   - Strings in Rust are either `&str` (string slices, typically for literals) or `String` (owned, heap-allocated strings).

2. **Variable Declaration**:
   - In Python, `FRUITS: List[str] = [...]` declares a variable with a type hint.
   - In Rust, variables are declared with `let`, `const`, or `static`, and types are often inferred or explicitly annotated using `:`.
   - For a constant array of string literals, `const` is appropriate, and the type would be `[&str; N]`.

3. **String Literals**:
   - The literals `"Orange"`, `"Apple"`, etc., are valid in Rust as `&str` (string slices with a `'static` lifetime).
   - However, the collection syntax must match Rustâ€™s collection types.

---

## Related Projects

For guided learning of each collection type, see:

- [01-TicketManagement](../../01-TicketManagement/README.md) — `Vec`, `HashMap`, `BTreeMap`, iterators, lifetimes (canonical collections teaching)
- [02-VectorFruitSalad](../../02-VectorFruitSalad/README.md) — `Vec<T>`, `SliceRandom`, `rand` integration
- [03-ArrayFruitSalad](../../03-ArrayFruitSalad/README.md) — `[T; N]` fixed-size arrays
- [04-HashMapCount](../../04-HashMapCount/README.md) — `HashMap`, `.entry().or_insert()` upsert, `BTreeMap` for sorting
- [05-LinkedListFruitSalad](../../05-LinkedListFruitSalad/README.md) — `LinkedList`, when (not) to use it
- [06-VecDequeFruitSalad](../../06-VecDequeFruitSalad/README.md) — `VecDeque`, ring buffer
- [07-HashMapLanguage](../../07-HashMapLanguage/README.md) — `HashMap` with complex values
- [09-BinaryHeapFruit](../../09-BinaryHeapFruit/README.md) — priority queue
- [10-BTreeSetFruit](../10-BTreeSetFruit/README.md) — ordered set
- [11-HashSetFruit](../11-HashSetFruit/README.md) — unique items, membership testing
- [12-RustIterators](../12-RustIterators/README.md) — lazy functional iteration
- [13-MutableFruitSalad](../13-MutableFruitSalad/README.md) — `Vec` mutation patterns (push, pop, insert, remove, capacity)

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

