# Workshop: MutableFruitSalad — Vec Mutation Patterns

**Goal**: Implement all functions in `src/lib.rs` to pass all **6** tests.

## Functions to Implement

### `add_fruit`
- **Signature**: `pub fn add_fruit<'a>(fruit_salad: &mut Vec<&'a str>, fruit: &'a str)`
- **Task**: Push a fruit to the end of the Vec
- **Tests**: test_add_fruit

### `remove_fruit`
- **Signature**: `pub fn remove_fruit(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) -> bool`
- **Task**: Remove first occurrence; return `true` if found, `false` otherwise
- **Tests**: test_remove_fruit_exists, test_remove_fruit_not_found

### `sort_fruits`
- **Signature**: `pub fn sort_fruits(fruit_salad: &mut Vec<&str>)`
- **Task**: Sort the Vec in-place
- **Tests**: test_sort_fruits

### `pick_random_fruit`
- **Signature**: `pub fn pick_random_fruit<'a>(fruit_salad: &[&'a str], rng: &mut impl rand::Rng) -> Option<&'a str>`
- **Task**: Pick a random element; return `None` if empty
- **Tests**: test_pick_random_fruit, test_pick_random_empty

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_mutability | 6 | Vec add, remove, sort, random pick |

## How to Run Tests
```bash
cargo test
```
