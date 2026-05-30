# Workshop: Configuration

**Goal**: Implement all 4 functions in `src/lib.rs` to pass all 12 tests.

## Functions to Implement

### `parse_toml_config`
- **Signature**: `pub fn parse_toml_config(toml_str: &str) -> Result<AppConfig, String>`
- **Task**: Parse TOML string into `AppConfig` using `toml` crate; return `Err` on missing fields or invalid input.
- **Tests**: test_parse_valid_toml, test_parse_toml_missing_field, test_parse_toml_empty

### `parse_json_config`
- **Signature**: `pub fn parse_json_config(json_str: &str) -> Result<AppConfig, String>`
- **Task**: Parse JSON string into `AppConfig` using `serde_json`.
- **Tests**: test_parse_valid_json, test_parse_invalid_json

### `parse_yaml_config`
- **Signature**: `pub fn parse_yaml_config(yaml_str: &str) -> Result<AppConfig, String>`
- **Task**: Parse YAML string into `AppConfig` using `serde_yaml`.
- **Tests**: test_parse_valid_yaml, test_parse_invalid_yaml

### `merge_config`
- **Signature**: `pub fn merge_config(file_config: &str, env_override: Option<(&str, &str)>) -> Result<AppConfig, String>`
- **Task**: Parse file config as TOML, then apply an optional env override (key=value pair) to overwrite fields.
- **Tests**: test_merge_env_override, test_merge_file_only, test_merge_invalid_file

### `get_or_default`
- **Signature**: `pub fn get_or_default(config: &AppConfig, key: &str) -> String`
- **Task**: Return the config value for the given key as a string, or empty string if the key doesn't exist.
- **Tests**: test_get_existing_key, test_get_missing_key_with_fallback

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_toml | 3 | TOML parsing with valid/missing/empty input |
| step_02_json_yaml | 4 | JSON and YAML parsing |
| step_03_merge | 3 | Merge file config with env overrides |
| step_04_defaults | 2 | Get-or-default fallback logic |

## How to Run Tests
```bash
cargo test
```
