use argon2::password_hash::{Error as PwError, PasswordHasher};
use argon2::Argon2;
use password_hash::SaltString;
use rand_core::OsRng;

pub fn hash_password(plain: &str) -> Result<String, PwError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(plain.as_bytes(), &salt)?
        .to_string())
}
