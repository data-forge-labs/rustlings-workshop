# 🦀 Async Clap CLI — Python to Rust Workshop

*Subtitle: Build async CLI tools with subcommands, derive-based parsing, and `tokio` integration.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

---

## Why Async CLIs in Rust?

**Python pain:** You write a CLI with `argparse` and `asyncio.run(main())`. The parser is imperative; you check each arg by hand. Subcommands mean nested `if __name__ == "__main__":` blocks. Validation happens at runtime, after the user has already typed 20 characters.

**Rust fix:** `clap`'s derive mode turns a struct into a full parser at compile time. The parser knows:
- Required vs optional fields
- Short and long flags
- Subcommand trees
- Help text generated from doc comments
- Type validation (e.g., `u8`, `PathBuf`)

Combined with `tokio` for async, you get a CLI that:
- Validates args at parse time (no `--parallelism 99999` accepted)
- Generates `--help` automatically
- Runs async I/O (DB queries, network calls) without blocking

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Derive parser | `#[derive(Parser)]` | `argparse.ArgumentParser` | Compile-time-validated args |
| 2 | Subcommands | `#[derive(Subcommand)]` enum | nested subparsers | Type-safe command tree |
| 3 | Global flags | `#[arg(global = true)]` | manual handling | `--log-level` available everywhere |
| 4 | Help text | `#[command(about = "...")]` | `description=` | Auto-generated `--help` |
| 5 | Type validation | `u8`, `PathBuf` | manual range check | Wrong types rejected at parse |
| 6 | Async runtime | `#[tokio::main]` | `asyncio.run` | tokio + async fn main |
| 7 | Exit codes | `ExitCode::from(n)` | `sys.exit(n)` | Standardized CLI behavior |
| 8 | JSON config | `serde_json::from_str` | `json.load` | Typed configuration loading |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The `Parser` and `Subcommand` Derives](#3-concept-the-parser-and-subcommand-derives)
4. [Concept: Global Flags and Inheritance](#4-concept-global-flags-and-inheritance)
5. [Concept: Async Main and Exit Codes](#5-concept-async-main-and-exit-codes)
6. [Concept: JSON Config Loading](#6-concept-json-config-loading)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

The `clap` crate is the de-facto standard for CLI parsing in Rust. It's used in:
- `cargo` itself
- `rustup`
- `ripgrep`
- `fd`
- `bat`
- `exa` / `eza`
- The `gh` CLI (GitHub's official tool)

**Python to Rust:** `argparse` works fine for simple CLIs, but it doesn't scale to nested subcommand trees. `click` is closer, but it's still imperative. Rust's `clap` derive mode is what you wish `argparse` looked like.

**Data-engineering motivation:** Every data tool has a CLI. `dbt run`, `airflow dags trigger`, `prefect deploy`, `dagster job launch` — all are subcommand-based CLIs. Knowing how to build one in Rust lets you build production-grade data tools.

## 2. Prerequisites

- Completed [06-CLIAndTools/01-CLISalad](../01-CLISalad/README.md) — comfortable with basic clap.
- Familiar with `async/await` from [05-Concurrency/02-Futures](../05-Concurrency/02-Futures/README.md).
- Comfortable with `Result` and `Box<dyn Error>`.

## 3. Concept: The `Parser` and `Subcommand` Derives

`clap` in derive mode is the modern, recommended way. You define your CLI as a struct, derive `Parser`, and add `#[arg(...)]` attributes to fields:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "etlctl", about = "Async ETL pipeline CLI")]
struct Cli {
    #[arg(long, global = true, default_value = "info")]
    log_level: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        config: String,

        #[arg(long, default_value_t = 1)]
        parallelism: u8,
    },
    Etl {
        #[command(subcommand)]
        action: EtlAction,
    },
}
```

The `#[command(subcommand)]` attribute marks a field that contains nested commands. Each variant of the enum becomes a subcommand. Field attributes (`#[arg(short, long)]`) make them available as `-c X` and `--config X`.

**In Python (`click`):**

```python
import click

@click.group()
@click.option("--log-level", default="info")
def cli(log_level):
    pass

@cli.command()
@click.option("--config", "-c", required=True)
@click.option("--parallelism", default=1, type=int)
def run(config, parallelism):
    ...
```

The Python version is more verbose and the types are checked at runtime. The Rust version's `u8` field will reject `--parallelism abc` or `--parallelism 999` at parse time.

## 4. Concept: Global Flags and Inheritance

The `#[arg(global = true)]` attribute makes a flag available on the main command AND all subcommands. This is the pattern for shared flags like `--log-level` or `--config`:

```rust
struct Cli {
    #[arg(long, global = true, default_value = "info")]
    log_level: String,
    ...
}
```

Now both of these work:
- `etlctl --log-level debug run -c p.json`
- `etlctl run --log-level debug -c p.json`

**In Python:** you have to manually add the option to every subcommand and propagate it via context. `clap` does this for free.

## 5. Concept: Async Main and Exit Codes

To use `tokio`'s async runtime, annotate `main` with `#[tokio::main]`:

```rust
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = match Cli::try_parse_from(&argv) {
        Ok(c) => c,
        Err(e) => {
            e.print().unwrap();
            return ExitCode::from(2);
        }
    };

    match run_pipeline(&cli).await {
        Ok(msg) => { println!("{}", msg); ExitCode::SUCCESS }
        Err(e) => { eprintln!("Error: {}", e); ExitCode::FAILURE }
    }
}
```

`ExitCode` is the standard return type for process exit codes:
- `0` = success
- `1` = general failure
- `2` = usage / parse error

**In Python:**

```python
import sys
sys.exit(0)  # or 1, 2, etc.
```

Same idea, different type. The `ExitCode` enum prevents the typo of `return 1;` when you mean `return 0;`.

## 6. Concept: JSON Config Loading

Long-running pipelines usually have a JSON config file with the source, target, and parallelism. The CLI is just the entry point — the actual config is in the file:

```json
{
  "name": "orders",
  "source": "s3://bucket/orders.csv",
  "target": "warehouse",
  "parallelism": 4
}
```

```rust
#[derive(Deserialize)]
struct PipelineConfig {
    name: String,
    source: String,
    target: String,
    parallelism: u8,
}

let json = std::fs::read_to_string("pipe.json")?;
let cfg: PipelineConfig = serde_json::from_str(&json)?;
```

`u8` for `parallelism` is checked at deserialize time — a value of `9999` is rejected with a clear error message.

**In Python:** `pydantic.BaseModel` does the same, but at runtime, after `PipelineConfig(...)` is called. Rust's `serde` does it during `from_str`, which is closer to the boundary.

## 7. Putting It All Together

`lib.rs` is organized in four progressive steps:

1. **Step 1 (`step_01_parse`)** — `parse_args` for all subcommands + global flag.
2. **Step 2 (`step_02_config`)** — JSON config deserialization.
3. **Step 3 (`step_03_helpers`)** — `extract_target`, `run_summary`.
4. **Step 4 (`step_04_async`)** — `fake_io_work` with `tokio::time::sleep`, `run_pipeline` matching all actions.

`main.rs` uses `#[tokio::main]`, converts `Vec<String>` from `std::env::args` to `Vec<&str>` for `parse_args`, and returns `ExitCode`.

## 8. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs).

## 9. Summary

| Concept | Used In |
|---------|---------|
| `#[derive(Parser)]` | `Cli` |
| `#[derive(Subcommand)]` | `Commands`, `EtlAction` |
| `#[arg(global = true)]` | `log_level` |
| `#[arg(short, long, default_value_t = ...)]` | `parallelism`, `config`, etc. |
| `Cli::try_parse_from` | `parse_args` |
| `serde_json::from_str` | `parse_pipeline_config` |
| `#[tokio::main]` | `main.rs` |
| `tokio::time::sleep` | `fake_io_work` |
| `ExitCode::SUCCESS` / `FAILURE` | `main.rs` |

## Further Reading

- [clap documentation](https://docs.rs/clap/)
- [clap derive tutorial](https://docs.rs/clap/latest/clap/_derive/index.html)
- [tokio documentation](https://tokio.rs/)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/) (free online book)

## Exercises

1. **Easy**: Add `--dry-run` flag to the `Run` subcommand (boolean, default false), and 1 test.
2. **Medium**: Add a `validate` subcommand that loads a config and returns an error if `parallelism > 16`. Add 1 test.
3. **Hard**: Add a `parallel_run` function that takes a `Run { parallelism, .. }` command, spawns `parallelism` tokio tasks each calling `fake_io_work(50)`, awaits all of them with `join_all`, and returns the total time. Add 1 test that asserts the result contains "spawned".
