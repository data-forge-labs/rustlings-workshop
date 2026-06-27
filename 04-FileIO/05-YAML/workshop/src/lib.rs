use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub path: String,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub cron: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub database: DatabaseConfig,
    pub sources: Vec<DataSource>,
    pub schedule: Schedule,
}

pub fn parse_pipeline_config(yaml: &str) -> Result<PipelineConfig, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

pub fn write_pipeline_config(config: &PipelineConfig) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(config)
}

pub fn read_pipeline_file(path: &str) -> Result<PipelineConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&contents)?)
}

pub fn write_pipeline_file(path: &str, config: &PipelineConfig) -> Result<(), Box<dyn std::error::Error>> {
    let yaml = serde_yaml::to_string(config)?;
    fs::write(path, yaml)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobResult {
    pub job_name: String,
    pub rows_written: u64,
    pub duration_ms: u64,
    pub status: JobStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Success,
    Failed,
    Skipped,
}

pub fn parse_job_results(yaml: &str) -> Result<Vec<JobResult>, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

pub fn serialize_job_results(results: &[JobResult]) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(results)
}

pub fn merge_configs(base: &str, override_yaml: &str) -> Result<PipelineConfig, serde_yaml::Error> {
    let mut config: PipelineConfig = serde_yaml::from_str(base)?;
    let override_config: PipelineConfig = serde_yaml::from_str(override_yaml)?;
    config.database = override_config.database;
    Ok(config)
}

pub fn unique_source_names(config: &PipelineConfig) -> Vec<String> {
    let mut names: Vec<String> = config.sources.iter().map(|s| s.name.clone()).collect();
    names.sort();
    names.dedup();
    names
}

pub fn count_sources_by_format(config: &PipelineConfig, format: &str) -> usize {
    config.sources.iter().filter(|s| s.format == format).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_YAML: &str = r#"
database:
  host: db.example.com
  port: 5432
  username: etl_user
  pool_size: 20
sources:
  - name: orders
    path: /data/orders.csv
    format: csv
  - name: inventory
    path: /data/inventory.parquet
    format: parquet
schedule:
  cron: "0 2 * * *"
  enabled: true
"#;

    mod step_01_serde_derive {
        use super::*;

        #[test]
        fn test_parse_pipeline_config_returns_struct() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            assert_eq!(cfg.database.host, "db.example.com");
            assert_eq!(cfg.database.port, 5432);
            assert_eq!(cfg.database.username, "etl_user");
            assert_eq!(cfg.database.pool_size, 20);
        }

        #[test]
        fn test_parse_pipeline_config_sources() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            assert_eq!(cfg.sources.len(), 2);
            assert_eq!(cfg.sources[0].name, "orders");
            assert_eq!(cfg.sources[1].format, "parquet");
        }

        #[test]
        fn test_parse_pipeline_config_schedule() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            assert_eq!(cfg.schedule.cron, "0 2 * * *");
            assert!(cfg.schedule.enabled);
        }
    }

    mod step_02_serialize_roundtrip {
        use super::*;

        #[test]
        fn test_write_then_parse_roundtrip() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            let yaml = write_pipeline_config(&cfg).unwrap();
            let cfg2 = parse_pipeline_config(&yaml).unwrap();
            assert_eq!(cfg, cfg2);
        }

        #[test]
        fn test_serialize_contains_database_key() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            let yaml = write_pipeline_config(&cfg).unwrap();
            assert!(yaml.contains("database:"));
            assert!(yaml.contains("host:"));
        }
    }

    mod step_03_file_io {
        use super::*;

        #[test]
        fn test_read_pipeline_file() {
            let cfg = read_pipeline_file("data/pipeline.yaml").unwrap();
            assert_eq!(cfg.database.port, 5432);
            assert_eq!(cfg.sources.len(), 2);
        }

        #[test]
        fn test_read_then_write_pipeline_file() {
            let cfg = read_pipeline_file("data/pipeline.yaml").unwrap();
            let tmp = std::env::temp_dir().join("pipeline_test_roundtrip.yaml");
            write_pipeline_file(tmp.to_str().unwrap(), &cfg).unwrap();
            let cfg2 = read_pipeline_file(tmp.to_str().unwrap()).unwrap();
            assert_eq!(cfg, cfg2);
            let _ = fs::remove_file(&tmp);
        }
    }

    mod step_04_job_results {
        use super::*;

        #[test]
        fn test_parse_job_results() {
            let yaml = r#"
- job_name: extract_orders
  rows_written: 1000
  duration_ms: 250
  status: success
- job_name: transform
  rows_written: 950
  duration_ms: 100
  status: failed
"#;
            let results = parse_job_results(yaml).unwrap();
            assert_eq!(results.len(), 2);
            assert_eq!(results[0].job_name, "extract_orders");
            assert_eq!(results[0].status, JobStatus::Success);
            assert_eq!(results[1].status, JobStatus::Failed);
        }

        #[test]
        fn test_serialize_job_results_lowercase_status() {
            let results = vec![JobResult {
                job_name: "load".to_string(),
                rows_written: 500,
                duration_ms: 50,
                status: JobStatus::Success,
            }];
            let yaml = serialize_job_results(&results).unwrap();
            assert!(yaml.contains("status: success"));
            assert!(!yaml.contains("Success"));
        }
    }

    mod step_05_merge_and_query {
        use super::*;

        #[test]
        fn test_merge_configs_overrides_pool_size() {
            let base_yaml = SAMPLE_YAML;
            let override_yaml = r#"
database:
  host: db.example.com
  port: 5432
  username: etl_user
  pool_size: 50
"#;
            let merged = merge_configs(base_yaml, override_yaml).unwrap();
            assert_eq!(merged.database.pool_size, 50);
            assert_eq!(merged.sources.len(), 2);
        }

        #[test]
        fn test_unique_source_names_dedupes() {
            let yaml = r#"
database:
  host: h
  port: 1
  username: u
  pool_size: 1
sources:
  - name: orders
    path: /a
    format: csv
  - name: orders
    path: /b
    format: csv
  - name: users
    path: /c
    format: csv
schedule:
  cron: "0 0 * * *"
  enabled: true
"#;
            let cfg = parse_pipeline_config(yaml).unwrap();
            let mut names = unique_source_names(&cfg);
            names.sort();
            assert_eq!(names, vec!["orders".to_string(), "users".to_string()]);
        }

        #[test]
        fn test_count_sources_by_format() {
            let cfg = parse_pipeline_config(SAMPLE_YAML).unwrap();
            assert_eq!(count_sources_by_format(&cfg, "csv"), 1);
            assert_eq!(count_sources_by_format(&cfg, "parquet"), 1);
            assert_eq!(count_sources_by_format(&cfg, "json"), 0);
        }
    }
}
