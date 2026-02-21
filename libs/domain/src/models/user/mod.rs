pub mod email;
pub mod error;
pub mod password_hash;
pub mod service;
pub mod user_id;

pub use email::{Email, EmailError};
pub use error::UserError;
pub use password_hash::{PasswordError, PasswordHash};
pub use service::{UserUniquenessChecker, UserUniquenessViolation};
pub use user_id::UserId;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// ユーザーの識別情報を表すトレイト。
pub trait UserIdentity {
    fn id(&self) -> UserId;
    fn email(&self) -> &Email;
}

/// 認証可能なユーザー（パスワード情報を保持する）を表すトレイト。
pub trait Authenticatable {
    fn password_hash(&self) -> &PasswordHash;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Database connection failed")]
    ConnectionFailed,

    #[error("Database query failed: {0}")]
    QueryFailed(#[source] anyhow::Error),

    #[error("Data mapping failed: {0}")]
    MappingFailed(#[source] anyhow::Error),

    #[error("Unexpected repository error")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
}

impl User {
    /// ユーザーモデルの新規生成。
    pub fn new(id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }
}

impl UserIdentity for User {
    fn id(&self) -> UserId {
        self.id
    }

    fn email(&self) -> &Email {
        &self.email
    }
}

impl Authenticatable for User {
    fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
}
