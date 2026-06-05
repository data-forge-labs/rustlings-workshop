fn main() {
    println!("Rust Configuration — Python configparser/pydantic equivalent");

    let toml_cfg = rust_configuration::parse_toml_config(
        r#"host = "localhost" port = 8080 debug = true"#,
    );
    println!("TOML config: {:?}", toml_cfg);

    let json_cfg = rust_configuration::parse_json_config(
        r#"{"host":"localhost","port":8080,"debug":true}"#,
    );
    println!("JSON config: {:?}", json_cfg);

    let merged = rust_configuration::merge_config(
        r#"host = "localhost" port = 8080 debug = false"#,
        Some(("debug", "true")),
    );
    println!("Merged config (env overrides file): {:?}", merged);
}
