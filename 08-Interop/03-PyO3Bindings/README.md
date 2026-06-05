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
