pub mod error;

pub use error::AuthError;

use crate::models::user::{PasswordHash, UserId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordServiceError {
    #[error("Failed to hash password")]
    HashingFailed(#[source] anyhow::Error),

    #[error("Failed to verify password")]
    VerificationFailed(#[source] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Failed to issue token")]
    IssuanceFailed(#[source] anyhow::Error),

    #[error("Failed to verify token")]
    VerificationFailed(#[source] anyhow::Error),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,
}

#[async_trait]
pub trait PasswordService: Send + Sync {
    /// 生のパスワードがハッシュと一致するか検証する
    async fn verify(
        &self,
        password: &str,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordServiceError>;

    /// 生のパスワードをハッシュ化する（サインアップ用）
    async fn hash(&self, password: &str) -> Result<PasswordHash, PasswordServiceError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub iat: usize,
    pub exp: usize,
}

#[async_trait]
pub trait AuthService: Send + Sync {
    /// ユーザーIDからJWTを発行する
    fn issue_token(&self, user_id: UserId) -> Result<String, AuthServiceError>;

    /// JWTを検証し、Claimsを返す
    fn verify_token(&self, token: &str) -> Result<Claims, AuthServiceError>;
}
