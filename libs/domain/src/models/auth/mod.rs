pub mod error;

pub use error::AuthError;

use crate::models::user::PasswordHash;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordServiceError {
    #[error("Failed to hash password")]
    HashingFailed(#[source] anyhow::Error),

    #[error("Failed to verify password")]
    VerificationFailed(#[source] anyhow::Error),
}

#[async_trait]
pub trait PasswordService: Send + Sync {
    /// 生のパスワードがハッシュと一致するか検証する
    async fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, AuthError>;

    /// 生のパスワードをハッシュ化する（サインアップ用）
    async fn hash(&self, password: &str) -> Result<PasswordHash, AuthError>;
}

#[async_trait]
pub trait AuthService: Send + Sync {
    // 認証に関連するその他のドメイン知識のインターフェース
}
