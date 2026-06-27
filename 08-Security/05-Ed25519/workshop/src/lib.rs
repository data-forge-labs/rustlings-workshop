use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub fn generate_keypair() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

pub fn sign_message(key: &SigningKey, message: &[u8]) -> Signature {
    key.sign(message)
}

pub fn verify_signature(key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
    key.verify(message, signature).is_ok()
}

pub fn public_key_to_hex(key: &VerifyingKey) -> String {
    hex::encode(key.to_bytes())
}

pub fn public_key_from_hex(s: &str) -> Result<VerifyingKey, ed25519_dalek::SignatureError> {
    let bytes = hex::decode(s).map_err(|_| ed25519_dalek::SignatureError::from_source("invalid hex"))?;
    VerifyingKey::from_bytes(&bytes.try_into().map_err(|_| ed25519_dalek::SignatureError::from_source("invalid length"))?)
}

pub fn sign_then_verify(message: &[u8]) -> bool {
    let key = generate_keypair();
    let sig = sign_message(&key, message);
    verify_signature(&key.verifying_key(), message, &sig)
}

pub fn tampered_signature_fails(message: &[u8]) -> bool {
    let key = generate_keypair();
    let sig = sign_message(&key, message);
    let tampered_bytes = sig.to_bytes();
    let mut modified = tampered_bytes;
    modified[0] ^= 0xff;
    let tampered_sig = Signature::from_bytes(&modified);
    verify_signature(&key.verifying_key(), message, &tampered_sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_keypair {
        use super::*;

        #[test]
        fn test_generate_keypair() {
            let key = generate_keypair();
            assert_eq!(key.to_bytes().len(), 32);
        }

        #[test]
        fn test_two_keypairs_differ() {
            let k1 = generate_keypair();
            let k2 = generate_keypair();
            assert_ne!(k1.to_bytes(), k2.to_bytes());
        }
    }

    mod step_02_sign_and_verify {
        use super::*;

        #[test]
        fn test_sign_and_verify_roundtrip() {
            let key = generate_keypair();
            let msg = b"hello, ed25519";
            let sig = sign_message(&key, msg);
            assert!(verify_signature(&key.verifying_key(), msg, &sig));
        }

        #[test]
        fn test_verify_wrong_message_fails() {
            let key = generate_keypair();
            let sig = sign_message(&key, b"original");
            assert!(!verify_signature(&key.verifying_key(), b"tampered", &sig));
        }

        #[test]
        fn test_sign_then_verify_helper() {
            assert!(sign_then_verify(b"a message"));
        }

        #[test]
        fn test_tampered_signature_fails() {
            assert!(!tampered_signature_fails(b"a message"));
        }
    }

    mod step_03_serialization {
        use super::*;

        #[test]
        fn test_public_key_hex_roundtrip() {
            let key = generate_keypair();
            let hex = public_key_to_hex(&key.verifying_key());
            let parsed = public_key_from_hex(&hex).unwrap();
            assert_eq!(key.verifying_key().to_bytes(), parsed.to_bytes());
        }

        #[test]
        fn test_public_key_from_hex_invalid() {
            let result = public_key_from_hex("not-hex-at-all");
            assert!(result.is_err());
        }

        #[test]
        fn test_public_key_hex_length() {
            let key = generate_keypair();
            let hex = public_key_to_hex(&key.verifying_key());
            assert_eq!(hex.len(), 64);
        }
    }
}
