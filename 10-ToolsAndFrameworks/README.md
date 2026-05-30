# Section 10: Tools and Frameworks

This section introduces essential Rust tools and frameworks that data engineers use daily in production systems. Each project mirrors a tool you already know from Python, so you can transfer knowledge while learning idiomatic Rust patterns.

## Projects

| # | Project | Python Equivalent | What You'll Learn |
|---|---------|-------------------|-------------------|
| 1 | **Logging** — structured logging with multiple backends | loguru, structlog | `log` crate facade, `env_logger`, `tracing` spans, log levels, structured output |
| 2 | **Configuration** — multi-format config parsing | configparser, pydantic, pyyaml | `config` crate, TOML/JSON/YAML parsing, `serde` derive, env override merging |
| 3 | **Testing** — comprehensive test patterns | pytest, hypothesis | `#[test]`, `#[should_panic]`, `Result<T,E>` in tests, property-based patterns, integration tests |

## Learning Path

1. Start with **01-Logging** — it's the simplest and most immediately useful
2. Move to **02-Configuration** — config management is essential for CLI tools and services
3. Finish with **03-Testing** — build confidence in the testing patterns used across the entire course

All three projects follow the **test-driven architecture**: functions start as `todo!()` stubs, and you replace them while running `cargo test` to see progress.
