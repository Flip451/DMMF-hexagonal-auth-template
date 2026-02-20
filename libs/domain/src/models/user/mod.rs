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

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum UserRepositoryError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Database query failed: {0}")]
    QueryFailed(String),

    #[error("Data mapping failed: {0}")]
    MappingFailed(String),

    #[error("Unknown repository error: {0}")]
    Unknown(String),
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
