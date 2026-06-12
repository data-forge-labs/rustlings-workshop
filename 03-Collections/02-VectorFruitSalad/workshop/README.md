# Workshop: VectorFruitSalad — Vec<T> Dynamic Arrays

**Goal**: Implement all functions in `src/lib.rs` to pass all **4** tests.

## Functions to Implement

### `select_random_fruits`
- **Signature**: `pub fn select_random_fruits<'a>(fruit_count: usize, fruits: &[&'a str], rng: &mut impl rand::Rng) -> Vec<&'a str>`
- **Task**: Randomly select `fruit_count` fruits from `fruits` slice using `rng`
- **Tests**: step_01_select (4 tests: zero, one, multiple, all)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_select | 4 | Random selection, empty edge case |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

