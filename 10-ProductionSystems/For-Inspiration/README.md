# For Inspiration — Production-Ready Rust Data Engineering

> **Curated list of modular, production-ready Rust repositories** for deepening your data engineering skills. These are not tutorials — they are reference implementations you can study, fork, or embed in your own systems.

---

## 🌟 Tansu — Apache Kafka® Compatible Broker with Data Lake Integration

> **Repository:** [https://github.com/tansu-io/tansu](https://github.com/tansu-io/tansu)

### What is Tansu?

Tansu is a **drop-in replacement for Apache Kafka** with PostgreSQL, libSQL (SQLite), S3, or memory storage engines. Schema-backed topics (Avro, JSON, or Protocol Buffers) can be written as **Apache Iceberg** or **Delta Lake** tables.

> **Why it matters for data engineers:** Tansu demonstrates how to build a production-grade, Kafka-compatible event streaming system entirely in Rust — with zero JVM dependencies, built-in schema registry, and native data lake integration.

---

### Key Features

| Feature | Description |
|---------|-------------|
| **Apache Kafka API Compatible** | Drop-in replacement for existing Kafka producers/consumers |
| **Multiple Storage Engines** | PostgreSQL, libSQL (SQLite), S3, or in-memory |
| **Schema Validation** | JSON Schema, Apache Avro, or Protocol Buffers per topic |
| **Data Lake Integration** | Topics written as Apache Iceberg or Delta Lake tables (Apache Parquet) |
| **Single Binary** | Broker, schema registry, topic CLI, cat CLI, and proxy in one statically-linked binary |

### Storage Engines & Durability

| Engine | Durability | Use Case |
|--------|------------|----------|
| **S3** | 99.999999999% (11 nines) | Production data lake |
| **PostgreSQL** | Continuous archiving (WAL streaming) | Transactional durability |
| **libSQL (SQLite)** | Local file durability | Edge / embedded deployments |
| **Memory** | Ephemeral | Development / testing only |

### Architecture (Single Binary)

```
tansu (single statically-linked binary)
├── broker    # Kafka-compatible broker + schema registry
├── topic     # CLI: create/delete topics
├── cat       # CLI: consume/produce Avro/JSON/Protobuf messages
├── proxy     # Kafka-compatible proxy
```

### CLI Commands

```bash
# Start broker (default command)
tansu broker

# Create a topic
tansu topic create --topic orders --partitions 4 --replication 1

# Produce Avro/JSON/Protobuf messages
tansu cat produce --topic orders --format avro < data.avro

# Consume messages
tansu cat consume --topic orders --format json

# Run proxy
tansu proxy
```

### Configuration (Environment Variables)

| Variable | Default | Description |
|----------|---------|-------------|
| `CLUSTER_ID` | `tansu_cluster` | All cluster members use same ID |
| `LISTENER_URL` | `tcp://[::]:9092` | Broker listen address |
| `ADVERTISED_LISTENER_URL` | `tcp://localhost:9092` | Advertised to clients |
| `STORAGE_ENGINE` | `memory://tansu/` | `postgres://...`, `s3://...`, `memory://...` |
| `SCHEMA_REGISTRY` | `file://./etc/schema` | Schema registry location |
| `DATA_LAKE` | `s3://lake/` | Parquet output location |
| `ICEBERG_CATALOG` | `http://localhost:8181/` | Iceberg catalog endpoint |
| `ICEBERG_NAMESPACE` | `tansu` | Iceberg namespace |
| `PROMETHEUS_LISTENER_URL` | `tcp://[::]:9100` | Metrics endpoint |

### Example: Local Development with PostgreSQL + Schema Registry

```bash
# .env file
CLUSTER_ID=my-cluster
LISTENER_URL=tcp://0.0.0.0:9092
ADVERTISED_LISTENER_URL=tcp://localhost:9092
STORAGE_ENGINE=postgres://postgres:postgres@localhost:5432/tansu
SCHEMA_REGISTRY=file://./etc/schema
DATA_LAKE=file://./lake
ICEBERG_CATALOG=http://localhost:8181/
ICEBERG_NAMESPACE=tansu
PROMETHEUS_LISTENER_URL=tcp://0.0.0.0:9100

# Start Tansu
tansu broker
```

### Why Study Tansu for Rust Data Engineering?

| Aspect | What You Learn |
|--------|----------------|
| **Protocol Implementation** | Full Kafka protocol (RESP-like wire format) in pure Rust |
| **Async Runtime** | `tokio` with work-stealing for high-throughput networking |
| **Schema Registry** | Avro/Protobuf/JSON schema validation at ingest |
| **Storage Abstraction** | Trait-based storage engines (PostgreSQL, S3, SQLite, memory) |
| **Data Lake Integration** | Writing Parquet + Iceberg/Delta Lake metadata |
| **Schema Validation** | Rejecting invalid messages at broker level |
| **CLI Design** | `clap` derive for subcommand-heavy CLI |
| **Observability** | Prometheus metrics, structured tracing |

### Related Examples (from Tansu Repo)

| Example | Description |
|---------|-------------|
| [pyiceberg](https://github.com/tansu-io/tansu/tree/main/examples/pyiceberg) | Read Tansu data with pyiceberg |
| [Apache Spark](https://github.com/tansu-io/tansu/tree/main/examples/spark) | Query Iceberg tables with Spark |
| [Delta Lake](https://github.com/tansu-io/tansu/tree/main/examples/deltalake) | Delta Lake integration |

---

## How to Use This Section

1. **Clone & Explore** — `git clone https://github.com/tansu-io/tansu` and read the broker/src code
2. **Run Locally** — Start with `memory://` storage, then switch to PostgreSQL/S3
2. **Study the Architecture** — Focus on:
   - `broker/src/` — Core broker logic
   - `storage/` — Storage engine traits and implementations
   - `schema/` — Schema registry & validation
   - `lake/` — Iceberg/Delta Lake writers
4. **Experiment** — Swap storage engines, add custom schema types, extend the proxy

---

## Next Inspiration Targets

> This section will grow with more production-ready Rust data engineering repositories.

| Repository | Focus Area | Status |
|------------|------------|--------|
| **Tansu** | Kafka-compatible broker + data lake | ✅ Added |
| *Your suggestion* | *Topic* | 🔲 |

> **Want to contribute?** Submit a PR adding another production-ready Rust data engineering project with the same template structure.