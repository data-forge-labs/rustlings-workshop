# Cargo Cheatsheet

Every Rust project uses `cargo` for building, testing, and managing dependencies. Here's every command you'll need in this course.

## Project Lifecycle

| Command | What It Does | Python Equivalent |
|---------|-------------|-------------------|
| `cargo new my_project` | Create new Rust project | `mkdir my_project && cd my_project && pip init` |
| `cargo init` | Init cargo in existing dir | — |
| `cargo build` | Compile (debug) | `python -m py_compile` |
| `cargo build --release` | Compile (optimized) | — |
| `cargo run` | Build + run | `python main.py` |
| `cargo check` | Check compilation without producing binary | `mypy main.py` (type-check only) |

## Dependencies

| Command | What It Does | Python Equivalent |
|---------|-------------|-------------------|
| `cargo add rand` | Add a dependency | `pip install rand` + add to `pyproject.toml` |
| `cargo add clap --features derive` | Add with features | `pip install clap[derive]` |
| `cargo rm rand` | Remove a dependency | `pip uninstall rand` |
| `cargo update` | Update all deps to latest semver | `pip install --upgrade` |
| `cargo tree` | Show dependency tree | `pip freeze` / `pipdeptree` |

## Testing

| Command | What It Does | Python Equivalent |
|---------|-------------|-------------------|
| `cargo test` | Run all tests | `pytest` |
| `cargo test foo` | Run tests with "foo" in name | `pytest -k foo` |
| `cargo test -- --nocapture` | Show stdout for passing tests | `pytest -s` |
| `cargo test -- --ignored` | Run ignored tests only | `pytest -m ignored` |
| `cargo test --test integration` | Run integration tests | `pytest tests/` |

## Code Quality

| Command | What It Does | Python Equivalent |
|---------|-------------|-------------------|
| `cargo fmt` | Auto-format code | `black` |
| `cargo fmt --check` | Check formatting (CI) | `black --check` |
| `cargo clippy` | Lint code (150+ rules) | `ruff` |
| `cargo doc --open` | Build + open docs in browser | `pdoc` / `sphinx-build` |
| `cargo audit` | Check for security advisories | `pip-audit` |

## Project Structure

```
my_project/
├── Cargo.toml       # Package manifest (dependencies + metadata)
├── Cargo.lock       # Lockfile — pin exact versions (commit this!)
└── src/
    ├── main.rs      # Binary entry point
    └── lib.rs       # Library entry point (for shared code)
```

### `Cargo.toml` Breakdown

```toml
[package]
name = "my_project"       # Package name
version = "0.1.0"         # Semantic version
edition = "2021"          # Rust edition (2021 is current)

[dependencies]
rand = "0.8"              # Crate name + version requirement
serde = { version = "1", features = ["derive"] }  # With features

[dev-dependencies]        # Only for tests/benchmarks
criterion = "0.5"

[profile.release]          # Release build optimization
opt-level = 3              # Max optimization
```

## Common Workflows

```bash
# Start a new project
cargo new my_pipeline
cd my_pipeline

# Add dependencies
cargo add csv
cargo add serde --features derive

# Code, then check
cargo check          # Fast feedback
cargo test           # Run tests
cargo clippy         # Lint
cargo fmt            # Format

# Build for production
cargo build --release
./target/release/my_pipeline
```

## FAQ

**Q: `cargo build` is slow. What do I do?**
A: Normal. First build downloads and compiles all deps. Subsequent builds are incremental (only changed files). Use `cargo check` for faster feedback during development.

**Q: Where's the binary after `cargo build`?**
A: `./target/debug/my_project` (debug) or `./target/release/my_project` (release).

**Q: How do I run a specific test?**
A: `cargo test test_name` runs any test whose name contains `test_name`.

**Q: What's `Cargo.lock`? Should I commit it?**
A: Yes, commit `Cargo.lock` for application projects (not libraries). It pins exact dependency versions so everyone builds the same thing.
