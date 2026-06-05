# Section 11: Rust + Python Interop

*Bridge the two worlds: use Rust from Jupyter notebooks, and call pandas from Rust.*

---

## Why This Section?

### The Problem — You Don't Have to Choose

Many teams face a false choice:

```
┌─────────────────────────────────────────────────────┐
│  "Should we rewrite everything in Rust?"             │
│  ─────────────────────────────────────────────────── │
│                                                      │
│  Option A: Keep everything in Python                 │
│  ✓ Fast prototyping, rich ecosystem (pandas, sklearn)│
│  ✗ Slow for compute-heavy tasks                      │
│  ✗ GIL-limited concurrency                           │
│  ✗ Hard to distribute (no single binary)             │
│                                                      │
│  Option B: Rewrite everything in Rust                │
│  ✓ Blazing fast, safe, single binary                 │
│  ✗ Months of rewrites                                │
│  ✗ Smaller ecosystem for data science                │
│  ✗ Team needs to learn Rust first                    │
│                                                      │
│  Option C: Use BOTH (best answer)                    │
│  ▼                                                   │
└─────────────────────────────────────────────────────┘
```

### The Rust Solution — Interoperability

Rust doesn't force you to abandon Python. Instead, it integrates with your existing Python workflow:

```
  ┌─────────────────────────────────────────────────┐
  │  Your Data Pipeline                              │
  │                                                   │
  │  ┌──────────┐    ┌──────────┐    ┌──────────┐   │
  │  │ Python   │    │ Rust     │    │ Python   │   │
  │  │ (prototype│───►│ (speed up│───►│ (analysis│   │
  │  │  & setup)│    │  hot path│    │  & viz)  │   │
  │  └──────────┘    └──────────┘    └──────────┘   │
  │       │               │               │          │
  │       ▼               ▼               ▼          │
  │  ┌──────────┐    ┌──────────┐    ┌──────────┐   │
  │  │ Pandas   │    │ Rayon    │    │ matplotlib│   │
  │  │ Jupyter  │    │ iterators│    │ Plotters │   │
  │  └──────────┘    └──────────┘    └──────────┘   │
  └─────────────────────────────────────────────────┘
```

This section shows you two concrete integration strategies:
1. **Run Rust inside Jupyter** (evcxr) — for exploration and teaching
2. **Pandas + Rust comparison** — understand what Rust does differently

---

## What You'll Learn

| # | Concept | Rust Tool / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Interactive Rust | `evcxr` Jupyter kernel | `ipykernel` | Run Rust cells in Jupyter |
| 2 | Array operations | `ndarray` crate | `numpy` | N-dimensional arrays |
| 3 | Plotting from Rust | `plotters` crate | `matplotlib` | Data visualization |
| 4 | Parallel computation | `rayon` crate | `concurrent.futures` | Automatic parallelization |
| 5 | DataFrames in Rust | `polars` (conceptual) | `pandas` | Fast DataFrame operations |
| 6 | Python-Rust FFI | PyO3 (conceptual) | Cython, C extensions | Call Rust from Python |
| 7 | Benchmarking | `criterion` crate | `timeit`, `pytest-benchmark` | Performance measurement |
| 8 | Type safety | Compile-time checks | Runtime duck typing | Catch errors before execution |

---

## Concepts at a Glance

### 1. `evcxr` — Rust in Jupyter Notebooks

```
  ┌─────────────────────────────────────────────────┐
  │  Jupyter Cell: %%rust                           │
  │  ┌───────────────────────────────────────────┐  │
  │  │ let x = vec![1, 2, 3, 4, 5];             │  │
  │  │ let sum: i32 = x.par_iter().sum();        │  │
  │  │ println!("Sum: {}", sum);                  │  │
  │  └───────────────────────────────────────────┘  │
  │  Output:                                         │
  │  Sum: 15                                         │
  └─────────────────────────────────────────────────┘
```

`evcxr` is a Rust kernel for Jupyter that compiles and runs each cell like Python — caching dependencies between cells.

### 2. `ndarray` — Like NumPy

```rust
use ndarray::Array2;

let mut a = Array2::<f64>::zeros((3, 4));
a[[0, 0]] = 1.0;
a[[1, 2]] = 3.5;
let sum = a.sum();  // element-wise sum
```

In Python: `np.zeros((3, 4))`

### 3. `plotters` — Visualization

```rust
use plotters::prelude::*;

let root = SVGBackend::new("chart.svg", (640, 480)).into_drawing_area();
let mut chart = ChartBuilder::on(&root)
    .caption("Data Pipeline Throughput", ("sans-serif", 20))
    .build_cartesian_2d(0..10, 0..100)?;
chart.configure_mesh().draw()?;
chart.draw_series(LineSeries::new(
    (0..10).map(|x| (x, x * 10)),
    &RED,
))?;
```

In Python: `matplotlib.pyplot.plot(x, y)`

### 4. `rayon` — Automatic Parallelization

```rust
use rayon::prelude::*;

let numbers: Vec<i64> = (0..1_000_000).collect();
let sum: i64 = numbers.par_iter().sum();  // auto-parallel
```

This is the key advantage of Rust in Jupyter: **trivial parallelism without the GIL**.

### 5. DataFrame Comparison — Pandas vs Rust

```python
# Python — pandas
df = pd.read_csv("data.csv")
filtered = df[df["value"] > 0]
grouped = filtered.groupby("category")["value"].sum()
```

```rust
// Rust — using csv + HashMap
let mut reader = csv::Reader::from_path("data.csv")?;
let mut totals: HashMap<String, f64> = HashMap::new();
for result in reader.deserialize() {
    let record: Record = result?;
    if record.value > 0.0 {
        *totals.entry(record.category).or_insert(0.0) += record.value;
    }
}
// Same result, 10x faster, 5x less memory
```

---

## Prerequisites

- Completed [Section 5: Concurrency](../05-Concurrency/README.md)
- Familiar with Jupyter notebooks

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 57 | **ExploringPandas** — Rust meets pandas | DataFrame operations, Python/Rust interop, filtering/grouping, matplotlib | Project |
| 58 | **RustJupyterNotebook** — interactive Rust with evcxr | `evcxr` Jupyter kernel, interactive Rust, `plotters`/`ndarray`/`rayon` | Project |
| 59 | **PyO3Bindings** — Call Rust from Python | `pyo3` 0.23, `#[pyfunction]` / `#[pymodule]`, `cdylib`, `maturin develop`, feature-gated FFI | Project |
| 60 | **GILRelease** — Free the GIL, free the CPU | `pyo3::Python::allow_threads`, GIL contention factor, multi-threaded CPU work | Project |

## Learning Path

1. Start with **02-RustJupyterNotebook** to set up interactive Rust in Jupyter
2. Explore **01-ExploringPandas** to see Rust and Python working together
3. **03-PyO3Bindings** shows the *producer* direction — write a Python module in Rust
4. **04-GILRelease** shows the killer feature: `Python::allow_threads` releases the GIL so other Python threads can run while Rust computes
