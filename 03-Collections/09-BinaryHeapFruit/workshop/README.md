# Workshop: BinaryHeapFruit — Priority Queue with Custom Ord

**Goal**: Implement all functions in `src/lib.rs` to pass all **5** tests.

## Functions to Implement

### `Fruit` enum with `Ord`
- **Variants**: `Fig`, `Other(String)`
- **Task**: Implement `Ord` so `Fig` sorts before all `Other` variants; `Other` variants are equal
- **Tests**: step_01_fruit_ord (3 tests: fig > other, fig == fig, other == other)

### `generate_fruit_salad`
- **Signature**: `pub fn generate_fruit_salad() -> BinaryHeap<Fruit>`
- **Task**: Return a BinaryHeap containing at least one `Fig` and some `Other` fruits
- **Tests**: step_02_generate (2 tests: contains_figs, non_empty)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_fruit_ord | 3 | Custom Ord for enum (Fig priority) |
| step_02_generate | 2 | BinaryHeap creation and contents |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

