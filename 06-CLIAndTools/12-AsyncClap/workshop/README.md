# Workshop: Async Clap

**Goal**: Implement all functions in `src/lib.rs` to pass all 14 tests.

## Functions to Implement

### Step 1 — Parsing

#### `parse_args`
- **Signature**: `pub fn parse_args(args: &[&str]) -> Result<Cli, clap::Error>`
- **Task**: `Cli::try_parse_from(args)`

### Step 2 — Config

#### `parse_pipeline_config`
- **Signature**: `pub fn parse_pipeline_config(json: &str) -> Result<PipelineConfig, serde_json::Error>`
- **Task**: `serde_json::from_str(json)`

### Step 3 — Helpers

#### `extract_target`
- **Signature**: `pub fn extract_target(cli: &Cli) -> Option<String>`
- **Task**: Match on `Commands::Etl { action: EtlAction::Load { target } }` and return `Some(target.clone())`. All others return `None`.

#### `run_summary`
- **Signature**: `pub fn run_summary(cli: &Cli) -> String`
- **Task**: `format!("etlctl {:?}", cli.command)`.

### Step 4 — Async I/O

#### `fake_io_work`
- **Signature**: `pub async fn fake_io_work(ms: u64) -> Result<String, String>`
- **Task**: `tokio::time::sleep(Duration::from_millis(ms)).await; Ok(format!("done in {}ms", ms))`.

#### `run_pipeline`
- **Signature**: `pub async fn run_pipeline(cli: &Cli) -> Result<String, String>`
- **Task**: Match on `cli.command`:
  - `Run { config, parallelism }` → call `parse_pipeline_config` (read file), then `fake_io_work(50 * *parallelism as u64)`
  - `Etl { action: EtlAction::Extract { .. } }` → `fake_io_work(100)`
  - `Etl { action: EtlAction::Transform { .. } }` → `fake_io_work(50)`
  - `Etl { action: EtlAction::Load { .. } }` → `fake_io_work(150)`
  - `Status { .. }` → `Ok("status: running".to_string())`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_parse | 8 | Parse all subcommands + global log level |
| step_02_config | 1 | JSON config deserialization |
| step_03_helpers | 3 | extract_target + run_summary |
| step_04_async | 3 | fake_io_work + run_pipeline for two actions |

## How to Run Tests
```bash
cargo test
```
