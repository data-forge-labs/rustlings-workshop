# 🦀 YAML Config Files — Python to Rust Workshop

*Subtitle: Parse, generate, and validate YAML for data-pipeline configuration.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

---

## What Is This Project?

Typed YAML parsing with `serde_yaml` — turning untyped `dict`s into validated structs.

### Python equivalent

```python
import yaml

with open("config.yaml") as f:
    config = yaml.safe_load(f)  # returns a dict — no schema, no validation

# A typo like pool_szie silently becomes None
db_pool = config["database"]["pool_szie"]  # KeyError or None
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | YAML parsing | Typed deserialization into structs |
| 2 | Derive macros | `#[derive(Deserialize)]` — compile-time checks |
| 3 | Nested structs | Schema enforces shape |
| 4 | Serialization | Generate config from code |
| 5 | Custom enums | YAML keys map to enum variants |
| 6 | Error handling | Errors carry line/column info |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib yaml_workshop
cd yaml_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "yaml_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "04-FileIO/05-YAML/workshop/src/lib.rs" src/lib.rs
cp "04-FileIO/05-YAML/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib yaml_workshop
cd yaml_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "yaml_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "04-FileIO/05-YAML/workshop/src/lib.rs" src/lib.rs
cp "04-FileIO/05-YAML/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: `serde` and `serde_yaml`](#3-concept-serde-and-serde_yaml)
4. [Concept: Deriving `Deserialize` and `Serialize`](#4-concept-deriving-deserialize-and-serialize)
5. [Concept: Custom Enums with `rename_all`](#5-concept-custom-enums-with-rename_all)
6. [Concept: File I/O Wrapping Deserialization](#6-concept-file-io-wrapping-deserialization)
7. [Concept: Config Merging and Queries](#7-concept-config-merging-and-queries)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

YAML is the lingua franca of DevOps and data engineering. Airflow DAGs, Kubernetes manifests, GitHub Actions, dbt profiles, Dagster pipelines — all YAML. Reading it correctly is a core skill.

**Python to Rust:** Python's `yaml.safe_load` returns `Any` (a dict-of-dicts). You have to manually validate. Rust's `serde::Deserialize` derives a schema at compile time — your struct IS the schema, and the compiler enforces it.

**Data-engineering motivation:** Pipeline configs include database credentials, source paths, schedules, and retry policies. One typo can cause data loss. Typed config catches these at compile time.

## 2. Prerequisites

- Completed [04-FileIO/04-Arrow](../../04-Arrow/README.md) — familiar with progressive `todo!()` workshops.
- Comfortable with `serde::Serialize`/`Deserialize` from [04-FileIO/02-CSVWriter](../../02-CSVWriter/README.md).
- Understands `Result<T, E>` and `Box<dyn Error>` from [Section 1: Foundations](../../../../01-Foundations/README.md).

## 3. Concept: `serde` and `serde_yaml`

`serde` is Rust's **se**rialization and **de**serialization framework. It defines the `Serialize` and `Deserialize` traits; data-format crates (`serde_json`, `serde_yaml`, `bincode`) implement those traits for their formats.

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Point { x: i32, y: i32 }

let p: Point = serde_yaml::from_str("x: 1\ny: 2\n")?;
```

**In Python**, this is closest to `pydantic.BaseModel`:

```python
from pydantic import BaseModel

class Point(BaseModel):
    x: int
    y: int

p = Point(x=1, y=2)
```

The big difference: Pydantic validates at runtime when the model is constructed; `serde` validates at deserialize-time, but the type system means you cannot construct an invalid `Point` in the first place.

## 4. Concept: Deriving `Deserialize` and `Serialize`

`#[derive(Deserialize)]` asks the compiler to generate a parser that walks the struct fields by name and pulls each one from the input. The default mapping is **field name = YAML key**.

Nested structs work transparently — `PipelineConfig { database: DatabaseConfig, ... }` recursively deserializes each nested struct.

For our project, the schema is:

```yaml
database:
  host: localhost
  port: 5432
sources:
  - name: sales
    path: /data/sales.csv
schedule:
  cron: "0 */6 * * *"
```

maps to:

```rust
struct PipelineConfig {
    database: DatabaseConfig,
    sources: Vec<DataSource>,
    schedule: Schedule,
}
```

## 5. Concept: Custom Enums with `rename_all`

The `JobStatus` enum serializes lowercase:

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum JobStatus { Success, Failed, Skipped }
```

This produces YAML keys `success`, `failed`, `skipped` — matching the convention used in Airflow, GitHub Actions, and most CI systems.

**In Python**, you would write a custom `dict` or use `strEnum`:

```python
class JobStatus(str, Enum):
    success = "success"
    failed = "failed"
    skipped = "skipped"
```

The `rename_all` attribute supports many cases: `"lowercase"`, `"UPPERCASE"`, `"PascalCase"`, `"camelCase"`, `"snake_case"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`, `"verbatim"` (use field name as-is).

## 6. Concept: File I/O Wrapping Deserialization

For production use, configs live on disk. A common Rust pattern is to wrap the deserializer in a file-read function:

```rust
pub fn read_pipeline_file(path: &str) -> Result<PipelineConfig, Box<dyn Error>> {
    let yaml = fs::read_to_string(path)?;
    let cfg = serde_yaml::from_str(&yaml)?;
    Ok(cfg)
}
```

The `Box<dyn Error>` return type lets us propagate both `std::io::Error` and `serde_yaml::Error` without a custom error enum. For larger codebases, prefer a `thiserror` enum.

**In Python**, this is just:

```python
import yaml
with open("config.yaml") as f:
    cfg = yaml.safe_load(f)
```

The Python version loses the return type — you don't know whether `f` opened successfully or whether YAML parsing succeeded. Rust's `Result` chains them.

## 7. Concept: Config Merging and Queries

Pipeline configs often come in layers: a base config plus environment-specific overrides. The merging logic here is deliberately simple — override's `database` wins, base's `sources` and `schedule` are kept:

```rust
let merged = PipelineConfig {
    database: override_cfg.database,
    sources: base_cfg.sources,
    schedule: base_cfg.schedule,
};
```

For more sophisticated merging (deep-merge with type coercion), look at `config` crate or `figment`.

For querying, `BTreeSet<String>` gives a sorted, deduplicated list:

```rust
use std::collections::BTreeSet;
let mut names: BTreeSet<String> = config.sources.iter().map(|s| s.name.clone()).collect();
```

**In Python**, `set()` does the same, but ordering is insertion-based and unstable.

## 8. Putting It All Together

The `lib.rs` file is structured in five progressive steps:

1. **Step 1 (`step_01_serde_derive`)** — parse YAML into a typed `PipelineConfig`.
2. **Step 2 (`step_02_serialize_roundtrip`)** — write a config back to YAML, parse it again, verify equality.
3. **Step 3 (`step_03_file_io`)** — read/write YAML files to disk.
4. **Step 4 (`step_04_job_results`)** — custom `JobStatus` enum with `rename_all = "lowercase"`.
5. **Step 5 (`step_05_merge_and_query`)** — merge configs, dedupe with `BTreeSet`, count by format.

The `main.rs` ties them together: read a pipeline YAML, merge an override, write the result.

## 9. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs). The sample input is at [`workshop/data/pipeline.yaml`](workshop/data/pipeline.yaml).

## 10. Summary

| Concept | Used In |
|---------|---------|
| `#[derive(Deserialize)]` | All `step_01_serde_derive` tests |
| `serde_yaml::from_str` | `parse_pipeline_config` |
| `serde_yaml::to_string` | `write_pipeline_config` |
| `fs::read_to_string` | `read_pipeline_file` |
| `fs::write` | `write_pipeline_file` |
| `#[serde(rename_all = "lowercase")]` | `JobStatus` enum |
| Struct composition | `PipelineConfig` |
| `BTreeSet` | `unique_source_names` |
| Iterator `filter` + `count` | `count_sources_by_format` |

## Further Reading

- [serde documentation](https://serde.rs/) — covers all derive attributes
- [serde_yaml crate docs](https://docs.rs/serde_yaml/) — note the project is in maintenance mode; consider `serde_yml` for new code
- [config crate](https://docs.rs/config/) — production-grade config layering
- [figment](https://docs.rs/figment/) — popular Rocket framework config library

## Exercises

1. **Easy**: Add a function `find_source_by_name(config: &PipelineConfig, name: &str) -> Option<&DataSource>` with 1 test.
2. **Medium**: Add a `RetryPolicy { max_retries: u32, backoff_ms: u64 }` field to `PipelineConfig` and add a test that deserializes a YAML with this field.
3. **Hard**: Replace `merge_configs` with a deep-merge that recursively merges nested objects. Test that `merge_configs(base_with_nested_a, override_with_nested_a)` produces the union of all keys.

---

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.

## Functions to Implement

### Step 1 — `serde_yaml` + `serde::Deserialize`

#### `parse_pipeline_config`
- **Signature**: `pub fn parse_pipeline_config(yaml: &str) -> Result<PipelineConfig, serde_yaml::Error>`
- **Task**: Use `serde_yaml::from_str` to deserialize YAML into a `PipelineConfig`.

### Step 2 — Serialization round-trip

#### `write_pipeline_config`
- **Signature**: `pub fn write_pipeline_config(config: &PipelineConfig) -> Result<String, serde_yaml::Error>`
- **Task**: Use `serde_yaml::to_string` to serialize back to YAML.

### Step 3 — File I/O

#### `read_pipeline_file`
- **Signature**: `pub fn read_pipeline_file(path: &str) -> Result<PipelineConfig, Box<dyn std::error::Error>>`
- **Task**: `fs::read_to_string` then `parse_pipeline_config`.

#### `write_pipeline_file`
- **Signature**: `pub fn write_pipeline_file(path: &str, config: &PipelineConfig) -> Result<(), Box<dyn std::error::Error>>`
- **Task**: Serialize then `fs::write`.

### Step 4 — Custom enums

#### `parse_job_results` / `serialize_job_results`
- **Task**: Use `#[serde(rename_all = "lowercase")]` on `JobStatus` so the YAML keys are `success`, `failed`, `skipped`.

### Step 5 — Merge and query

#### `merge_configs`
- **Signature**: `pub fn merge_configs(base: &str, override_yaml: &str) -> Result<PipelineConfig, serde_yaml::Error>`
- **Task**: Parse both, then build a new `PipelineConfig` taking the override's `database` and the base's `sources` and `schedule`.

#### `unique_source_names`
- **Signature**: `pub fn unique_source_names(config: &PipelineConfig) -> Vec<String>`
- **Task**: Use a `BTreeSet<String>` to dedupe and return a sorted `Vec<String>`.

#### `count_sources_by_format`
- **Signature**: `pub fn count_sources_by_format(config: &PipelineConfig, format: &str) -> usize`
- **Task**: Count sources whose `format` field matches.

## Structs

### `PipelineConfig`
- `database: DatabaseConfig`
- `sources: Vec<DataSource>`
- `schedule: Schedule`

### `DatabaseConfig`
- `host: String, port: u16, username: String, pool_size: u32`

### `DataSource`
- `name: String, path: String, format: String`

### `Schedule`
- `cron: String, enabled: bool`

### `JobResult` / `JobStatus`
- `JobResult { job_name, rows_written, duration_ms, status }`
- `JobStatus`: enum with `Success`, `Failed`, `Skipped` (serialized lowercase)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_serde_derive | 3 | Parse nested PipelineConfig from YAML |
| step_02_serialize_roundtrip | 2 | Serialize → parse returns same value |
| step_03_file_io | 2 | Read/write YAML to disk |
| step_04_job_results | 2 | Custom enum with rename_all = "lowercase" |
| step_05_merge_and_query | 3 | Merge configs, dedupe names, count by format |

## How to Run Tests
```bash
cargo test
```
