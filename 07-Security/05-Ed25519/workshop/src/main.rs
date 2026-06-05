use ed25519_workshop::{
    generate_keypair, public_key_from_hex, public_key_to_hex, sign_message, verify_signature,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let key = generate_keypair();
    let message = b"this is a signed message";

    let signature = sign_message(&key, message);
    println!("Signed message ({} bytes): OK", signature.to_bytes().len());

    let hex = public_key_to_hex(&key.verifying_key());
    println!("Public key (hex): {}", &hex[..16]);

    let parsed = public_key_from_hex(&hex)?;
    assert_eq!(key.verifying_key().to_bytes(), parsed.to_bytes());
    println!("Hex roundtrip: OK");

    assert!(verify_signature(&key.verifying_key(), message, &signature));
    println!("Verify correct message: OK");

    assert!(!verify_signature(&key.verifying_key(), b"tampered", &signature));
    println!("Verify tampered message rejected: OK");

    Ok(())
}
