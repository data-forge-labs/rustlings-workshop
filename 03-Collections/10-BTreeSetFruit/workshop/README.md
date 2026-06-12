# Workshop: BTreeSetFruit — Ordered Set

**Goal**: Implement all functions in `src/lib.rs` to pass all **4** tests.

## Functions to Implement

### `generate_fruit_set`
- **Signature**: `pub fn generate_fruit_set(fruits: &[&str], amount: usize, rng: &mut impl rand::Rng) -> (BTreeSet<&str>, HashMap<&str, u32>)`
- **Task**: Select `amount` random fruits, return a `BTreeSet` (unique) and a `HashMap` (counts)
- **Tests**: step_01_btreeset (test_generate_set_unique, test_generate_set_no_duplicates)

### `format_set_sorted`
- **Signature**: `pub fn format_set_sorted(set: &BTreeSet<&str>) -> Vec<&str>`
- **Task**: Return all elements in ascending order
- **Tests**: step_01_btreeset (test_format_set_sorted)

### `format_set_reverse`
- **Signature**: `pub fn format_set_reverse(set: &BTreeSet<&str>) -> Vec<&str>`
- **Task**: Return all elements in descending order
- **Tests**: step_01_btreeset (test_format_set_reverse)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_btreeset | 4 | BTreeSet creation, uniqueness, ordering |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

