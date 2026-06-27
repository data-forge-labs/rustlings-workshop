use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

pub fn parse_user(json: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn serialize_user(user: &User) -> Result<String, serde_json::Error> {
    serde_json::to_string(user)
}

pub fn parse_value(json: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn pretty_print(value: &Value) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

pub fn get_nested_string<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(key)?;
    }
    current.as_str()
}

pub fn read_ndjson_users(path: &str) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let mut users = Vec::new();
    for line in contents.lines() {
        if !line.trim().is_empty() {
            users.push(serde_json::from_str(line)?);
        }
    }
    Ok(users)
}

pub fn write_ndjson_users(path: &str, users: &[User]) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = users.iter().map(|u| serde_json::to_string(u)).collect::<Result<_, _>>()?;
    fs::write(path, lines.join("\n"))?;
    Ok(())
}

pub fn write_pretty_json_file(path: &str, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn count_keys(value: &Value) -> usize {
    match value {
        Value::Object(map) => map.len(),
        _ => 0,
    }
}

pub fn merge_values(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            let mut merged = a_map.clone();
            for (k, v) in b_map {
                merged.insert(k.clone(), v.clone());
            }
            Value::Object(merged)
        }
        _ => b.clone(),
    }
}

pub fn filter_users_by_age(users: &[User], min_age: u32) -> Vec<User> {
    users.iter().filter(|u| u.age >= min_age).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{"id":1,"name":"Alice","age":30}"#;
    const NESTED: &str = r#"{"user":{"profile":{"name":"Alice"}}}"#;

    mod step_01_basic_typed {
        use super::*;

        #[test]
        fn test_parse_user() {
            let u = parse_user(SAMPLE).unwrap();
            assert_eq!(u.id, 1);
            assert_eq!(u.name, "Alice");
            assert_eq!(u.age, 30);
        }

        #[test]
        fn test_serialize_user() {
            let u = User { id: 7, name: "Grace".into(), age: 40 };
            let s = serialize_user(&u).unwrap();
            assert!(s.contains("\"id\":7"));
            assert!(s.contains("\"Grace\""));
        }

        #[test]
        fn test_serialize_user_is_compact() {
            let u = User { id: 1, name: "x".into(), age: 1 };
            let s = serialize_user(&u).unwrap();
            assert!(!s.contains('\n'), "compact JSON should not contain newlines");
        }
    }

    mod step_02_value_walking {
        use super::*;

        #[test]
        fn test_parse_value_returns_json_value() {
            let v = parse_value(SAMPLE).unwrap();
            assert_eq!(v["name"], "Alice");
            assert_eq!(v["age"], 30);
        }

        #[test]
        fn test_pretty_print_indents() {
            let v = parse_value(SAMPLE).unwrap();
            let s = pretty_print(&v).unwrap();
            assert!(s.contains('\n'), "pretty JSON should contain newlines");
            assert!(s.contains("  "), "pretty JSON should contain indentation");
        }

        #[test]
        fn test_get_nested_string_returns_leaf() {
            let v = parse_value(NESTED).unwrap();
            assert_eq!(get_nested_string(&v, &["user", "profile", "name"]), Some("Alice"));
        }

        #[test]
        fn test_get_nested_string_missing_path_is_none() {
            let v = parse_value(SAMPLE).unwrap();
            assert_eq!(get_nested_string(&v, &["user", "profile", "name"]), None);
        }

        #[test]
        fn test_count_keys_top_level() {
            let v = parse_value(SAMPLE).unwrap();
            assert_eq!(count_keys(&v), 3);
        }

        #[test]
        fn test_count_keys_nested() {
            let v = parse_value(NESTED).unwrap();
            assert_eq!(count_keys(&v), 1);
        }
    }

    mod step_03_merge {
        use super::*;

        #[test]
        fn test_merge_values_object_overlay() {
            let a = parse_value(r#"{"a":1,"b":2}"#).unwrap();
            let b = parse_value(r#"{"b":99,"c":3}"#).unwrap();
            let merged = merge_values(&a, &b);
            assert_eq!(merged["a"], 1);
            assert_eq!(merged["b"], 99);
            assert_eq!(merged["c"], 3);
        }
    }

    mod step_04_ndjson_streaming {
        use super::*;

        #[test]
        fn test_read_ndjson_users() {
            let users = read_ndjson_users("data/users.ndjson").unwrap();
            assert_eq!(users.len(), 5);
            assert_eq!(users[0].name, "Alice");
            assert_eq!(users[4].age, 42);
        }

        #[test]
        fn test_write_then_read_ndjson_roundtrip() {
            let users = vec![
                User { id: 1, name: "Zed".into(), age: 99 },
                User { id: 2, name: "Yara".into(), age: 88 },
            ];
            let tmp = std::env::temp_dir().join("users_test_roundtrip.ndjson");
            write_ndjson_users(tmp.to_str().unwrap(), &users).unwrap();
            let read_back = read_ndjson_users(tmp.to_str().unwrap()).unwrap();
            assert_eq!(read_back, users);
            let _ = fs::remove_file(&tmp);
        }

        #[test]
        fn test_filter_users_by_age() {
            let users = read_ndjson_users("data/users.ndjson").unwrap();
            let older = filter_users_by_age(&users, 30);
            assert_eq!(older.len(), 3);
            assert!(older.iter().all(|u| u.age >= 30));
        }
    }

    mod step_05_file_pretty_write {
        use super::*;

        #[test]
        fn test_write_pretty_json_file() {
            let v = parse_value(SAMPLE).unwrap();
            let tmp = std::env::temp_dir().join("user_pretty_test.json");
            write_pretty_json_file(tmp.to_str().unwrap(), &v).unwrap();
            let contents = fs::read_to_string(&tmp).unwrap();
            assert!(contents.contains('\n'));
            assert!(contents.contains("Alice"));
            let _ = fs::remove_file(&tmp);
        }
    }
}
