# 🐍 GIL Release — Free the GIL, Free the CPU

*Subtitle: how `pyo3::Python::allow_threads` lets other Python threads actually run while Rust works.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 11 tests pass**.

> **Python build note**: `cargo test` runs without Python installed. To see the
> GIL release in real action, run `maturin develop --release --features python`
> and call `cpu_intensive_sum` from multiple Python threads.

---

## What Is GIL Release?

Freeing the Python GIL from Rust — letting other Python threads run while Rust computes.

### Python equivalent

```python
import threading

total = 0
def compute():
    global total
    for i in range(50_000_000):
        total += i  # blocks all other threads (GIL)

threads = [threading.Thread(target=compute) for _ in range(4)]
# All 4 threads run sequentially due to GIL
``` Spawning 4 Python threads to
do CPU work often **runs slower than 1 thread** because of GIL contention.

**Rust fix:** When CPython calls into a PyO3 function, you can wrap the
computation in `Python::allow_threads(|| ...)`. The GIL is released for the
duration; other Python threads can do I/O, NumPy releases, or even run pure-Python
code that doesn't hold the GIL. The Rust function runs on all cores freely.

```rust
#[pyfunction]
fn cpu_intensive_sum(py: Python<'_>, n: u64) -> u64 {
    py.allow_threads(|| (0..n).sum())
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | GIL | CPython interpreter lock | `sys.setswitchinterval` | The thing `allow_threads` releases |
| 2 | `Python::allow_threads` | `pyo3` API | n/a | The call that releases the GIL |
| 3 | `std::thread` | Native OS thread | `threading.Thread` | Rust threads aren't limited by the GIL |
| 4 | `available_parallelism` | `std::thread` | `os.cpu_count()` | Detect the worker's core count |
| 5 | GIL contention factor | `(serial / parallel)` | n/a | Quantify how badly the GIL serialises work |
| 6 | Long-running CPU | Rust loop | Python `for` | The work that needs the GIL released |
| 7 | Feature-gated FFI | `pyo3` (optional) | setuptools | Allow `cargo test` without Python |
| 8 | Pure-Rust split | lib.rs without `#[cfg]` | n/a | Same logic, Rust tests + Python caller |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 11 tests pass**.

> **Python build note**: `cargo test` runs without Python installed. To use the
> GIL-release pattern with real Python, run `maturin develop --release --features
> python` and call from Python; see workshop README for the Python demo.

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib gil_release_workshop
cd gil_release_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "gil_release_workshop"
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
cp "11-Interop/04-GILRelease/workshop/src/lib.rs" src/lib.rs
cp "11-Interop/04-GILRelease/workshop/src/main.rs" src/main.rs
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

### Step 1 — Basic

#### `cpu_intensive_sum`
- **Signature**: `pub fn cpu_intensive_sum(n: u64) -> u64`
- **Task**: `0..n.sum()` — simulates a long-running CPU computation. (In Python this is exactly the work that would block the GIL.)

### Step 2 — Contention

#### `is_single_threaded_bottleneck`
- **Signature**: `pub fn is_single_threaded_bottleneck(work_per_thread: u64, num_threads: usize) -> Duration`
- **Task**: Spawn `num_threads` threads, each computing `cpu_intensive_sum(work_per_thread)`. Join, return total elapsed.

#### `benchmark_parallel`
- **Signature**: `pub fn benchmark_parallel(work_per_thread: u64, num_threads: usize) -> (Duration, u64)`
- **Task**: Same as above but returns `(elapsed, total_work)` where `total_work = work_per_thread * num_threads`.

### Step 3 — Workers

#### `worker_count_active`
- **Signature**: `pub fn worker_count_active() -> usize`
- **Task**: `std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4)`.

### Step 4 — Release

#### `release_gil_simulation`
- **Signature**: `pub fn release_gil_simulation(work_units: u64) -> Duration`
- **Task**: Time `cpu_intensive_sum(work_units)`. (In PyO3, the equivalent would be wrapped in `Python::allow_threads(|| cpu_intensive_sum(work_units))` — see the `python` feature.)

### Step 5 — Validation

#### `validate_inputs`
- **Signature**: `pub fn validate_inputs(work: u64, threads: usize) -> Result<(), &'static str>`
- **Task**: Return `Err("work must be > 0")` if `work == 0`. Return `Err("threads must be > 0")` if `threads == 0`. Otherwise `Ok(())`.

### Step 6 — Metric

#### `gil_contention_factor`
- **Signature**: `pub fn gil_contention_factor(work_per_thread: u64, num_threads: usize) -> f64`
- **Task**: Ratio of single-thread time to multi-thread time. `(serial.as_secs_f64()) / (parallel.as_secs_f64())`. With 1 thread the factor is 1.0; with contention it drops below 1.0.

### Step 7 — Format

#### `format_result`
- **Signature**: `pub fn format_result(name: &str, duration: Duration, work: u64) -> String`
- **Task**: `format!("{}: {:?} (work={})", name, duration, work)`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_basic | 2 | cpu_intensive_sum |
| step_02_contention | 2 | bottleneck timing + parallel speedup |
| step_03_workers | 1 | worker_count_active |
| step_04_release | 1 | release_gil_simulation |
| step_05_validation | 2 | validate_inputs ok + error |
| step_06_metric | 2 | gil_contention_factor at 1 and 8 threads |
| step_07_format | 1 | format_result string |

## Building the Python Module (GIL release in real Python)

```python
import gil_release_workshop as r
import threading, time

# Without GIL release, 4 Python threads are limited to ~1 core.
# With Python::allow_threads inside the Rust function, all cores run free.
def python_loop(n):
    s = 0
    for i in range(n):
        s += i
    return s

start = time.time()
results = [r.cpu_intensive_sum(5_000_000) for _ in range(4)]
print("Python loops:", time.time() - start)

start = time.time()
threads = [threading.Thread(target=lambda: r.cpu_intensive_sum(5_000_000)) for _ in range(4)]
for t in threads: t.start()
for t in threads: t.join()
print("Rust threads (GIL released):", time.time() - start)
```

## How to Run Tests (Rust only)
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

