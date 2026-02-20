pub mod email;
pub mod error;
pub mod password_hash;
pub mod service;
pub mod user_id;

pub use email::Email;
pub use error::{EmailError, PasswordError, UserError, UserRepositoryError};
pub use password_hash::PasswordHash;
pub use service::UserUniquenessChecker;
pub use user_id::UserId;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
