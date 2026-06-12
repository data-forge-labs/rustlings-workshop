# Section 12: Rust Data-Engineering Analytics Engines

*Polars, DuckDB, and DataFusion — the three OLAP engines that consume Apache Arrow.*

---

## Why This Section?

The data-engineering community in 2025-2026 has converged on three high-performance Rust OLAP engines, all built on Apache Arrow. **Polars** is the de-facto single-node DataFrame library, used in production at Apple, Netflix, and Shopify. **DuckDB** is the in-process analytical database — "SQLite for analytics". **DataFusion** is the SQL engine that powers distributed query systems like Ballista and InfluxDB IOx.

In Python, the equivalents are `pandas` (Polars is 5-30x faster), `duckdb` Python bindings (also very fast), and `pyspark` / `ibis` (DataFusion's spiritual cousin). Rust's versions are faster, parallel, and zero-overhead at the FFI boundary.

**The key insight:** because all three share the Apache Arrow in-memory format, you can pass a `RecordBatch` between them with zero copies. Read a Parquet file with DataFusion, materialize a Polars DataFrame from it, query with DuckDB — all without serializing.

### The Three Engines

| Engine | Style | Best For | Python Equivalent |
|--------|-------|----------|-------------------|
| **Polars** | DataFrame API (chainable, eager + lazy) | ETL pipelines, data prep | `pandas` + `dask` |
| **DuckDB** | Embedded SQL database | Ad-hoc analytics, BI tools | `duckdb` Python, SQLite |
| **DataFusion** | Query planner + SQL executor | Custom query engines, distributed systems | `pyspark` SQL, `ibis` |

## Prerequisites

- Completed [Section 4: File I/O](../../../../../04-FileIO/README.md) — comfortable with Arrow, Parquet, CSV, JSON.
- Understands `Result` and `Box<dyn Error>`.
- Familiar with `Vec`, `HashMap`, iterators.

## Projects in This Section

| # | Project | Concepts | Tests |
|---|---------|----------|-------|
| 60 | **Polars** — single-node DataFrame | `DataFrame`, `LazyFrame`, group-by, joins, Parquet I/O | 12 |
| 61 | **DuckDB** — in-process OLAP | SQL on DataFrames, prepared statements, Arrow zero-copy | 12 |
| 62 | **DataFusion** — query planner | `SessionContext`, CSV→DataFrame, UDFs, Parquet write | 12 |

## Learning Path

1. Start with **01-Polars** for the DataFrame API (closest to pandas)
2. Move to **02-DuckDB** for SQL-driven analytics
3. Finish with **03-DataFusion** for custom query engines

## Why All Three?

A common question: "if all three use Arrow, why learn all three?" The answer is **different APIs for different use cases**:

- **Polars** is best when you want DataFrame code to read like pandas (`df.filter(...).groupby(...).agg(...)`).
- **DuckDB** is best when you have a SQL pipeline and want to drop into Rust (or call from any language) without standing up a server.
- **DataFusion** is best when you want to embed a query engine into a custom system (e.g., a query layer for a custom storage format, or a distributed query system like Ballista).

In production data-platform code, you will often see **all three used together** — Polars for ETL transforms, DuckDB for ad-hoc SQL, DataFusion for the query layer.

## Further Reading

- [Polars user guide](https://pola-rs.github.io/polars/)
- [DuckDB Rust API](https://duckdb.org/docs/api/rust.html)
- [DataFusion documentation](https://arrow.apache.org/datafusion/)
- Alex Merced, "Polars vs DataFusion" (Medium, 2025-2026)

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

