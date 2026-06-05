use argon2_workshop::{hash_password, is_password_valid, verify_password};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let password = "correct horse battery staple";

    if !is_password_valid(password, 8) {
        eprintln!("Password too short");
        std::process::exit(1);
    }

    let hash = hash_password(password)?;
    println!("Hashed: {}", &hash[..50]);
    println!("Hash length: {} chars", hash.len());

    assert!(verify_password(password, &hash)?);
    println!("Verified: OK");

    assert!(!verify_password("wrong password", &hash)?);
    println!("Wrong password rejected: OK");

    Ok(())
}
