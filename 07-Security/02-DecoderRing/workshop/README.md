# Workshop: DecoderRing

**Goal**: Implement all 5 functions in `src/lib.rs` to pass all 7 tests.

## Functions to Implement

### `gen_counts`
- **Signature**: `pub fn gen_counts() -> HashMap<char, f32>`
- **Task**: Build a map of English letter frequencies (percentages, e.g., `'e' -> 12.7`).
- **Tests**: gen_counts_contains_expected_letters

### `decrypt`
- **Signature**: `pub fn decrypt(text: &str, shift: usize) -> String`
- **Task**: Apply a Caesar backward shift; preserve non-alphabetic characters; wrap around a→z.
- **Tests**: decrypt_basic_shift, decrypt_wrap_around, decrypt_empty_string, decrypt_non_alpha_preserved

### `score_text`
- **Signature**: `pub fn score_text(text: &str, freqs: &HashMap<char, f32>) -> f32`
- **Task**: Score how English-like the text is by comparing letter frequencies.
- **Tests**: english_text_scores_higher_than_random

### `guess_shift`
- **Signature**: `pub fn guess_shift(text: &str, depth: usize) -> (usize, String, f32)`
- **Task**: Try all shifts up to `depth` and return the best (shift, decrypted text, score).
- **Tests**: known_shift_returns_correct_shift

### `guess_shift_parallel`
- **Signature**: `pub fn guess_shift_parallel(text: &str, depth: usize) -> (usize, String, f32)`
- **Task**: Parallel version of `guess_shift` using Rayon's `into_par_iter`.
- **Tests**: (benchmarked via `cargo bench`)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_frequencies | 1 | Building English letter frequency map |
| step_02_decryption | 4 | Caesar shift decryption with wrapping |
| step_03_scoring | 1 | Frequency-based text scoring |
| step_04_guess | 1 | Automatic shift detection |

## How to Run Tests
```bash
cargo test
cargo bench    # performance comparison
```
