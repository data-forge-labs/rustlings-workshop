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

## Why Release the GIL from Rust?

**Python pain:** A pure-Python `for i in range(50_000_000): total += i` blocks
every other Python thread on the same process. CPython's GIL (Global Interpreter
Lock) prevents true parallelism for CPU-bound work. Spawning 4 Python threads to
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
