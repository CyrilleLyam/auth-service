use crate::config::JWT_SECRET;
use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::handler::Claims;

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let secret = &JWT_SECRET.clone();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "hunter42";
        let hashed_password = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed_password).is_ok());
    }

    #[test]
    fn test_verify_password_with_wrong_password() {
        let password = "hunter42";
        let wrong_password = "wrongpassword";
        let hashed_password = hash_password(password).unwrap();
        assert!(verify_password(wrong_password, &hashed_password).is_err());
    }

    #[test]
    fn test_generate_salt() {
        let salt = SaltString::generate(&mut OsRng);
        assert!(!salt.as_str().is_empty());
    }
}
