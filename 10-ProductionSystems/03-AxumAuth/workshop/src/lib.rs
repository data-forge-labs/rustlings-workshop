use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("missing Authorization header")]
    MissingHeader,
    #[error("invalid Authorization scheme, expected 'Bearer'")]
    InvalidScheme,
    #[error("invalid token: {0}")]
    InvalidToken(String),
    #[error("token expired")]
    Expired,
    #[error("unauthorized role: {0}")]
    UnauthorizedRole(String),
}

pub fn sign_token(claims: &Claims, secret: &[u8]) -> Result<String, AuthError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| AuthError::InvalidToken(e.to_string()))
}

pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
        _ => AuthError::InvalidToken(e.to_string()),
    })
}

pub fn extract_bearer(authorization_header: &str) -> Result<&str, AuthError> {
    if authorization_header.is_empty() {
        return Err(AuthError::MissingHeader);
    }
    if let Some(token) = authorization_header.strip_prefix("Bearer ") {
        Ok(token)
    } else {
        Err(AuthError::InvalidScheme)
    }
}

pub fn is_expired(claims: &Claims) -> bool {
    Utc::now().timestamp() >= claims.exp
}

pub fn create_access_token(
    subject: &str,
    role: &str,
    secret: &[u8],
    ttl_seconds: i64,
) -> Result<String, AuthError> {
    let now = Utc::now();
    let claims = Claims {
        sub: subject.to_string(),
        role: role.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::seconds(ttl_seconds)).timestamp(),
    };
    sign_token(&claims, secret)
}

pub fn create_refresh_token(subject: &str, secret: &[u8]) -> Result<String, AuthError> {
    let now = Utc::now();
    let claims = Claims {
        sub: subject.to_string(),
        role: "refresh".to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::days(30)).timestamp(),
    };
    sign_token(&claims, secret)
}

pub fn has_role(claims: &Claims, allowed: &[&str]) -> bool {
    allowed.contains(&claims.role.as_str())
}

pub fn require_role(claims: &Claims, allowed: &[&str]) -> Result<(), AuthError> {
    if has_role(claims, allowed) {
        Ok(())
    } else {
        Err(AuthError::UnauthorizedRole(claims.role.clone()))
    }
}

pub fn key_id_from_header(token: &str) -> Option<&str> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    let header_b64 = parts[0];
    use base64::Engine;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(header_b64).ok()?;
    let header: serde_json::Value = serde_json::from_slice(&decoded).ok()?;
    header.get("kid")?.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"super-secret-key-for-tests-only-32+chars";

    fn make_claims() -> Claims {
        let now = Utc::now();
        Claims {
            sub: "user-1".to_string(),
            role: "admin".to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(1)).timestamp(),
        }
    }

    mod step_01_sign_verify {
        use super::*;

        #[test]
        fn test_sign_then_verify_roundtrip() {
            let claims = make_claims();
            let token = sign_token(&claims, SECRET).unwrap();
            let decoded = verify_token(&token, SECRET).unwrap();
            assert_eq!(decoded.sub, "user-1");
            assert_eq!(decoded.role, "admin");
        }

        #[test]
        fn test_verify_with_wrong_secret_fails() {
            let claims = make_claims();
            let token = sign_token(&claims, SECRET).unwrap();
            let err = verify_token(&token, b"a-different-secret-key-1234567890").unwrap_err();
            assert!(matches!(err, AuthError::InvalidToken(_)));
        }

        #[test]
        fn test_verify_garbage_token_fails() {
            let err = verify_token("not-a-jwt", SECRET).unwrap_err();
            assert!(matches!(err, AuthError::InvalidToken(_)));
        }
    }

    mod step_02_extract_bearer {
        use super::*;

        #[test]
        fn test_extract_bearer_ok() {
            assert_eq!(extract_bearer("Bearer abc.def.ghi").unwrap(), "abc.def.ghi");
        }

        #[test]
        fn test_extract_bearer_lowercase_scheme_rejected() {
            assert_eq!(
                extract_bearer("bearer abc.def.ghi").unwrap_err(),
                AuthError::InvalidScheme
            );
        }

        #[test]
        fn test_extract_bearer_missing() {
            assert_eq!(extract_bearer("").unwrap_err(), AuthError::MissingHeader);
        }

        #[test]
        fn test_extract_bearer_basic_scheme_rejected() {
            assert_eq!(
                extract_bearer("Basic dXNlcjpwYXNz").unwrap_err(),
                AuthError::InvalidScheme
            );
        }
    }

    mod step_03_expiration {
        use super::*;

        #[test]
        fn test_fresh_claim_not_expired() {
            assert!(!is_expired(&make_claims()));
        }

        #[test]
        fn test_past_claim_is_expired() {
            let now = Utc::now();
            let claims = Claims {
                sub: "u".into(),
                role: "user".into(),
                iat: (now - Duration::hours(2)).timestamp(),
                exp: (now - Duration::hours(1)).timestamp(),
            };
            assert!(is_expired(&claims));
        }
    }

    mod step_04_create_tokens {
        use super::*;

        #[test]
        fn test_create_access_token() {
            let token = create_access_token("u-1", "admin", SECRET, 3600).unwrap();
            let claims = verify_token(&token, SECRET).unwrap();
            assert_eq!(claims.sub, "u-1");
            assert_eq!(claims.role, "admin");
            assert!(claims.exp - claims.iat >= 3599);
        }

        #[test]
        fn test_create_refresh_token_long_lived() {
            let token = create_refresh_token("u-1", SECRET).unwrap();
            let claims = verify_token(&token, SECRET).unwrap();
            assert_eq!(claims.sub, "u-1");
            let now = Utc::now().timestamp();
            assert!(claims.exp - now >= 86_400 * 29);
        }
    }

    mod step_05_roles {
        use super::*;

        #[test]
        fn test_has_role_admin() {
            assert!(has_role(&make_claims(), &["admin", "user"]));
        }

        #[test]
        fn test_has_role_denied() {
            let claims = Claims {
                role: "viewer".into(),
                ..make_claims()
            };
            assert!(!has_role(&claims, &["admin"]));
        }

        #[test]
        fn test_require_role_ok() {
            assert!(require_role(&make_claims(), &["admin"]).is_ok());
        }

        #[test]
        fn test_require_role_unauthorized() {
            let claims = Claims {
                role: "viewer".into(),
                ..make_claims()
            };
            assert_eq!(
                require_role(&claims, &["admin"]).unwrap_err(),
                AuthError::UnauthorizedRole("viewer".into())
            );
        }
    }

    mod step_06_key_id {
        use super::*;

        #[test]
        fn test_key_id_default_header() {
            let claims = make_claims();
            let token = sign_token(&claims, SECRET).unwrap();
            assert_eq!(key_id_from_header(&token), None);
        }
    }
}
