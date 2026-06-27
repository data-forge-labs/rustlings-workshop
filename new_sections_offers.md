# Workshop Draft: Demystifying Data Engines (Building TurboClean.rs)

## 1. Workshop Overview

**Objective:** Build a high-performance, adversarial-resistant data cleansing engine in Rust, expose it to Python, and critically analyze the marketing claims of existing "Turbo" data tools.

**Core Philosophy:** 
```
Python wrappers claim innovation by hiding the engine.
Rust engineers claim innovation by building the engine.
```

**What We Will Build:**
A Rust library (`turboclean-core`) that streams multi-GB CSVs, enforces strict memory/compression limits to prevent bombing, auto-profiles data to generate cleaning rules, executes those rules via Apache Arrow/Polars, and is exposed to Python via PyO3 as `turboclean_rs`.

---

## 2. File Structure (Cargo Workspace)

We use a workspace to separate concerns: library logic, CLI binary, and Python bindings.

```text
turboclean-rs-workshop/
├── Cargo.toml              # Workspace root
├── .python-version         # Python 3.10+
├── pyproject.toml          # Maturin build config for PyO3
├── data/                   # Test datasets (normal + adversarial)
│
├── crates/
│   ├── core/               # The pure Rust library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── error.rs        # Custom error types (thiserror)
│   │       ├── limits.rs       # Resource bounds & bomb detection
│   │       ├── rules.rs        # Cleaning rule definitions (Enum/Traits)
│   │       ├── profiler.rs     # Auto-magic heuristic generator
│   │       └── engine.rs       # The main Polars execution pipeline
│   │
│   ├── cli/                # The standalone binary
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs         # Clap definitions & CLI wiring
│   │
│   └── python/             # The PyO3 bindings
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs          # #[pymodule] definitions
```

---

## 3. Module Requirements (Dependencies)

### `crates/core/Cargo.toml`
```toml
[dependencies]
# The actual engine
polars = { version = "0.41", features = ["lazy", "csv", "parquet", "json", "streaming", "dtype-full"] }

# Security & Adversarial handling
flate2 = "1.0"          # Gzip decompression with limits

# Ergonomics
thiserror = "1.0"       # Library-level typed errors
serde = { version = "1.0", features = ["derive"] } # Config serialization
serde_json = "1.0"
log = "0.4"             # Logging interface
```

### `crates/cli/Cargo.toml`
```toml
[dependencies]
turboclean-core = { path = "../core" }
clap = { version = "4.5", features = ["derive"] }
env_logger = "0.11"
indicatif = "0.17"       # Progress bars
```

### `crates/python/Cargo.toml`
```toml
[dependencies]
turboclean-core = { path = "../core" }
pyo3 = { version = "0.21", features = ["extension-module"] }
```

---

## 4. Module Skeletons (What Students Implement)

### Module 1: `error.rs` (Typed Errors)
**Goal:** Teach library-grade error handling.
```rust
// Skeleton: Define the error enum mapping domain failures to Rust types
#[derive(Debug, thiserror::Error)]
pub enum TurboError {
    // IO failures (file not found)
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    
    // Polars execution failures
    #[error("Execution Error: {0}")]
    Execution(#[from] polars::prelude::PolarsError),
    
    // Adversarial failures
    #[error("Security Limit Exceeded: {msg}")]
    SecurityLimit { msg: String },
    
    // Schema/Logic failures
    #[error("Schema Mismatch: {0}")]
    Schema(String),
}
```

### Module 2: `limits.rs` (The "Unbreakable" Layer)
**Goal:** Teach RAII patterns and input validation. No data is read without passing through here.
```rust
// Skeleton: Resource constraints
pub struct ProcessingLimits {
    pub max_memory_bytes: usize,
    pub max_decompression_ratio: f64,
    pub max_columns: usize,
}

// Skeleton: Safe gzip reader wrapper
pub struct SafeGzipReader<R> { /* ... */ }

impl<R: std::io::Read> SafeGzipReader<R> {
    pub fn new(inner: R, compressed_size: usize, limits: &ProcessingLimits) -> Self { todo!() }
}

impl<R: std::io::Read> std::io::Read for SafeGzipReader<R> {
    // Skeleton: Implement read()
    // Requirement: Track bytes read. If bytes_read > (compressed_size * max_ratio), 
    // return Err(TurboError::SecurityLimit). This stops gzip bombs.
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> { todo!() }
}

pub fn validate_headers(headers: &[String], limits: &ProcessingLimits) -> Result<(), TurboError> {
    // Skeleton: Check if headers.len() > limits.max_columns (prevents 1M column crash)
    todo!()
}
```

