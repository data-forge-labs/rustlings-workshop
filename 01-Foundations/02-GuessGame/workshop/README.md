# Workshop: 02-GuessGame — Functions Reference

**Goal**: Implement all functions in `src/lib.rs` to pass all **20** tests.

## Functions to Implement

### `check_guess`
- **Signature**: `pub fn check_guess(secret: u32, guess: u32) -> GuessOutcome`
- **Task**: Compare `guess` to `secret`; return `Correct`, `TooHigh`, or `TooLow`
- **Tests**: test_correct, test_too_high, test_too_low, test_boundary_high, test_boundary_low
- **README §3**

### `hint_for`
- **Signature**: `pub fn hint_for(outcome: GuessOutcome) -> &'static str`
- **Task**: Map the outcome to a player-facing message (`"Correct!"`, `"Too high!"`, `"Too low!"`)
- **Tests**: test_hint_correct, test_hint_too_high, test_hint_too_low
- **README §3**

### `parse_guess`
- **Signature**: `pub fn parse_guess(input: &str) -> Result<u32, String>`
- **Task**: Trim the input and parse it as a `u32`; return `Err(message)` for invalid input
- **Tests**: test_parse_valid, test_parse_with_whitespace, test_parse_zero, test_parse_negative_out_of_range, test_parse_non_numeric, test_parse_empty_string
- **README §5**

### `play_round`
- **Signature**: `pub fn play_round(secret: u32, input: &str) -> Result<GuessOutcome, String>`
- **Task**: Combine `parse_guess` and `check_guess`; return the outcome or the parse error
- **Tests**: test_play_round_correct, test_play_round_too_low, test_play_round_too_high, test_play_round_bad_input
- **README §6**

### `generate_secret`
- **Signature**: `pub fn generate_secret(min: u32, max: u32) -> u32`
- **Task**: Return a random number in the inclusive range `[min, max]` using `rand::random_range`
- **Tests**: test_secret_in_range_small, test_secret_in_range_wide
- **README §7**

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_check | 5 | `check_guess` outcomes, boundaries |
| step_02_hint | 3 | `hint_for` message mapping |
| step_03_parse | 6 | `parse_guess` success and failure paths |
| step_04_play_round | 4 | `play_round` composition |
| step_05_secret | 2 | `generate_secret` stays in range |

## How to Run

```bash
cargo test        # run all 20 tests
cargo run         # play the interactive game
```

## Concepts Introduced Here

These are the **new** topics this workshop teaches (everything else was covered in [01-Intro](../../01-Intro/README.md)):

- **Adding an external crate** (`rand`) via `Cargo.toml`
- **Custom `enum`** with `derive(Debug, PartialEq, Eq)`
- **`String` vs `&str`** (owned `String` from `read_line`, borrowed `&str` literals)
- **`std::io::stdin().read_line(&mut buf)`** for console input
- **`io::stdout().flush()`** to make the prompt appear before input
- **`Result<T, E>`** and `parse()` returning `Ok` / `Err`
- **`.expect()`** for the simplest form of error handling
- **`?` operator** (basic, inside `play_round`)
- **`match` on `Result`** (basic, not exhaustive enum matching)
- **`match` on the custom `GuessOutcome` enum** (with `==` comparison)
- **`continue`** inside a loop
