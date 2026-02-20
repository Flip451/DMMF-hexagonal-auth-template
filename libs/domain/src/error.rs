use crate::models::auth::error::AuthError;
use crate::models::user::error::{UserError, UserRepositoryError, UserUniquenessViolation};
use crate::repository::tx::IntoTxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Auth(#[from] AuthError),

    /// インフラ層の技術的失敗
    #[error("Infrastructure failure: {0}")]
    Infrastructure(#[from] anyhow::Error),

    /// 論理的な不変条件の違反（バグ）
    #[error("Logic invariant violation: {0}")]
    LogicViolation(&'static str),
}

// 推移的な From 実装により、UseCase層での ? 演算子を使いやすくする
impl From<UserUniquenessViolation> for DomainError {
    fn from(error: UserUniquenessViolation) -> Self {
        Self::User(UserError::from(error))
    }
}

impl From<UserRepositoryError> for DomainError {
    fn from(error: UserRepositoryError) -> Self {
        Self::User(UserError::from(error))
    }
}

impl IntoTxError for DomainError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self {
        Self::Infrastructure(error.into())
    }
}

pub type DomainResult<T> = Result<T, DomainError>;
