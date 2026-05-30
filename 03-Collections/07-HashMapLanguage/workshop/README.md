# Workshop: HashMapLanguage — Complex HashMap Data

**Goal**: Implement all functions in `src/lib.rs` to pass all **6** tests.

## Functions to Implement

### `languages`
- **Signature**: `pub fn languages() -> HashMap<String, u32>`
- **Task**: Return a HashMap of 15 programming language names → creation year
- **Tests**: step_01_languages (3 tests: contains Rust, len 15, Python year)

### `normalize`
- **Signature**: `pub fn normalize(languages: &mut HashMap<String, u32>) -> HashMap<String, u32>`
- **Task**: Normalize years to weight 1-100 (newest year → 100, oldest → 1)
- **Tests**: step_02_normalize (3 tests: count, range, Rust > C)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_languages | 3 | HashMap initialization, key lookup, len |
| step_02_normalize | 3 | Value transformation, range clamping |

## How to Run Tests
```bash
cargo test
```