### Module 3: `rules.rs` (The Domain Model)
**Goal:** Teach Enums, Pattern Matching, and Serde.
```rust
// Skeleton: The cleaning strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImputeStrategy {
    Mean, Median, Mode, ForwardFill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CleaningRule {
    DropNulls { columns: Vec<String> },
    ImputeMissing { column: String, strategy: ImputeStrategy },
    FilterOutliersIQR { column: String, factor: f64 },
    TrimWhitespace { columns: Vec<String> },
}

// Skeleton: Trait for custom/extensible rules (Advanced)
pub trait CustomRule: Send + Sync {
    fn name(&self) -> &str;
    fn to_expression(&self) -> Result<polars::prelude::Expr, TurboError>;
}
```

### Module 4: `profiler.rs` (The "Auto-Magic" Logic)
**Goal:** Teach Polars LazyExpressions for aggregation without materializing data.
```rust
// Skeleton: Profile statistics for a single column
#[derive(Debug, Serialize)]
pub struct ColumnProfile {
    pub null_count: u32,
    pub skewness: f64,
    pub is_numeric: bool,
}

pub struct Profiler { pub limits: ProcessingLimits }

impl Profiler {
    // Skeleton: Takes a LazyFrame, scans it, returns profiles
    // Key constraint: MUST use LazyFrame .collect() with streaming, 
    // NOT scan_csv().finish() which loads eagerly.
    pub fn profile(lf: polars::prelude::LazyFrame) -> Result<Vec<ColumnProfile>, TurboError> {
        // 1. Select columns
        // 2. Apply polars aggregations: col(x).null_count(), col(x).skew()
        // 3. Collect with streaming
        todo!()
    }

    // Skeleton: Heuristic rule generator
    pub fn suggest_rules(profiles: &[ColumnProfile]) -> Vec<CleaningRule> {
        // Logic: if skewness > 2.0 -> push CleaningRule::FilterOutliersIQR
        //        if null_count > 0 -> push CleaningRule::ImputeMissing
        todo!()
    }
}
```

### Module 5: `engine.rs` (The Executor)
**Goal:** Teach query plan building.
```rust
pub struct CleaningEngine {
    limits: ProcessingLimits,
}

impl CleaningEngine {
    // Skeleton: The main pipeline
    pub fn clean(
        &self,
        path: &str,
        rules: Vec<CleaningRule>,
        output_path: &str,
    ) -> Result<CleaningReport, TurboError> {
        // 1. Open file, wrap in SafeGzipReader (if .gz)
        // 2. Validate headers
        // 3. Create LazyCsvReader
        // 4. Iterate through rules, match on CleaningRule enum, 
        //    map to Polars Expr, apply .with_columns() or .filter()
        // 5. Execute .with_streaming(true).sink_parquet(output_path)
        todo!()
    }
}

pub struct CleaningReport {
    pub rows_read: u64,
    pub rows_written: u64,
    pub rules_applied: usize,
}
```

### Module 6: `python/src/lib.rs` (The PyO3 Bridge)
**Goal:** Teach FFI and Python packaging.
```rust
use pyo3::prelude::*;

#[pymodule]
fn turboclean_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // Skeleton: Expose the cleaning function to Python
    #[pyfn(m)]
    fn clean_file(py: Python, input_path: &str, output_path: &str, auto_magic: bool) -> PyResult<String> {
        // 1. Convert py::exceptions into TurboError
        // 2. Call turboclean_core::CleaningEngine
        // 3. Return JSON string of CleaningReport
        todo!()
    }

    // Skeleton: Expose profiling to Python
    #[pyfn(m)]
    fn profile_file(py: Python, input_path: &str) -> PyResult<String> {
        todo!()
    }

    Ok(())
}
```

---

## 5. How to Import & Use in Python

Students will use `maturin` to compile the Rust code into a Python wheel.

### Build Steps (in terminal)
```bash
# Navigate to workspace root
cd turboclean-rs-workshop

# Install maturin
pip install maturin

# Compile the Rust library and install it in the current venv
# --release is crucial for actual performance benchmarks
maturin develop --release
```

### Python Usage (`test_pipeline.py`)
```python
import turboclean_rs
import json

# 1. The "Auto-Magic" mode (replicating the Python TurboClean API)
print("Running auto-magic clean...")
report_json = turboclean_rs.clean_file(
    input_path="data/dirty_10gb.csv", 
    output_path="data/clean.parquet",
    auto_magic=True
)
report = json.loads(report_json)
print(f"Cleaned {report['rows_read']} down to {report['rows_written']} rows.")

# 2. The Adversarial Test (What the Python version fails at)
print("\nTesting Gzip Bomb...")
try:
    turboclean_rs.clean_file(
        input_path="data/bomb.csv.gz", 
        output_path="data/blocked.parquet",
        auto_magic=False
    )
except Exception as e:
    print(f"Blocked successfully: {e}")
```

