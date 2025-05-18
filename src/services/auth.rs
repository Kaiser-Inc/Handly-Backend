use argon2::{
    password_hash::{Error as PwError, PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use password_hash::{PasswordHasher, SaltString};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::env;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub kind: String, // "access" | "refresh"
}

pub fn hash_password(p: &str) -> Result<String, PwError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(p.as_bytes(), &salt)?
        .to_string())
}

pub fn verify_password(hash: &str, p: &str) -> bool {
    PasswordHash::new(hash)
        .and_then(|ph| Argon2::default().verify_password(p.as_bytes(), &ph))
        .is_ok()
}

fn secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET missing")
}

fn jwt(claims: &Claims) -> String {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(secret().as_bytes()),
    )
    .unwrap()
}

pub fn generate_tokens(user_id: &str) -> (String, String) {
    let now = OffsetDateTime::now_utc();
    let access = Claims {
        sub: user_id.to_owned(),
        exp: (now + Duration::hours(1)).unix_timestamp(),
        kind: "access".into(),
    };
    let refresh = Claims {
        sub: user_id.to_owned(),
        exp: (now + Duration::days(30)).unix_timestamp(),
        kind: "refresh".into(),
    };
    (jwt(&access), jwt(&refresh))
}

pub fn verify_token(token: &str, expected_kind: &str) -> Option<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .ok()?;
    if data.claims.kind == expected_kind {
        Some(data.claims)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn hash_and_verify() {
        let h = hash_password("pwd").unwrap();
        assert!(verify_password(&h, "pwd"));
        assert!(!verify_password(&h, "bad"));
    }

    #[test]
    fn tokens_roundtrip() {
        env::set_var("JWT_SECRET", "test-secret");
        let (acc, ref_tok) = generate_tokens("u1");
        assert!(verify_token(&acc, "access").is_some());
        assert!(verify_token(&ref_tok, "refresh").is_some());
        assert!(verify_token(&acc, "refresh").is_none());
    }
}
