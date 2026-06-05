use std::error::Error;
use json_stream::{
    filter_users_by_age, get_nested_string, parse_user, parse_value, read_ndjson_users,
    serialize_user, write_pretty_json_file, User,
};

fn main() -> Result<(), Box<dyn Error>> {
    let users = read_ndjson_users("data/users.ndjson")?;
    println!("Read {} users from NDJSON", users.len());

    let older = filter_users_by_age(&users, 30);
    println!("Users aged 30+: {}", older.len());

    let json = serialize_user(&users[0])?;
    println!("First user JSON: {}", json);

    let parsed = parse_user(&json)?;
    println!("Re-parsed: {} (age {})", parsed.name, parsed.age);

    let nested = parse_value(r#"{"user":{"profile":{"name":"Alice"}}}"#)?;
    println!(
        "Nested name: {:?}",
        get_nested_string(&nested, &["user", "profile", "name"])
    );

    let tmp = std::env::temp_dir().join("demo_user.json");
    write_pretty_json_file(tmp.to_str().unwrap(), &nested)?;
    println!("Wrote pretty JSON to {}", tmp.display());

    Ok(())
}
