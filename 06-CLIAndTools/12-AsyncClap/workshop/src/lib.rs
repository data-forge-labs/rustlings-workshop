use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(name = "etlctl", version, about = "Async ETL pipeline CLI")]
pub struct Cli {
    #[arg(long, global = true, default_value = "info")]
    pub log_level: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Run {
        #[arg(short, long)]
        config: String,

        #[arg(long, default_value_t = 1)]
        parallelism: u8,
    },
    Etl {
        #[command(subcommand)]
        action: EtlAction,
    },
    Status {
        #[arg(short, long)]
        pipeline: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum EtlAction {
    Extract {
        #[arg(short, long)]
        source: String,
    },
    Transform {
        #[arg(short, long)]
        rule: String,
    },
    Load {
        #[arg(short, long)]
        target: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub name: String,
    pub source: String,
    pub target: String,
    pub parallelism: u8,
}

pub fn parse_args(args: &[&str]) -> Result<Cli, clap::Error> {
    todo!()
}

pub fn parse_pipeline_config(json: &str) -> Result<PipelineConfig, serde_json::Error> {
    todo!()
}

pub fn extract_target(cli: &Cli) -> Option<String> {
    todo!()
}

pub fn run_summary(cli: &Cli) -> String {
    todo!()
}

pub async fn fake_io_work(ms: u64) -> Result<String, String> {
    todo!()
}

pub async fn run_pipeline(cli: &Cli) -> Result<String, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_parse {
        use super::*;

        #[test]
        fn test_parse_run_subcommand() {
            let cli = parse_args(&["etlctl", "run", "--config", "pipe.json"]).unwrap();
            match &cli.command {
                Commands::Run { config, parallelism } => {
                    assert_eq!(config, "pipe.json");
                    assert_eq!(*parallelism, 1);
                }
                _ => panic!("expected Run"),
            }
        }

        #[test]
        fn test_parse_run_with_parallelism() {
            let cli = parse_args(&["etlctl", "run", "-c", "p.json", "--parallelism", "4"]).unwrap();
            match &cli.command {
                Commands::Run { config, parallelism } => {
                    assert_eq!(config, "p.json");
                    assert_eq!(*parallelism, 4);
                }
                _ => panic!("expected Run"),
            }
        }

        #[test]
        fn test_parse_etl_extract() {
            let cli = parse_args(&["etlctl", "etl", "extract", "--source", "s3://bucket"]).unwrap();
            match &cli.command {
                Commands::Etl { action: EtlAction::Extract { source } } => {
                    assert_eq!(source, "s3://bucket");
                }
                _ => panic!("expected Etl::Extract"),
            }
        }

        #[test]
        fn test_parse_etl_transform() {
            let cli = parse_args(&["etlctl", "etl", "transform", "--rule", "normalize"]).unwrap();
            match &cli.command {
                Commands::Etl { action: EtlAction::Transform { rule } } => {
                    assert_eq!(rule, "normalize");
                }
                _ => panic!("expected Etl::Transform"),
            }
        }

        #[test]
        fn test_parse_etl_load() {
            let cli = parse_args(&["etlctl", "etl", "load", "--target", "warehouse"]).unwrap();
            match &cli.command {
                Commands::Etl { action: EtlAction::Load { target } } => {
                    assert_eq!(target, "warehouse");
                }
                _ => panic!("expected Etl::Load"),
            }
        }

        #[test]
        fn test_parse_status() {
            let cli = parse_args(&["etlctl", "status", "-p", "my_pipeline"]).unwrap();
            assert!(matches!(cli.command, Commands::Status { .. }));
        }

        #[test]
        fn test_global_log_level_default() {
            let cli = parse_args(&["etlctl", "run", "-c", "p.json"]).unwrap();
            assert_eq!(cli.log_level, "info");
        }

        #[test]
        fn test_global_log_level_override() {
            let cli = parse_args(&["etlctl", "--log-level", "debug", "run", "-c", "p.json"]).unwrap();
            assert_eq!(cli.log_level, "debug");
        }
    }

    mod step_02_config {
        use super::*;

        #[test]
        fn test_parse_pipeline_config() {
            let json = r#"{"name":"orders","source":"s3://x","target":"warehouse","parallelism":4}"#;
            let cfg = parse_pipeline_config(json).unwrap();
            assert_eq!(cfg.name, "orders");
            assert_eq!(cfg.parallelism, 4);
        }
    }

    mod step_03_helpers {
        use super::*;

        #[test]
        fn test_extract_target_from_etl_load() {
            let cli = parse_args(&["etlctl", "etl", "load", "--target", "warehouse"]).unwrap();
            assert_eq!(extract_target(&cli), Some("warehouse".to_string()));
        }

        #[test]
        fn test_extract_target_from_run_returns_none() {
            let cli = parse_args(&["etlctl", "run", "-c", "p.json"]).unwrap();
            assert_eq!(extract_target(&cli), None);
        }

        #[test]
        fn test_run_summary_includes_command() {
            let cli = parse_args(&["etlctl", "run", "-c", "p.json"]).unwrap();
            let s = run_summary(&cli);
            assert!(s.contains("run"));
        }
    }

    mod step_04_async {
        use super::*;

        #[tokio::test]
        async fn test_fake_io_work() {
            let result = fake_io_work(10).await.unwrap();
            assert!(result.starts_with("done in "));
        }

        #[tokio::test]
        async fn test_run_pipeline_extract() {
            let cli = parse_args(&["etlctl", "etl", "extract", "--source", "s3://x"]).unwrap();
            let result = run_pipeline(&cli).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_run_pipeline_load() {
            let cli = parse_args(&["etlctl", "etl", "load", "--target", "warehouse"]).unwrap();
            let result = run_pipeline(&cli).await;
            assert!(result.is_ok());
        }
    }
}
