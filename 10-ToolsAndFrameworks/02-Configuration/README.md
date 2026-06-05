# Rust Configuration — Python configparser / pydantic Equivalent

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 11 tests pass**.

## Why Use the `config` Crate + `serde`?

**Python pain:** Configuration is fragmented — TOML/JSON/YAML/INI/ENV each have a different API, merging env vars with files is manual, and there's no type safety (every value comes out as a `dict` or `str`):

```python
import json, yaml, configparser, os
config = configparser.ConfigParser()
config.read("app.ini")
host = config.get("server", "host", fallback="localhost")
if os.getenv("APP_HOST"):            # manual env override
    host = os.getenv("APP_HOST")
```

**Rust fix:** The `config` crate + `serde` gives one unified API for every format, with compile-time type safety:

```rust
use config::{Config, FileFormat};
use serde::{Deserialize};
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub database_url: Option<String>,
}
```

Same struct parses TOML, JSON, YAML, or env vars. Missing or wrong-type fields become compile errors via `serde`, not runtime `KeyError`s.

---
        .and_then(|c| c.try_deserialize().map_err(|e| e.to_string()))
}
```

Swap `FileFormat::Toml` to `Json` or `Yaml` — the code stays the same. Environment variable overrides are a single `.set_override()` call. All type mismatches are caught at deserialization time, not as runtime type errors.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Serde Deserialize | `#[derive(Deserialize)]` | pydantic `BaseModel` | Type-safe config structs |
| 2 | Config builder | `config::Config::builder()` | `configparser.ConfigParser` | Construct layered configuration |
| 3 | `FileFormat` enum | `FileFormat::Toml` | `toml.load()` / `json.load()` | Parse TOML, JSON, or YAML uniformly |
| 4 | Source layering | `.add_source(File::from_str(...))` | manual `{**d1, **d2}` | Stack config sources with priority |
| 5 | Env overrides | `.set_override(key, value)` | `os.environ.get()` + manual | Apply env on top of file config |
| 6 | `try_deserialize` | `.try_deserialize::<AppConfig>()` | pydantic validation | Convert parsed config to typed struct |
| 7 | Match fallback | `match key { ... _ => "" }` | `config.get(key, default)` | Return field values with default |

---

## Concepts at a Glance

**1. Serde Deserialize derive** — Python's pydantic `BaseModel` validates at runtime during construction. Rust's `#[derive(Deserialize)]` generates a parser at compile time — the struct definition *is* the schema. Mismatched fields produce immediate deserialization errors, not silent NaN/null values.

**2-3. Config builder & FileFormat** — Python uses separate libraries per format (toml, json, yaml). Rust's `Config::builder()` works identically for all formats — only the `FileFormat::*` variant changes. One pattern, many backends.

**4. Source layering** — Python requires manual `{**file_config, **env_config}` dict merging. Rust's `.add_source()` stacks sources — later sources override earlier ones. Add a file, then env vars, then CLI args — all with the same builder API.

**5. Environment overrides** — Python's `os.environ.get()` returns strings needing manual type conversion. Rust's `.set_override(key, value)` feeds into the same deserialization pipeline — type conversion happens automatically via serde.

