# 🐍 PyO3 Bindings — Call Rust Functions from Python

*Subtitle: expose Rust to Python as a native C-extension via `pyo3` and `maturin`.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

> **Python build note**: `cargo test` runs without Python installed. To import
> the module from Python, install `maturin` (`pip install maturin`) and run
> `maturin develop --release --features python`. See the workshop README for the
> full Python round-trip demo.

---

## Why Embed Rust Inside a Python Process?

**Python pain:** NumPy is fast, but custom numerical logic, regex, parsing,
cryptography, or anything CPU-heavy still bottlenecks pure-Python loops. A
team typically drops into Cython or C extensions, which is its own build-tool
nightmare and silently segfaults on bad refcounts.

**Rust fix:** With `pyo3` + `maturin` you write the hot path in Rust (memory
safe, no GC pauses, no refcounting) and call it from Python with zero-copy
buffers. The build is `maturin develop`; the import is `import <your_module>`.

```python
import pyo3_bindings_workshop as r
r.moving_average([1.0, 3.0, 5.0, 7.0], 2)  # [2.0, 4.0, 6.0]
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `#[pyfunction]` macro | `pyo3` | Cython `cdef` | Declare a function callable from Python |
| 2 | `#[pymodule]` macro | `pyo3` | Cython module init | Register functions in the Python module |
| 3 | Feature-gated FFI | `[features] python = ["dep:pyo3"]` | setuptools extras | Keep `cargo test` independent of Python headers |
| 4 | `cdylib` crate type | `crate-type = ["cdylib"]` | Cython `.so` | Produce a `.so`/`.pyd` Python can dlopen |
| 5 | Pure-Rust split | lib.rs without `#[cfg]` | n/a | Same logic, two consumers: Python + Rust tests |
| 6 | `maturin develop` | `cargo` workflow | `pip install -e .` | One-command build + install of a Rust extension |
| 7 | Vectorised slices | `&[f64]` ↔ `Vec<f64>` | NumPy `np.float64` | Natural interop with Python lists and arrays |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

> **Python build note**: To build the Python module, install `maturin`
> (`pip install maturin`) and run `maturin develop --release --features python`
> from this directory. The `cargo test` path runs without Python installed.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib pyo3_bindings_workshop
cd pyo3_bindings_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "pyo3_bindings_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
pyo3 = { version = "0.23", features = ["auto-initialize"], optional = true }
[features]
default = []
python = ["dep:pyo3"]
```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "11-Interop/03-PyO3Bindings/workshop/src/lib.rs" src/lib.rs
cp "11-Interop/03-PyO3Bindings/workshop/src/main.rs" src/main.rs
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

## Functions to Implement

### Step 1 — Basic vectorized operations

#### `double_values`
- **Signature**: `pub fn double_values(values: &[f64]) -> Vec<f64>`
- **Task**: `values.iter().map(|x| x * 2.0).collect()`

#### `sum_values`
- **Signature**: `pub fn sum_values(values: &[f64]) -> f64`
- **Task**: `values.iter().sum()`

### Step 2 — Transforms

#### `normalize`
- **Signature**: `pub fn normalize(values: &[f64]) -> Vec<f64>`
- **Task**: Find min and max; map each value to `(v - min) / (max - min)`. If `max == min`, return zeros.

### Step 3 — Windowed

#### `moving_average`
- **Signature**: `pub fn moving_average(values: &[f64], window: usize) -> Vec<f64>`
- **Task**: For each index `i` in `0..=(values.len() - window)`, average `values[i..i+window]`. Returns `values.len() - window + 1` elements.

### Step 4 — Counting

#### `count_above_threshold`
- **Signature**: `pub fn count_above_threshold(values: &[f64], threshold: f64) -> usize`
- **Task**: `values.iter().filter(|&&v| v > threshold).count()`

### Step 5 — Misc

#### `hello_from_rust`
- **Signature**: `pub fn hello_from_rust(name: &str) -> String`
- **Task**: `format!("Hello, {} from Rust!", name)`

#### `reverse_in_place`
- **Signature**: `pub fn reverse_in_place(values: &mut [f64])`
- **Task**: `values.reverse()`

#### `parse_csv_line`
- **Signature**: `pub fn parse_csv_line(line: &str) -> Vec<f64>`
- **Task**: `line.split(',').filter_map(|s| s.trim().parse().ok()).collect()`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_basic | 4 | double_values, sum_values |
| step_02_transforms | 2 | normalize to [0,1] + constant input |
| step_03_windowed | 2 | moving_average with windows 2 and 3 |
| step_04_counting | 2 | count_above_threshold with hit and miss |
| step_05_misc | 3 | hello + reverse + parse_csv_line |

## Building the Python Module

```bash
pip install maturin numpy
cd 11-Interop/03-PyO3Bindings/workshop
maturin develop --release --features python
python -c "import pyo3_bindings_workshop; print(pyo3_bindings_workshop.hello_from_rust('Alice'))"
```

## How to Run Tests (Rust only, no Python needed)
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

