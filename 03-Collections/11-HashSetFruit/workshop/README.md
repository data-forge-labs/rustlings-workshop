# Workshop: HashSetFruit — Unique Items / Set Operations

**Goal**: Implement all functions in `src/lib.rs` to pass all **4** tests.

## Functions to Implement

### `generate_fruit`
- **Signature**: `pub fn generate_fruit() -> &'static str`
- **Task**: Return a random fruit from a predefined list
- **Tests**: step_01_generate (1 test)

### `collect_unique_fruits`
- **Signature**: `pub fn collect_unique_fruits(count: usize) -> (HashSet<&'static str>, HashMap<&'static str, u32>)`
- **Task**: Collect `count` random fruits into a HashSet (uniques) and HashMap (counts)
- **Tests**: step_02_hashset (test_collect_unique_fruits)

### (Set operations)
- **Task**: Demonstrate HashSet insert, contains, no-duplicates
- **Tests**: step_02_hashset (test_hashset_no_duplicates, test_hashset_insert)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_generate | 1 | Random fruit generation |
| step_02_hashset | 3 | HashSet uniqueness, insert, contains |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

