use crate::errors::{
    ERROR_PASSWORD_EXCEEDS_MAX_LENGTH, ERROR_PASSWORD_FAILED_TO_HASH,
    ERROR_PASSWORD_FAILED_TO_MATCH, ERROR_PASSWORD_IS_EMPTY,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

const MAX_PASSWORD_LENGTH: usize = 12;

pub fn hash(password: impl Into<String>) -> Result<String, String> {
    let password = password.into();
    let argon2 = Argon2::default();

    if password.is_empty() {
        return Err(ERROR_PASSWORD_IS_EMPTY.to_string());
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ERROR_PASSWORD_EXCEEDS_MAX_LENGTH.to_string());
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> String {
    let argon2 = Argon2::default();

    if password.is_empty() {
        ERROR_PASSWORD_IS_EMPTY.to_string()
    } else if password.len() > MAX_PASSWORD_LENGTH {
        ERROR_PASSWORD_EXCEEDS_MAX_LENGTH.to_string()
    } else {
        let parsed_hash = PasswordHash::new(hashed_password);

        if let Ok(parsed_hash) = parsed_hash {
            let password_matched = argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);
            if password_matched {
                let data = r#"{
                "status": "success",
                "data": "user validated"
            }"#;
                data.to_string()
            } else {
                ERROR_PASSWORD_FAILED_TO_MATCH.to_string()
            }
        } else {
            ERROR_PASSWORD_FAILED_TO_HASH.to_string()
        }
    }
}
