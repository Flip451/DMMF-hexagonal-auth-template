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

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Database connection failed")]
    ConnectionFailed,

    #[error("Database query failed")]
    QueryFailed(#[source] anyhow::Error),

    #[error("Data mapping failed")]
    MappingFailed(#[source] anyhow::Error),

    #[error("Unexpected repository error")]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
}
