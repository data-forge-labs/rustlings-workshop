# Workshop: Polars

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### Step 1 — Load CSV

#### `load_sales_csv`
- **Signature**: `pub fn load_sales_csv(path: &str) -> Result<DataFrame, PolarsError>`
- **Task**: `CsvReader::from_path(path)?.has_header(true).finish()`

### Step 2 — Aggregations

#### `total_units`
- **Signature**: `pub fn total_units(sales: &DataFrame) -> Result<i64, PolarsError>`
- **Task**: `sales.column("units")?.sum::<i64>()`

#### `total_revenue`
- **Signature**: `pub fn total_revenue(sales: &DataFrame) -> Result<f64, PolarsError>`
- **Task**: `sales.lazy().select([(col("amount") * col("units")).sum().alias("revenue")]).collect()?.column("revenue")?.f64()?.get(0).unwrap_or(0.0)`

### Step 3 — Filter

#### `filter_expensive`
- **Signature**: `pub fn filter_expensive(sales: &DataFrame, min_amount: f64) -> Result<DataFrame, PolarsError>`
- **Task**: `sales.clone().lazy().filter(col("amount").gt_eq(lit(min_amount))).collect()`

### Step 4 — Group-by

#### `revenue_per_product`
- **Signature**: `pub fn revenue_per_product(sales: &DataFrame) -> Result<DataFrame, PolarsError>`
- **Task**: `sales.lazy().group_by([col("name")]).agg([(col("amount") * col("units")).sum().alias("revenue")]).sort("revenue", SortOptions::default().with_order_desc(true)).collect()`

#### `high_revenue_products`
- **Signature**: `pub fn high_revenue_products(sales: &DataFrame, min_revenue: f64) -> Result<DataFrame, PolarsError>`
- **Task**: First compute revenue per product, then filter rows where revenue >= min_revenue.

### Step 5 — Parquet I/O

#### `write_parquet` / `read_parquet`
- **Task**: `ParquetWriter::new(File::create(path)?).finish(df)` and `ParquetReader::new(File::open(path)?).finish()`.

### Step 6 — Lazy

#### `lazy_filter_expensive`
- **Signature**: `pub fn lazy_filter_expensive(min_amount: f64) -> Result<DataFrame, PolarsError>`
- **Task**: Read CSV via `LazyFrame::scan_csv`, filter, collect.

#### `lazy_group_by_total`
- **Signature**: `pub fn lazy_group_by_total() -> Result<DataFrame, PolarsError>`
- **Task**: Read CSV via `LazyCsvReader`, group by `name`, sum `units`, collect.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_load | 2 | CSV → DataFrame shape and column names |
| step_02_aggregations | 2 | Total units and total revenue |
| step_03_filter_select | 1 | Filter rows by amount |
| step_04_group_by | 2 | Group-by and threshold filter |
| step_05_parquet | 1 | Parquet roundtrip |
| step_06_lazy | 2 | LazyFrame filter and group-by |

## How to Run Tests
```bash
cargo test
```
