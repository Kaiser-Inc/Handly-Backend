use argon2::{
    password_hash::{Error as PwError, PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use password_hash::{PasswordHasher, SaltString};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

const JWT_SECRET: &[u8] = b"super-secret-change-me";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: i64,
}

pub fn hash_password(plain: &str) -> Result<String, PwError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(plain.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password(hash: &str, plain: &str) -> bool {
    if let Ok(parsed) = PasswordHash::new(hash) {
        Argon2::default()
            .verify_password(plain.as_bytes(), &parsed)
            .is_ok()
    } else {
        false
    }
}

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp();
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_not_plaintext() {
        let plain = "my-secret";
        let hash = hash_password(plain).unwrap();
        assert_ne!(plain, hash);
    }

    #[test]
    fn verify_correct_password() {
        let plain = "123456";
        let hash = hash_password(plain).unwrap();
        assert!(verify_password(&hash, plain));
    }

    #[test]
    fn verify_wrong_password_fails() {
        let hash = hash_password("correct").unwrap();
        assert!(!verify_password(&hash, "wrong"));
    }
}
