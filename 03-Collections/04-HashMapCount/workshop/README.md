# Workshop: HashMapCount — Word Frequency Counting

**Goal**: Implement all functions in `src/lib.rs` to pass all **7** tests.

## Functions to Implement

### `count_frequencies`
- **Signature**: `pub fn count_frequencies(numbers: Vec<i32>) -> HashMap<i32, u32>`
- **Task**: Count occurrences of each number and return a frequency map
- **Tests**: step_01_frequencies (4 tests: empty, single, multiple, all_same)

### `most_frequent`
- **Signature**: `pub fn most_frequent(numbers: &[i32]) -> Option<(i32, u32)>`
- **Task**: Find the most frequent element and its count; return `None` if empty
- **Tests**: step_02_most_frequent (3 tests: basic, empty, tie)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_frequencies | 4 | HashMap insertion, counting |
| step_02_most_frequent | 3 | Most frequent lookup, empty, ties |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

