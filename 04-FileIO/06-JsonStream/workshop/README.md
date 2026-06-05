# Workshop: JSON Stream

**Goal**: Implement all functions in `src/lib.rs` to pass all 13 tests.

## Functions to Implement

### Step 1 — Basic typed parsing

#### `parse_user`
- **Signature**: `pub fn parse_user(json: &str) -> Result<User, serde_json::Error>`
- **Task**: `serde_json::from_str::<User>(json)`

#### `serialize_user`
- **Signature**: `pub fn serialize_user(user: &User) -> Result<String, serde_json::Error>`
- **Task**: `serde_json::to_string(user)` — compact, no newlines.

### Step 2 — `Value` walking

#### `parse_value`
- **Signature**: `pub fn parse_value(json: &str) -> Result<Value, serde_json::Error>`
- **Task**: `serde_json::from_str::<Value>(json)`

#### `pretty_print`
- **Signature**: `pub fn pretty_print(value: &Value) -> Result<String, serde_json::Error>`
- **Task**: `serde_json::to_string_pretty(value)`

#### `get_nested_string`
- **Signature**: `pub fn get_nested_string<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str>`
- **Task**: Walk the path, return the leaf as `&str` if it is a `Value::String`, else `None`.

#### `count_keys`
- **Signature**: `pub fn count_keys(value: &Value) -> usize`
- **Task**: If `value` is an object, return the number of top-level keys.

### Step 3 — Merge

#### `merge_values`
- **Signature**: `pub fn merge_values(a: &Value, b: &Value) -> Value`
- **Task**: Overlay `b` onto `a`. When both are objects, `b`'s keys take precedence. Otherwise return `a.clone()`.

### Step 4 — NDJSON streaming

#### `read_ndjson_users`
- **Signature**: `pub fn read_ndjson_users(path: &str) -> Result<Vec<User>, Box<dyn std::error::Error>>`
- **Task**: Read file line-by-line with `BufReader::lines`, parse each line as JSON, collect into `Vec<User>`.
- **Hint**: Skip empty lines.

#### `write_ndjson_users`
- **Signature**: `pub fn write_ndjson_users(path: &str, users: &[User]) -> Result<(), Box<dyn std::error::Error>>`
- **Task**: Open a `BufWriter<File>`, serialize each user, write each line followed by `\n`.

#### `filter_users_by_age`
- **Signature**: `pub fn filter_users_by_age(users: &[User], min_age: u32) -> Vec<User>`
- **Task**: `users.iter().filter(|u| u.age >= min_age).cloned().collect()`

### Step 5 — Pretty write to file

#### `write_pretty_json_file`
- **Signature**: `pub fn write_pretty_json_file(path: &str, value: &Value) -> Result<(), Box<dyn std::error::Error>>`
- **Task**: `fs::write(path, serde_json::to_string_pretty(value)?)`

## Structs

### `User`
- `id: u32, name: String, age: u32`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_basic_typed | 3 | `serde_json` parse/serialize of `User` |
| step_02_value_walking | 6 | `Value`, pretty-print, nested path, key count |
| step_03_merge | 1 | Overlay two JSON objects |
| step_04_ndjson_streaming | 3 | NDJSON read/write round-trip + filter |
| step_05_file_pretty_write | 1 | Pretty-write a Value to file |

## How to Run Tests
```bash
cargo test
```