---

## 6. Workshop Flow & Milestones

| Hour | Module | Student Deliverable |
|------|--------|---------------------|
| **0.5** | **Deconstruction** | Run Python TurboClean via `cProfile`. Document that 95% of time is inside `polars` C/Rust extensions. |
| **1.5** | **Limits & Safety** | `SafeGzipReader` passes bomb tests. `validate_headers` rejects 100k column file without crashing. |
| **1.5** | **Rules & Profiler** | `suggest_rules` correctly outputs `ImputeMissing` for a 50% null CSV without loading it into RAM. |
| **1.5** | **Engine & Polars** | `clean_file` successfully processes a 5GB CSV using < 500MB RAM (verified via `htop`/Activity Monitor). |
| **1.0** | **PyO3 & CLI** | `maturin develop` works. Python script imports `turboclean_rs` and runs the pipeline. |
| **0.5** | **Benchmarking** | Create a table comparing: Python wrapper, Direct Python Polars, Rust CLI, Rust via PyO3. |

## 7. The "Gotchas" Checklist for Instructors

When reviewing student PRs, automatically fail if you see:
1. **`CsvReader::new(file).finish()`**: They loaded the whole file into RAM. (Should be `LazyCsvReader`).
2. **`.unwrap()` in library code**: They didn't handle the bomb/header errors gracefully.
3. **Row-by-row iteration**: `for row in df.iter()`. They must use Polars vectorized expressions.
4. **Missing `--release`**: If they benchmark using `maturin develop` (debug mode), Rust will look 10x slower than Python.


To make this a truly bulletproof instructor's guide, I have prepared **Instructor Addendums**. These are deep-dive expansions, exact code implementations for the hardest parts, and advanced "gotchas" that you can plug directly into your existing markdown.

---

### Addendum 1: The PyO3 GIL Release (Crucial for Module 6)
*Insert this into **Module 6: `python/src/lib.rs`**.*

**Instructor Note:** Data engineers often complain that "Rust is slow in Python." 90% of the time, this is because they are holding the Python Global Interpreter Lock (GIL) during the heavy Rust computation, blocking all other Python threads. You must teach `py.allow_threads()`.

```rust
use pyo3::prelude::*;
use turboclean_core::{CleaningEngine, ProcessingLimits, TurboError};

#[pymodule]
fn turboclean_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    
    #[pyfn(m)]
    fn clean_file(py: Python, input_path: &str, output_path: &str) -> PyResult<String> {
        // CRITICAL: Release the GIL! 
        // This allows other Python threads to run while Rust processes the 10GB file.
        let result = py.allow_threads(|| {
            let engine = CleaningEngine::new(ProcessingLimits::default());
            // We pass empty rules for auto-magic, or parse them from a JSON string
            engine.clean(input_path, vec![], output_path) 
        });

        // Map Rust errors to Python exceptions
        let report = result.map_err(|e| match e {
            TurboError::SecurityLimit { msg } => {
                pyo3::exceptions::PyValueError::new_err(format!("Security Blocked: {}", msg))
            }
            _ => pyo3::exceptions::PyRuntimeError::new_err(e.to_string()),
        })?;

        // Serialize report to JSON string for Python to parse
        serde_json::to_string(&report)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    Ok(())
}
```

---

### Addendum 2: The Exact `SafeGzipReader` Implementation (Crucial for Module 2)
*Insert this into **Module 2: `limits.rs`**.*

**Instructor Note:** Students will struggle with the `std::io::Read` trait. Provide this skeleton to show them how to wrap `flate2::read::GzDecoder` and intercept the byte stream to enforce the "bomb" limit.

```rust
use flate2::read::GzDecoder;
use std::io::Read;

pub struct SafeGzipReader<R: Read> {
    decoder: GzDecoder<R>,
    bytes_read: u64,
    max_bytes: u64, // Calculated as: compressed_size * max_decompression_ratio
}

impl<R: Read> SafeGzipReader<R> {
    pub fn new(inner: R, compressed_size: usize, limits: &ProcessingLimits) -> Self {
        Self {
            decoder: GzDecoder::new(inner),
            bytes_read: 0,
            max_bytes: (compressed_size as f64 * limits.max_decompression_ratio) as u64,
        }
    }
}

impl<R: Read> Read for SafeGzipReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // 1. Read from the underlying gzip decoder
        let bytes_decompressed = self.decoder.read(buf)?;
        
        // 2. Track the bytes
        self.bytes_read += bytes_decompressed as u64;
        
        // 3. Check against limits (The "Bomb" Detector)
        if self.bytes_read > self.max_bytes {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Gzip bomb detected! Decompressed {} bytes, limit is {}.", 
                    self.bytes_read, self.max_bytes
                ),
            ));
        }
        
        Ok(bytes_decompressed)
    }
}
```

