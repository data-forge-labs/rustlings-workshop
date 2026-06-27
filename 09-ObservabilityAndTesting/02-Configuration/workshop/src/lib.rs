use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub database_url: Option<String>,
}

use config::Config;

pub fn parse_toml_config(toml_str: &str) -> Result<AppConfig, String> {
    let config = Config::builder()
        .add_source(config::File::from_str(toml_str, config::FileFormat::Toml))
        .build()
        .map_err(|e| e.to_string())?;
    config.try_deserialize().map_err(|e| e.to_string())
}

pub fn parse_json_config(json_str: &str) -> Result<AppConfig, String> {
    let config = Config::builder()
        .add_source(config::File::from_str(json_str, config::FileFormat::Json))
        .build()
        .map_err(|e| e.to_string())?;
    config.try_deserialize().map_err(|e| e.to_string())
}

pub fn parse_yaml_config(yaml_str: &str) -> Result<AppConfig, String> {
    let config = Config::builder()
        .add_source(config::File::from_str(yaml_str, config::FileFormat::Yaml))
        .build()
        .map_err(|e| e.to_string())?;
    config.try_deserialize().map_err(|e| e.to_string())
}

pub fn merge_config(
    file_config: &str,
    env_override: Option<(&str, &str)>,
) -> Result<AppConfig, String> {
    let mut builder = Config::builder()
        .add_source(config::File::from_str(file_config, config::FileFormat::Toml));

    if let Some((key, value)) = env_override {
        let map: std::collections::HashMap<String, String> =
            std::collections::HashMap::from([(key.to_string(), value.to_string())]);
        builder = builder.add_source(config::Config::try_from(&map).map_err(|e| e.to_string())?);
    }

    let config = builder.build().map_err(|e| e.to_string())?;
    config.try_deserialize().map_err(|e| e.to_string())
}

pub fn get_or_default(config: &AppConfig, key: &str) -> String {
    match key {
        "host" => config.host.clone(),
        "port" => config.port.to_string(),
        "debug" => config.debug.to_string(),
        "database_url" => config.database_url.clone().unwrap_or_default(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    mod step_01_toml {
        use crate::parse_toml_config;

        #[test]
        fn test_parse_valid_toml() {
            let toml = r#"
                host = "localhost"
                port = 8080
                debug = true
                database_url = "postgres://localhost/db"
            "#;
            let config = parse_toml_config(toml).unwrap();
            assert_eq!(config.host, "localhost");
            assert_eq!(config.port, 8080);
            assert!(config.debug);
            assert_eq!(config.database_url, Some("postgres://localhost/db".into()));
        }

        #[test]
        fn test_parse_toml_missing_field() {
            let toml = r#"host = "localhost""#;
            let result = parse_toml_config(toml);
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_toml_empty() {
            let result = parse_toml_config("");
            assert!(result.is_err());
        }
    }

    mod step_02_json_yaml {
        use crate::{parse_json_config, parse_yaml_config};

        #[test]
        fn test_parse_valid_json() {
            let json = r#"{"host": "127.0.0.1", "port": 3000, "debug": false}"#;
            let config = parse_json_config(json).unwrap();
            assert_eq!(config.host, "127.0.0.1");
            assert_eq!(config.port, 3000);
            assert!(!config.debug);
        }

        #[test]
        fn test_parse_invalid_json() {
            let result = parse_json_config("not json");
            assert!(result.is_err());
        }

        #[test]
        fn test_parse_valid_yaml() {
            let yaml = "host: example.com\nport: 443\ndebug: true\n";
            let config = parse_yaml_config(yaml).unwrap();
            assert_eq!(config.host, "example.com");
            assert_eq!(config.port, 443);
            assert!(config.debug);
        }

        #[test]
        fn test_parse_invalid_yaml() {
            let result = parse_yaml_config("");
            assert!(result.is_err());
        }
    }

    mod step_03_merge {
        use crate::merge_config;

        #[test]
        fn test_merge_env_override() {
            let file_config = r#"host = "localhost" port = 8080 debug = false"#;
            let config =
                merge_config(file_config, Some(("debug", "true"))).unwrap();
            assert!(config.debug);
        }

        #[test]
        fn test_merge_file_only() {
            let file_config =
                r#"host = "localhost" port = 8080 debug = false"#;
            let config = merge_config(file_config, None).unwrap();
            assert_eq!(config.host, "localhost");
            assert!(!config.debug);
        }

        #[test]
        fn test_merge_invalid_file() {
            let result = merge_config("", None);
            assert!(result.is_err());
        }
    }

    mod step_04_defaults {
        use crate::{get_or_default, AppConfig};

        fn sample_config() -> AppConfig {
            AppConfig {
                host: "localhost".into(),
                port: 8080,
                debug: true,
                database_url: None,
            }
        }

        #[test]
        fn test_get_existing_key() {
            let config = sample_config();
            assert_eq!(get_or_default(&config, "host"), "localhost");
        }

        #[test]
        fn test_get_missing_key_with_fallback() {
            let config = sample_config();
            // Non-existent key should return a default/fallback representation
            let result = get_or_default(&config, "nonexistent");
            assert_eq!(result, "");
        }
    }
}
