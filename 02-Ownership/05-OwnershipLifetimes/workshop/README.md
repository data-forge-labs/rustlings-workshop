# Workshop: OwnershipLifetimes — Lifetimes, Move, Copy

**Goal**: Implement all functions in `src/lib.rs` to pass all **16** tests.

## Functions to Implement

### `longest`
- **Signature**: `pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
- **Task**: Return the longer of two string slices; return first if equal
- **Tests**: step_01_lifetime_functions (3 tests)

### `first`
- **Signature**: `pub fn first<'a>(items: &'a [i32]) -> &'a i32`
- **Task**: Return reference to first element; panic on empty slice
- **Tests**: step_01_lifetime_functions (3 tests)

### `Bookmark` struct
- **Signature**: `pub struct Bookmark<'a> { pub title: &'a str, pub url: &'a str }`
- **Methods**: `new(title, url)`, `display() -> String` (format `"<title> - <url>"`)
- **Tests**: step_02_struct_lifetimes (3 tests)

### `move_demo`
- **Signature**: `pub fn move_demo(s: String) -> String`
- **Task**: Take ownership of a String and return it (possibly modified)
- **Tests**: step_03_move_vs_copy (2 tests)

### `copy_demo`
- **Signature**: `pub fn copy_demo(x: i32) -> i32`
- **Task**: Accept i32 (Copy), return it (caller still owns original)
- **Tests**: step_03_move_vs_copy (2 tests)

### `lifetime_concepts`
- **Signature**: `pub fn lifetime_concepts() -> Vec<&'static str>`
- **Task**: Return list of ownership/lifetime concepts covered
- **Tests**: step_04_concepts (3 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_lifetime_functions | 6 | Lifetime annotations, elision |
| step_02_struct_lifetimes | 3 | Struct with lifetime parameters |
| step_03_move_vs_copy | 4 | Move semantics, Copy types |
| step_04_concepts | 3 | Concept listing verification |

## How to Run Tests
```bash
cargo test
```
