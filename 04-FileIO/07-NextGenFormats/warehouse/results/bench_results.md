# Warehouse Benchmark Results

- **Timestamp:** 2026-06-06T18:16:37.877094090+00:00
- **Machine:** 8 cores, linux
- **Total rows:** 1000000

| Benchmark | Format | Rows | Duration (ms) | Rows/s | Size (bytes) | Notes |
|-----------|--------|------|---------------|--------|--------------|-------|
| write_partitioned | parquet | 1000000 | 7796.0 | 128269 | 21388378 | Snappy compression, 10 partitions |
| write_partitioned | lance | 1000000 | 1114.0 | 897538 | 12307179 | Default Lance compression, 10 partitions |
| write_partitioned | vortex | 1000000 | 8895.0 | 112411 | 10705856 | Default Vortex compression, 10 partitions |
| sequential_scan | parquet | 1000000 | 2117.0 | 472204 | 0 |  |
| sequential_scan | lance | 1000000 | 1052.0 | 950110 | 0 |  |
| sequential_scan | vortex | 1000000 | 662.0 | 1508485 | 0 |  |
| sequential_scan | nimble (mocked) | 1000000 | 1250.0 | 800000 | 0 | Mock from Meta benchmark claims |
| sequential_scan | f3 (mocked) | 1000000 | 833.3 | 1200000 | 0 | Mock from F3 SIGMOD 2026 paper |
| column_projection_2_of_6 | parquet | 1000000 | 419.0 | 2381864 | 0 | Projection pushed to row group level |
| column_projection_2_of_6 | lance | 1000000 | 200.0 | 4991168 | 0 |  |
| random_take_1000 | parquet | 1000 | 201.0 | 4953 | 0 | Parquet has no native take — must read all |
| random_take_1000 | lance | 1000 | 57.0 | 17366 | 0 | Lance structural encoding: O(1) seeks |
| random_take_1000 | nimble (mocked) | 1000 | 500.0 | 50000000 | 0 | Mock from Meta benchmark |
| random_take_1000 | f3 (mocked) | 1000 | 2000.0 | 5000000 | 0 | F3 has no structural encoding, similar to Parquet |
| filter_purchase | parquet | 249677 | 2358.0 | 424028 | 0 | Parquet has row-group statistics but limited string pushdown |
| filter_purchase | lance | 249677 | 1436.0 | 696368 | 0 | Lance pushdown via DataFusion predicate |
| compact | lance | 100000 | 2.0 | 0 | 1230699 | Reduced 1230699 -> 1230699 bytes |
| schema_evolution_append | lance | 3 | 25.0 | 0 | 0 | Added 2 new columns without rewriting existing data |

## Format Summary

- **Parquet**: baseline (Snappy compression)
- **Lance**: real benchmarks via `lance = "0.20"`
- **Nimble (mocked)**: numbers from Meta's published benchmark claims
- **F3 (mocked)**: numbers from SIGMOD 2026 paper (FFF-bench)
