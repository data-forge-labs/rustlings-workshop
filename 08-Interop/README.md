# Section 8: Rust + Python Interop

*Bridge the two worlds: use Rust from Jupyter notebooks, and call pandas from Rust.*

## Prerequisites

- Completed [Section 5: Concurrency](../05-Concurrency/README.md)
- Familiar with Jupyter notebooks

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 57 | **ExploringPandas** — Rust meets pandas | DataFrame operations, Python/Rust interop, filtering/grouping, matplotlib | Project |
| 58 | **RustJupyterNotebook** — interactive Rust with evcxr | `evcxr` Jupyter kernel, interactive Rust, `plotters`/`ndarray`/`rayon` | Project |

## Learning Path

1. Start with **58-RustJupyterNotebook** to set up interactive Rust in Jupyter
2. Explore **57-ExploringPandas** to see Rust and Python working together

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `evcxr` | `ipykernel` | Run Rust in Jupyter |
| `plotters` | `matplotlib` | Plotting from Rust |
| `ndarray` | `numpy` | Array operations |
| `rayon` | N/A | Parallel computation |
| PyO3 (conceptual) | Cython / C extensions | Python-Rust FFI |
