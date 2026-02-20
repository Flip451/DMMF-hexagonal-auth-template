pub mod email;
pub mod error;
pub mod password_hash;
pub mod user_id;

pub use email::Email;
pub use error::{EmailError, PasswordError, UserError};
pub use password_hash::PasswordHash;
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
    // placeholder
}