---

### Addendum 3: Mixing Eager and Lazy for IQR (Crucial for Module 5)
*Insert this into **Module 5: `engine.rs`**.*

**Instructor Note:** Implementing `FilterOutliersIQR` in a purely Lazy pipeline is a classic trap. You cannot filter by Q1/Q3 without knowing what Q1/Q3 are. Teach students the pattern of **"Eager for Metadata, Lazy for Data."**

```rust
impl CleaningEngine {
    // ... inside the rule matching loop ...
    
    CleaningRule::FilterOutliersIQR { column, factor } => {
        // STEP 1: Eagerly calculate the bounds. 
        // This is incredibly cheap (returns exactly 2 rows) and doesn't violate streaming.
        let bounds_df = lf.clone().select([
            col(&column).quantile(lit(0.25), QuantileInterpolOptions::Linear).alias("q1"),
            col(&column).quantile(lit(0.75), QuantileInterpolOptions::Linear).alias("q3"),
        ]).collect()?; // Eager execution here is fine!

        let q1 = bounds_df.column("q1")?.f64()?.get(0).unwrap_or(0.0);
        let q3 = bounds_df.column("q3")?.f64()?.get(0).unwrap_or(0.0);
        let iqr = q3 - q1;
        let lower = q1 - (factor * iqr);
        let upper = q3 + (factor * iqr);

        // STEP 2: Apply the filter Lazily using the scalar values
        lf = lf.filter(
            col(&column).is_between(lit(lower), lit(upper))
        );
    }
```

---

### Addendum 4: Advanced "Gotchas" for Data Engineers
*Add these to **Section 7: The "Gotchas" Checklist**.*

When reviewing student PRs, look for these specific Data Engineering anti-patterns in Rust:

1. **The `.fetch()` Trap:** 
   * *The Sin:* Using `.fetch(10)` to "preview" data. 
   * *The Reality:* In Polars Lazy API, `.fetch(10)` pulls 10 rows *per partition*, not 10 rows total. Students will be confused when their preview returns 80 rows. Teach them to use `.slice(0, 10).collect()` instead.
2. **Schema Inference Crashes:**
   * *The Sin:* Using default `LazyCsvReader`.
   * *The Reality:* Polars infers schema from the first 100 rows. If row 101 has a string `"N/A"` in an integer column, the streaming engine will panic. 
   * *The Fix:* Teach them to use `.with_infer_schema_length(None)` to scan the whole file for schema, or explicitly pass a `Schema` object.
3. **The "String Allocation" Memory Leak:**
   * *The Sin:* Using `.map_elements(|s| s.as_str().unwrap().trim().to_string())` to clean strings.
   * *The Reality:* This forces Polars to drop out of its optimized C/Rust vectorized engine and into a slow, row-by-row Python-like closure, allocating a new `String` for every single row. 
   * *The Fix:* Force them to use Polars native string expressions: `col("name").str().strip_chars()`.
4. **Ignoring the "Null" Type:**
   * *The Sin:* Assuming `null_count()` returns an `Option<u32>`.
   * *The Reality:* In Polars, aggregations return Series. Extracting the scalar requires navigating the strict type system: `df.column("null_count")?.u32()?.get(0).unwrap()`. Students will fight the borrow checker here; provide a helper macro or function for scalar extraction.

---

### Addendum 5: The "Zero-Copy" Reality Check (Conceptual Discussion)
*Add this as a discussion point during **Hour 0.5 (Deconstruction)**.*

Before writing code, have the students answer this question: **"If Polars is zero-copy, why does my RAM usage still go up when I run `clean_file`?"**

**The Instructor Answer:**
Zero-copy means Polars doesn't duplicate the *underlying Apache Arrow buffers* when you filter or select columns. However, if your cleaning rules actually *modify* the data (e.g., `ImputeMissing` with `Mean`, or `TrimWhitespace`), Polars **must** allocate new memory for the new column arrays. 
*   **Takeaway for Data Engineers:** "Zero-copy" applies to routing and filtering. "Transformation" always costs memory. Design your pipelines to filter (drop rows) *before* you transform (modify columns) to keep the memory footprint as small as possible for as long as possible.