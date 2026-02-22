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
use crate::Entity;

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

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
pub struct User {
    #[entity(id)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::user_id::UserId;
    use uuid::Uuid;

    #[test]
    fn test_user_equality_based_on_id() {
        let id1 = UserId::from(Uuid::now_v7());
        let id2 = UserId::from(Uuid::now_v7());
        let email1 = Email::try_from("test1@example.com").unwrap();
        let email2 = Email::try_from("test2@example.com").unwrap();
        let pw = PasswordHash::from_str_unchecked("hash");

        let user1 = User::new(id1, email1.clone(), pw.clone());
        let user1_different_email = User::new(id1, email2, pw.clone());
        let user2 = User::new(id2, email1, pw);

        // Same ID should be equal even if other fields differ
        assert_eq!(user1, user1_different_email);
        // Different ID should not be equal
        assert_ne!(user1, user2);
    }
}
