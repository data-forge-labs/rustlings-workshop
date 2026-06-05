use axum_auth_workshop::{create_access_token, has_role, is_expired, verify_token, Claims};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let secret = b"demo-secret-key-for-cli-1234567890ab";
    let token = create_access_token("alice", "admin", secret, 3600)?;
    println!("Access token: {}", token);

    let claims: Claims = verify_token(&token, secret)?;
    println!("Decoded subject: {}", claims.sub);
    println!("Decoded role:    {}", claims.role);
    println!("Expired?         {}", is_expired(&claims));
    println!("Has admin role?  {}", has_role(&claims, &["admin", "user"]));

    Ok(())
}