**6-7. try_deserialize & fallback** — pydantic validates types at model construction. Rust's `try_deserialize::<AppConfig>()` is equivalent: it converts unstructured config into a typed struct, failing fast on mismatches. The `match` fallback in `get_or_default` mirrors Python's `config.get(key, default)`.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Serde — The Universal Serialization Framework](#3-serde--the-universal-serialization-framework)
4. [Parsing Config Formats — TOML, JSON, YAML](#4-parsing-config-formats--toml-json-yaml)
5. [Merging Config Sources — File + Environment](#5-merging-config-sources--file--environment)
6. [Defaults and Fallbacks](#6-defaults-and-fallbacks)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

Every data pipeline needs configuration: database URLs, hostnames, feature flags. In Python you use `configparser` for INI files, `pyyaml` for YAML, or `pydantic` for validated settings. Rust's approach is unified through the **`config` crate** and **`serde`** — one pattern for all formats.

**What you'll learn:**
- Deserializing config from TOML, JSON, and YAML with the `config` crate
- Using `serde` derive to define config structs (like pydantic models)
- Environment variable overrides (like `os.environ` in Python)
- Default values and fallback patterns

## 2. Prerequisites

- Structs, derives (`#[derive(Debug)]`)
- `Result<T, E>` and error handling
- **Projects**: [01-BasicCalculator](../../01-Foundations/01-Intro/README.md), [02-Traits](../../02-Ownership/02-Traits/README.md)

## 3. Serde — The Universal Serialization Framework

### Explanation

In Python you write a dataclass and optionally validate it with pydantic:
```python
from dataclasses import dataclass
@dataclass
class AppConfig:
    host: str = "localhost"
    port: int = 8080
```

In Rust, `serde` provides the `Serialize` and `Deserialize` derives so your struct can be parsed from any format:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub database_url: Option<String>,
}
```

| Concept | Python | Rust |
|---------|--------|------|
| Model definition | `@dataclass` or pydantic model | `struct` + `#[derive(Deserialize)]` |
| Optional fields | `Optional[str]` / `None` default | `Option<String>` |
| Field validation | pydantic validators | Custom `Deserialize` impl or `validator` crate |
| Format conversion | `.json()`, `yaml.dump()` | `serde_json`, `serde_yaml` via `config` crate |

Serde powers everything — the `config` crate uses it under the hood.

### Applying to Our Project

Our `AppConfig` struct is already defined in `workshop/src/lib.rs`. Every parse function will return this same type.

## 4. Parsing Config Formats — TOML, JSON, YAML

### Explanation

Python has separate libraries per format:
```python
import json, yaml, configparser
```

Rust's `config` crate unifies them under one API. Here's how you load a TOML file — the most common Rust format (used by Cargo itself):

```rust
use config::{Config, File, FileFormat};

fn load() -> Result<AppConfig, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::from_str(toml_str, FileFormat::Toml))
        .build()?;
    settings.try_deserialize::<AppConfig>()
}
```

TOML is the idiomatic Rust format. In Python you'd write `config.toml`:
```toml
host = "localhost"
port = 8080
debug = true
database_url = "postgres://localhost/mydb"
```

### Example

```rust
// parse_toml_config implementation hint:
use config::{Config, FileFormat};

let settings = Config::builder()
    .add_source(config::File::from_str(toml_str, FileFormat::Toml))
    .build()
    .map_err(|e| e.to_string())?;
settings.try_deserialize::<AppConfig>().map_err(|e| e.to_string())
```

### Applying to Our Project

Implement `parse_toml_config`, `parse_json_config`, and `parse_yaml_config` using `Config::builder().add_source(File::from_str(...))` with the corresponding `FileFormat` variant. Each returns `Result<AppConfig, String>`.

Available formats: `FileFormat::Toml`, `FileFormat::Json`, `FileFormat::Yaml`.

## 5. Merging Config Sources — File + Environment

### Explanation

In Python you manually merge:
```python
import os
config = load_toml("config.toml")
config["debug"] = os.getenv("DEBUG", config.get("debug", "false"))
```

The Rust `config` crate supports layered sources natively — later sources override earlier ones:

```rust
let settings = Config::builder()
    .add_source(File::from_str(toml_str, FileFormat::Toml))
    .set_override("debug", "true")  // like env var override
    .build()?;
```

This is the same pattern as `os.environ` overrides in Python web frameworks (e.g., Django's `python-decouple`).

### Applying to Our Project

`merge_config` receives a TOML string and an optional env override pair `(key, value)`. If the override is provided, apply it on top of the file config. Return the merged `AppConfig`.

## 6. Defaults and Fallbacks

### Explanation

Python pattern:
```python
host = config.get("host", "localhost")
port = config.getint("port", 8080)
```

Rust's `config` crate supports `.get_string(key)` with default, but with `serde` you can also set defaults directly in the struct using `#[serde(default)]`.

### Applying to Our Project

`get_or_default` returns the value for a given key from `AppConfig` as a string. If the key doesn't exist, return an empty string `""`.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement each function:

**`parse_toml_config`** — Use `Config::builder().add_source(File::from_str(toml_str, FileFormat::Toml)).build()` then `try_deserialize::<AppConfig>()`.

**`parse_json_config`** — Same pattern with `FileFormat::Json`.

**`parse_yaml_config`** — Same pattern with `FileFormat::Yaml`.

**`merge_config`** — Build config from TOML string, then conditionally call `set_override` with the env key/value if `Some`.

**`get_or_default`** — Match on `key` to return the appropriate field. For missing keys, return `""`.

Run tests after each function:
```bash
cd workshop && cargo test
```

## 8. Complete Code Reference

```rust
use config::{Config, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub database_url: Option<String>,
}

fn parse_from_str(source: &str, format: FileFormat) -> Result<AppConfig, String> {
    Config::builder()
        .add_source(config::File::from_str(source, format))
        .build()
        .map_err(|e| e.to_string())
        .and_then(|c| c.try_deserialize::<AppConfig>().map_err(|e| e.to_string()))
}

pub fn parse_toml_config(toml_str: &str) -> Result<AppConfig, String> {
    parse_from_str(toml_str, FileFormat::Toml)
}

pub fn parse_json_config(json_str: &str) -> Result<AppConfig, String> {
    parse_from_str(json_str, FileFormat::Json)
}

pub fn parse_yaml_config(yaml_str: &str) -> Result<AppConfig, String> {
    parse_from_str(yaml_str, FileFormat::Yaml)
}

pub fn merge_config(
    file_config: &str,
    env_override: Option<(&str, &str)>,
) -> Result<AppConfig, String> {
    let mut builder = Config::builder()
        .add_source(config::File::from_str(file_config, FileFormat::Toml));
    if let Some((key, value)) = env_override {
        builder = builder.set_override(key, value).map_err(|e| e.to_string())?;
    }
    builder
        .build()
        .map_err(|e| e.to_string())
        .and_then(|c| c.try_deserialize::<AppConfig>().map_err(|e| e.to_string()))
}

pub fn get_or_default(config: &AppConfig, key: &str) -> String {
    match key {
        "host" => config.host.clone(),
        "port" => config.port.to_string(),
        "debug" => config.debug.to_string(),
        "database_url" => config.database_url.clone().unwrap_or_default(),
        _ => String::new(),
    }
}
```

## 9. Summary

| Concept | Where Used | Python Equivalent |
|---------|-----------|-------------------|
| `serde::Deserialize` | Config struct derive | pydantic model |
| `Config::builder()` | All parse functions | `configparser.ConfigParser` |
| `FileFormat::Toml` | TOML parsing | `toml.load()` |
| `FileFormat::Json` | JSON parsing | `json.load()` |
| `FileFormat::Yaml` | YAML parsing | `yaml.safe_load()` |
| `set_override()` | Env variable merge | `os.environ.get()` override |
| Pattern matching fallback | `get_or_default` | `config.get(key, default)` |
