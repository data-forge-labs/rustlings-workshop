# 14 — Data Infrastructure & Integration

> *Build production data pipelines in Rust: PostgreSQL → Kafka/CDC → ClickHouse/DuckLake, with Redis caching, Apache Iggy, and a unified fan-out orchestrator.*

This section turns the language skills from Sections 01–13 into a
**complete, runnable data infrastructure stack**. Every project
ships as a `cargo new --lib` workshop with progressive tests, a
working `docker-compose.yml` to bring up the dependencies, and a
`main.rs` that runs end-to-end.

## Why a data-infrastructure section?

A Python data engineer who learns Rust for "general use" still
needs a guided tour of the *data-side* ecosystem: which crate
plays the role of `psycopg2`? of `confluent-kafka`? of
`clickhouse-driver`? This section answers those questions with
**test-driven workshops** that you can run on your laptop.

## Architecture

```
                    ┌──────────────────────────────────────┐
                    │         Source: PostgreSQL 16        │
                    │  (orders, outbox, events schemas)    │
                    └─────────────────┬────────────────────┘
                                      │ logical replication
                                      ▼
┌─────────────────────────────────────────────────────────────┐
│  Project 07 — CDC Pipeline (Debezium-style envelope)        │
│  Sink trait + leader claim + checkpoint + batching          │
└─────────────────┬───────────────────────────────────────────┘
                  │ PipelineEvent
                  ▼
┌─────────────────────────────────────────────────────────────┐
│  Project 08 — Unified Pipeline Orchestrator                 │
│  fan-out to N sinks, retry, dead-letter, live counters      │
└──┬──────────┬─────────────┬─────────────┬─────────────┬─────┘
   │          │             │             │             │
   ▼          ▼             ▼             ▼             ▼
┌──────┐ ┌──────────┐ ┌──────────────┐ ┌──────────┐ ┌────────┐
│Kafka │ │ClickHouse│ │ DuckLake     │ │Redis     │ │Apache  │
│01    │ │04        │ │ 06 (SQL+     │ │03 (cache │ │Iggy 05 │
│      │ │(OLAP)    │ │  Parquet)    │ │+ streams)│ │(alt.   │
│      │ │          │ │              │ │          │ │broker) │
└──────┘ └──────────┘ └──────────────┘ └──────────┘ └────────┘
```

## Quick Start

```bash
# 1. Bring up the full data infrastructure stack
docker compose up -d

# 2. Wait for healthchecks
docker compose ps

# 3. Run a workshop (Wave 1)
cargo run --release --manifest-path 01-KafkaRdkafka/workshop/Cargo.toml

# 4. Run all tests
for d in */workshop; do (cd "$d" && cargo test --quiet); done
```

Services (ports exposed to host):

| Service        | Port  | URL                           | Healthcheck                |
|----------------|-------|-------------------------------|----------------------------|
| PostgreSQL     | 5432  | `postgres://dataeng:dataeng@localhost:5432/dataeng` | `pg_isready`             |
| Kafka          | 9092  | `localhost:9092`              | `kafka-topics --list`      |
| Kafka UI       | 8080  | http://localhost:8080         | n/a                        |
| Redis          | 6379  | `redis://localhost:6379/0`    | `redis-cli ping`           |
| ClickHouse     | 8123  | http://localhost:8123         | `SELECT 1`                 |
| ClickHouse TCP | 9000  | `tcp://localhost:9000`        | n/a                        |
| Iggy TCP       | 3000  | `iggy://localhost:3000`       | n/a                        |
| Iggy HTTP      | 8090  | http://localhost:8090         | `/api/v1/health`           |
| MinIO S3       | 9000  | http://localhost:9000         | `mc ready local`           |
| MinIO Console  | 9001  | http://localhost:9001         | n/a (admin UI, root/minioadmin) |

Note: MinIO uses host port 9000 and ClickHouse native uses 9000
internally — they coexist because the ports are bound to different
container interfaces. If you only need one, the rest can be turned
off with `docker compose stop <service>`.

## Projects (Wave 1: Foundations)

| # | Project | Crate(s) | Real-world counterpart | What you build |
|---|---------|----------|------------------------|----------------|
| 01 | **KafkaRdkafka** | `rdkafka`, `tokio` | `confluent-kafka-python` | Idempotent producer, manual-commit consumer, dedup cache |
| 02 | **PostgreSQLSqlx** | `sqlx`, `tokio` | `psycopg2` + SQLAlchemy | Transactional outbox, batcher, retry policy |
| 03 | **RedisAsync** | `redis`, `tokio` | `redis-py` async | Cache + streams + consumer groups + SETNX idempotency |
| 04 | **ClickHouseIngestion** | `clickhouse`, `tokio` | `clickhouse-driver` | Batched columnar inserts, retry, minute-bucket projection |

