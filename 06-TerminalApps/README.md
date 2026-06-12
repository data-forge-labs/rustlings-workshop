# Section 6: Terminal Apps — CLIs, TUI Dashboards, Async Subcommands

*Building production-grade terminal applications in Rust: command-line tools, full-screen TUI dashboards, and async CLI front-ends for ETL pipelines.*

---

## Why This Section?

### The Problem — Python Terminal Apps Don't Scale

Python's terminal ecosystem (`argparse`, `click`, `prompt_toolkit`) works for small scripts, but it shows its limits the moment you need:

- **Sub-millisecond startup** for CLI tools called from cron / Airflow / Make (Python adds 100-300ms of import tax per invocation)
- **Single-binary distribution** (no `pip install` hell, no virtualenv, no Python version conflicts)
- **Full-screen TUI dashboards** for monitoring long-running pipelines (Python TUIs lag, reflow, and crash on resize)
- **Async CLI front-ends** that fan out to thousands of concurrent network calls

```python
# python — slow startup, single binary impossible, no async CLI
import argparse
import time
t0 = time.time()
import pandas as pd      # 200ms
import requests          # 100ms
parser = argparse.ArgumentParser()  # 30ms
print(f"Import tax: {time.time() - t0:.2f}s")  # ~0.33s
```

### The Rust Solution — Instant, Single-Binary Terminal Apps

```rust
// Rust — 5ms cold start, one binary, async-friendly
use clap::Parser;

#[derive(Parser)]
#[command(name = "etl-tool")]
struct Cli {
    #[arg(short, long)] input: String,
    #[arg(short, long, default_value_t = 8)] workers: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    // 5ms startup + async fan-out + native binary
    Ok(())
}
```

A Rust CLI starts in **single-digit milliseconds** and ships as a single static binary that runs anywhere — no runtime, no dependencies, no `pyproject.toml` to maintain.

---

## What You'll Learn

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | CLI argument parsing | `clap` derive API | `argparse`, `click` | Typed, compile-time-validated CLI args |
| 2 | Builder API | `clap::Command` | `argparse` builders | Define args programmatically |
| 3 | `lib.rs` + `main.rs` | library/CLI split | module/entry-point | Test CLI logic without spawning a process |
| 4 | Library re-exports | `pub use` | `__init__.py` | Control the public API surface |
| 5 | Modules & visibility | `mod`, `pub` | `import` | Encapsulate CLI internals |
| 6 | Terminal UI | `ratatui`, `crossterm` | `prompt_toolkit`, `urwid` | Full-screen dashboards for monitoring |
| 7 | Immediate-mode UI | widget tree per frame | retained-mode GUI | Recompute on every event; no stale state |
| 8 | Event loop | `loop { event::read()? }` | `getch()` loop | Non-blocking input |
| 9 | Async CLI | `#[tokio::main]`, `clap` subcommands | `asyncio.run()` | Fan-out to many concurrent tasks |
| 10 | Exit codes | `ExitCode`, `process::ExitCode` | `sys.exit(N)` | Communicate success/failure to shell |
| 11 | JSON config | `serde_json` for config | `json` module | Load pipeline config from disk |

---

## Concepts at a Glance

### 1. `clap` Derive API — Typed CLI Arguments

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "fruit-salad", version, about)]
struct Args {
    /// Input fruit list (comma-separated)
    #[arg(short, long)]
    fruits: String,
    /// How many fruits to pick
    #[arg(short, long, default_value_t = 5)]
    count: usize,
    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();  // auto --help, --version
    println!("Picking {} of {}", args.count, args.fruits);
}
```

In Python: `argparse.ArgumentParser()` with `add_argument` calls — verbose, runtime-validated, not type-checked.

### 2. `ratatui` — Full-Screen Terminal Dashboards

```rust
use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Block, Paragraph}, layout::{Layout, Constraint, Direction}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.size());
        f.render_widget(Paragraph::new("Pipeline status: OK").block(Block::bordered()), chunks[0]);
    })?;
    Ok(())
}
```

In Python: `prompt_toolkit` or `urwid` — both feel sluggish and don't survive terminal resizes gracefully.

### 3. Async CLI Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fetch a batch of URLs in parallel
    Fetch { urls: Vec<String> },
    /// Transform a CSV file
    Transform { input: String, output: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Fetch { urls } => fetch_all(&urls).await?,
        Cmd::Transform { input, output } => transform(&input, &output).await?,
    }
    Ok(())
}
```

In Python: `click` groups + `asyncio.run` — works, but startup tax makes the cost-per-invocation painful.

### 4. Library/CLI Separation — `lib.rs` + `main.rs`

```
my-cli/
├── Cargo.toml
├── src/
│   ├── lib.rs      ← ALL the logic (testable!)
│   └── main.rs     ← 10 lines, calls lib.rs
```

This pattern lets you **unit-test the CLI logic** by calling `lib::run(args)` directly, with no process spawn.

---

## Prerequisites

- Completed [Section 5: Concurrency](../../05-Concurrency/README.md) — async/await + Tokio are required for AsyncClap
- Comfortable with `Result`, `?`, and module organization
- Cargo installed and on `PATH`

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 01 | **CLISalad** — CLI with clap arg parsing | `clap` derive, `std::env`, pattern matching, `std::io` | Workshop |
| 02 | **CustomCLIFruitSalad** — advanced CLI + CSV | `clap` derive, CSV reading, `lib.rs`/`main.rs` separation, modules | Workshop |
| 03 | **RatatuiTUI** — terminal dashboard | `ratatui`, `crossterm`, `TestBackend`, immediate-mode UI, layouts, widgets, event loop | Workshop |
| 04 | **AsyncClap** — async CLI with subcommands | `clap` derive, `#[tokio::main]`, `ExitCode`, subcommand trees, JSON config | Workshop |

## Learning Path

1. **01-CLISalad** — your first `clap` CLI: derive a struct, parse args, run code
2. **02-CustomCLIFruitSalad** — split logic into `lib.rs` / `main.rs`; learn to unit-test CLI code
3. **03-RatatuiTUI** — escape the line-by-line model: full-screen dashboards for monitoring pipelines
4. **04-AsyncClap** — subcommand trees + Tokio; the foundation of any real-world ETL CLI tool

---

## How This Section Fits in the Course

- **Builds on**: Section 3 (Collections, error handling), Section 5 (async/await, Tokio for the AsyncClap project)
- **Sets up for**: Section 7 (Graph projects use `clap` for input), Section 9 (Production CLI services), Section 10 (REST APIs with `clap`-style request validation)

For graph algorithms and Neo4j, jump to [Section 7: Graph & Network Science](../../07-GraphAndNetworkScience/README.md).

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

