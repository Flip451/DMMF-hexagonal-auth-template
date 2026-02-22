use argon2::{
    Argon2,
    password_hash::{
        PasswordHash as Argon2Hash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};
use async_trait::async_trait;
use domain::models::auth::{PasswordService, PasswordServiceError, RawPassword};
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
    async fn verify(
        &self,
        password: &RawPassword,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordServiceError> {
        let parsed_hash = Argon2Hash::new(hash.as_ref())
            .map_err(|e| PasswordServiceError::VerificationFailed(e.into()))?;

        let is_valid = Argon2::default()
            .verify_password(password.expose_as_str().as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }

    async fn hash(&self, password: &RawPassword) -> Result<PasswordHash, PasswordServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hash_str = Argon2::default()
            .hash_password(password.expose_as_str().as_bytes(), &salt)
            .map_err(|e| PasswordServiceError::HashingFailed(e.into()))?
            .to_string();

        Ok(PasswordHash::from_str_unchecked(hash_str))
    }
}
