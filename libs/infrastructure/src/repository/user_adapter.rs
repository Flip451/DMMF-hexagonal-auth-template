use async_trait::async_trait;
use domain::clock::Clock;
use domain::models::user::{Email, User, UserRepository, UserRepositoryError};
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::repository::user::SqlxUserRepository;

/// トランザクションを保持し、`UserRepository` トレイトを実装するアダプター。
pub struct SqlxUserRepoAdapter<'a, C: Clock> {
    transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
    clock: Arc<C>,
}

impl<'a, C: Clock> SqlxUserRepoAdapter<'a, C> {
    pub fn new(transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>, clock: Arc<C>) -> Self {
        Self { transaction, clock }
    }
}

#[async_trait]
impl<'a, C: Clock> UserRepository for SqlxUserRepoAdapter<'a, C> {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let mut guard = self.transaction.lock().await;
        let tx = guard.as_mut().ok_or_else(|| {
            UserRepositoryError::Unexpected(anyhow::anyhow!("Transaction already closed or taken"))
        })?;
        SqlxUserRepository::find_by_email(&mut **tx, email).await
    }

    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        let mut guard = self.transaction.lock().await;
        let tx = guard.as_mut().ok_or_else(|| {
            UserRepositoryError::Unexpected(anyhow::anyhow!("Transaction already closed or taken"))
        })?;
        SqlxUserRepository::save(&mut **tx, user, &*self.clock).await
    }
}
