use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use rand::rngs::OsRng;
use subtle::ConstantTimeEq;

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

pub fn is_password_valid(password: &str, min_length: usize) -> bool {
    password.len() >= min_length && !password.is_empty()
}

pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

pub fn hash_with_salt(password: &str, salt: &SaltString) -> Result<String, password_hash::Error> {
    let hash = Argon2::default().hash_password(password.as_bytes(), salt)?;
    Ok(hash.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_hash_and_verify {
        use super::*;

        #[test]
        fn test_hash_password_returns_string() {
            let hash = hash_password("hunter2").unwrap();
            assert!(hash.starts_with("$argon2"));
        }

        #[test]
        fn test_verify_password_correct() {
            let hash = hash_password("hunter2").unwrap();
            assert!(verify_password("hunter2", &hash).unwrap());
        }

        #[test]
        fn test_verify_password_wrong() {
            let hash = hash_password("hunter2").unwrap();
            assert!(!verify_password("hunter3", &hash).unwrap());
        }
    }

    mod step_02_salt {
        use super::*;

        #[test]
        fn test_generate_salt_is_unique() {
            let s1 = generate_salt();
            let s2 = generate_salt();
            assert_ne!(s1.as_str(), s2.as_str());
        }

        #[test]
        fn test_hash_with_salt_is_deterministic() {
            let salt = generate_salt();
            let h1 = hash_with_salt("hunter2", &salt).unwrap();
            let h2 = hash_with_salt("hunter2", &salt).unwrap();
            assert_eq!(h1, h2);
        }
    }

    mod step_03_validation {
        use super::*;

        #[test]
        fn test_is_password_valid_min_length() {
            assert!(is_password_valid("hunter2", 6));
            assert!(!is_password_valid("abc", 6));
        }

        #[test]
        fn test_is_password_valid_zero_length() {
            assert!(!is_password_valid("", 0)); // not valid even at min=0
        }
    }

    mod step_04_constant_time {
        use super::*;

        #[test]
        fn test_constant_time_eq_same() {
            assert!(constant_time_eq(b"hunter2", b"hunter2"));
        }

        #[test]
        fn test_constant_time_eq_different() {
            assert!(!constant_time_eq(b"hunter2", b"hunter3"));
        }

        #[test]
        fn test_constant_time_eq_different_length() {
            assert!(!constant_time_eq(b"hunter2", b"hunter22"));
        }
    }
}
