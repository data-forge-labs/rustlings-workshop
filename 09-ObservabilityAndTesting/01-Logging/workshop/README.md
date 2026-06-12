# Workshop: Logging

**Goal**: Implement all 5 functions in `src/lib.rs` to pass all 8 tests.

## Functions to Implement

### `demo_log_levels`
- **Signature**: `pub fn demo_log_levels() -> Vec<String>`
- **Task**: Log one message at each of the 5 levels (trace, debug, info, warn, error) using `log` crate macros; return the captured log messages as strings.
- **Tests**: test_demo_log_levels_returns_all_levels, test_demo_log_levels_contains_expected_strings

### `log_structured_data`
- **Signature**: `pub fn log_structured_data(key: &str, value: &str) -> String`
- **Task**: Initialize `env_logger`, log structured data (key/value pair), and return the formatted log string.
- **Tests**: test_structured_data_format, test_structured_data_empty_values

### `tracing_demo`
- **Signature**: `pub fn tracing_demo(depth: usize) -> Vec<String>`
- **Task**: Use `tracing` spans to track recursive calls to depth `depth` (e.g., `depth_0`, `depth_1`, ...). Return span names in order.
- **Tests**: test_tracing_demo_returns_spans_in_order, test_tracing_demo_depth_zero

### `logging_overhead`
- **Signature**: `pub fn logging_overhead(iterations: usize) -> (u64, u64)`
- **Task**: Benchmark — run a loop `iterations` times with logging and without; return (with_log_ns, without_log_ns).
- **Tests**: (no dedicated test; used for exploration)

### `loguru_equivalents`
- **Signature**: `pub fn loguru_equivalents() -> Vec<(&'static str, &'static str)>`
- **Task**: Return Rust logging concepts mapped to their Python loguru equivalents (e.g., `("log::info!", "logger.info()")`).
- **Tests**: test_loguru_equivalents_non_empty, test_loguru_equivalents_maps_logger

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_log_levels | 2 | All 5 log levels produce messages |
| step_02_structured_logging | 2 | Key/value structured logging |
| step_03_tracing | 2 | Tracing span depth tracking |
| step_04_comparison | 2 | Rust→Python loguru equivalents |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

