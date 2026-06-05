use std::error::Error;
use yaml_workshop::{read_pipeline_file, write_pipeline_file, parse_pipeline_config};

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = read_pipeline_file("data/pipeline.yaml")?;
    println!(
        "Loaded pipeline: {} sources, database at {}:{}",
        cfg.sources.len(),
        cfg.database.host,
        cfg.database.port
    );

    let override_yaml = r#"
database:
  host: db.prod.example.com
  port: 5432
  username: prod_user
  pool_size: 100
"#;
    let merged = yaml_workshop::merge_configs(
        &std::fs::read_to_string("data/pipeline.yaml")?,
        override_yaml,
    )?;
    println!("Merged config: pool_size = {}", merged.database.pool_size);

    let tmp = std::env::temp_dir().join("pipeline_output.yaml");
    write_pipeline_file(tmp.to_str().unwrap(), &merged)?;
    println!("Wrote merged config to {}", tmp.display());

    let _ = parse_pipeline_config;

    Ok(())
}
