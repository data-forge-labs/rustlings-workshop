# Workshop: YAML

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.

## Functions to Implement

### Step 1 — `serde_yaml` + `serde::Deserialize`

#### `parse_pipeline_config`
- **Signature**: `pub fn parse_pipeline_config(yaml: &str) -> Result<PipelineConfig, serde_yaml::Error>`
- **Task**: Use `serde_yaml::from_str` to deserialize YAML into a `PipelineConfig`.

### Step 2 — Serialization round-trip

#### `write_pipeline_config`
- **Signature**: `pub fn write_pipeline_config(config: &PipelineConfig) -> Result<String, serde_yaml::Error>`
- **Task**: Use `serde_yaml::to_string` to serialize back to YAML.

### Step 3 — File I/O

#### `read_pipeline_file`
- **Signature**: `pub fn read_pipeline_file(path: &str) -> Result<PipelineConfig, Box<dyn std::error::Error>>`
- **Task**: `fs::read_to_string` then `parse_pipeline_config`.

#### `write_pipeline_file`
- **Signature**: `pub fn write_pipeline_file(path: &str, config: &PipelineConfig) -> Result<(), Box<dyn std::error::Error>>`
- **Task**: Serialize then `fs::write`.

### Step 4 — Custom enums

#### `parse_job_results` / `serialize_job_results`
- **Task**: Use `#[serde(rename_all = "lowercase")]` on `JobStatus` so the YAML keys are `success`, `failed`, `skipped`.

### Step 5 — Merge and query

#### `merge_configs`
- **Signature**: `pub fn merge_configs(base: &str, override_yaml: &str) -> Result<PipelineConfig, serde_yaml::Error>`
- **Task**: Parse both, then build a new `PipelineConfig` taking the override's `database` and the base's `sources` and `schedule`.

#### `unique_source_names`
- **Signature**: `pub fn unique_source_names(config: &PipelineConfig) -> Vec<String>`
- **Task**: Use a `BTreeSet<String>` to dedupe and return a sorted `Vec<String>`.

#### `count_sources_by_format`
- **Signature**: `pub fn count_sources_by_format(config: &PipelineConfig, format: &str) -> usize`
- **Task**: Count sources whose `format` field matches.

## Structs

### `PipelineConfig`
- `database: DatabaseConfig`
- `sources: Vec<DataSource>`
- `schedule: Schedule`

### `DatabaseConfig`
- `host: String, port: u16, username: String, pool_size: u32`

### `DataSource`
- `name: String, path: String, format: String`

### `Schedule`
- `cron: String, enabled: bool`

### `JobResult` / `JobStatus`
- `JobResult { job_name, rows_written, duration_ms, status }`
- `JobStatus`: enum with `Success`, `Failed`, `Skipped` (serialized lowercase)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_serde_derive | 3 | Parse nested PipelineConfig from YAML |
| step_02_serialize_roundtrip | 2 | Serialize → parse returns same value |
| step_03_file_io | 2 | Read/write YAML to disk |
| step_04_job_results | 2 | Custom enum with rename_all = "lowercase" |
| step_05_merge_and_query | 3 | Merge configs, dedupe names, count by format |

## How to Run Tests
```bash
cargo test
```
