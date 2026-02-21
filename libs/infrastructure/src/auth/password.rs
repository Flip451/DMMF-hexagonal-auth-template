use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash as Argon2Hash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use domain::models::auth::{AuthError, PasswordService, PasswordServiceError};
use domain::models::user::PasswordHash;

pub struct Argon2PasswordService;

impl Argon2PasswordService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Argon2PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PasswordService for Argon2PasswordService {
    async fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, AuthError> {
        let parsed_hash = Argon2Hash::new(hash.as_ref())
            .map_err(|e| PasswordServiceError::VerificationFailed(e.into()))?;

        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }

    async fn hash(&self, password: &str) -> Result<PasswordHash, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash_str = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| PasswordServiceError::HashingFailed(e.into()))?
            .to_string();

        Ok(PasswordHash::from_str_unchecked(hash_str))
    }
}
