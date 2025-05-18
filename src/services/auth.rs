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
    pub sub: String, // cpf_cnpj
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

fn secret() -> Vec<u8> {
    env::var("JWT_SECRET")
        .expect("JWT_SECRET missing")
        .into_bytes()
}

fn encode_jwt(claims: &Claims) -> String {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(&secret()),
    )
    .unwrap()
}

pub fn generate_tokens(pk: &str) -> (String, String) {
    let now = OffsetDateTime::now_utc();
    let access = Claims {
        sub: pk.to_owned(),
        exp: (now + Duration::hours(1)).unix_timestamp(),
        kind: "access".into(),
    };
    let refresh = Claims {
        sub: pk.to_owned(),
        exp: (now + Duration::days(30)).unix_timestamp(),
        kind: "refresh".into(),
    };
    (encode_jwt(&access), encode_jwt(&refresh))
}

pub fn verify_token(tok: &str, kind: &str) -> Option<Claims> {
    let data = decode::<Claims>(
        tok,
        &DecodingKey::from_secret(&secret()),
        &Validation::new(Algorithm::HS256),
    )
    .ok()?;
    (data.claims.kind == kind).then_some(data.claims)
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
        let (a, r) = generate_tokens("12345678900");
        assert!(verify_token(&a, "access").is_some());
        assert!(verify_token(&r, "refresh").is_some());
        assert!(verify_token(&a, "refresh").is_none());
    }
}