## Projects (Wave 2: Composition)

| # | Project | Crate(s) | Real-world counterpart | What you build |
|---|---------|----------|------------------------|----------------|
| 05 | **ApacheIggy** | `iggy` | n/a (Rust-native broker) | Thread-per-core producer/consumer, FNV-1a partitioner |
| 06 | **DuckLakeCatalog** | `duckdb` | PyIceberg / Delta Lake | SQL catalog + Parquet + time-travel + merge-upsert |
| 07 | **CdcPipeline** | `tokio`, `async-trait` | Debezium + Kafka Connect | Envelope shape, `Sink` trait, leader claim, checkpoint |
| 08 | **UnifiedPipeline** | `tokio`, `async-trait` | Airbyte / Prefect / Dagster | Multi-sink fan-out, retry, dead-letter, live counters |

## Learning Path

```
Wave 1 (build the layers):
  01 KafkaRdkafka       ──┐
  02 PostgreSQLSqlx     ──┤
  03 RedisAsync         ──┼── Each is independent, in any order
  04 ClickHouseIngestion──┘

Wave 2 (compose them):
  05 ApacheIggy        ── Alternative to 01
  06 DuckLakeCatalog   ── Alternative to 04
  07 CdcPipeline       ── Composes 02 as a source
  08 UnifiedPipeline   ── Composes 01/03/04/05/06 as sinks
```

After Wave 1 you have a *complete data pipeline* using
PostgreSQL → Kafka → ClickHouse with Redis caching. Wave 2 adds
the alternative broker (Iggy) and the alternative lake
(DuckLake), then the CDC pipeline and the unified orchestrator.

## Why these specific technologies?

- **PostgreSQL** — the default OLTP database in every company
  we've worked at. Transactional outbox is the canonical
  "exactly-once" pattern.
- **Kafka** — the default event backbone. rdkafka is the
  fastest, lowest-overhead client (it's the librdkafka C
  library that `confluent-kafka-python` itself wraps).
- **Redis** — the default cache. Streams give us a
  "Redis-only mini-Kafka" for low-volume cases.
- **ClickHouse** — the fastest open-source OLAP. The Rust
  client (`clickhouse-rs`) is async and well-maintained.
- **Apache Iggy** — the new kid: Rust-native, thread-per-core,
  zero-JVM. A real alternative for teams that hate JVMs.
- **DuckLake** — DuckDB's lakehouse protocol. SQL catalog +
  Parquet files; no JVM, no service, no Spark.
- **Debezium-style CDC** — the canonical "Postgres → Kafka"
  pipeline. We model the envelope shape; integrating with a
  real Debezium cluster is one config file away.
- **Unified orchestrator** — the lesson is: every team
  eventually writes this. Now you have a reference
  implementation.

## Python Pain → Rust Fix (one-line summary)

| Pain | Fix |
|------|-----|
| `confluent-kafka-python` redeliveries cause duplicate work | `DedupCache` FIFO id cache |
| SQLAlchemy typos crash at 3 AM | sqlx `query!` macro is compile-time-checked |
| Redis sync client blocks event loop | `ConnectionManager` is async-multiplexed |
| `clickhouse-driver` is per-row by default | `IngestBatcher` flushes by row count *or* bytes |
| Apache Iggy has no Python client | `iggy` crate is first-class |
| Iceberg/Delta need Spark | DuckLake is a `duckdb` extension |
| Debezium runs on Kafka Connect (JVM) | Our `Sink` trait works against any async backend |
| Every team writes a fan-out loop | This section's `PipelineConfig` + `Sink` is reusable |

## Further Reading

- [Apache Kafka docs](https://kafka.apache.org/documentation/) — broker reference
- [ClickHouse docs](https://clickhouse.com/docs/en/) — MergeTree + Kafka engine
- [Apache Iggy (incubating)](https://github.com/apache/iggy) — Rust-native broker
- [DuckLake announcement](https://duckdb.org/2025/05/13/announcing-ducklake.html) — design rationale
- [Debezium documentation](https://debezium.io/documentation/reference/stable/index.html) — CDC
- [PostgreSQL logical replication](https://www.postgresql.org/docs/current/logical-replication.html) — `wal2json` / `pgoutput`
