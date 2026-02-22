use async_trait::async_trait;
use domain::clock::Clock;
use domain::repository::tx::{IntoTxError, RepositoryFactory, TransactionManager};
use futures_util::future::BoxFuture;
use sqlx::{Pool, Postgres, Transaction};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::repository::user_adapter::SqlxUserRepoAdapter;

pub struct SqlxRepositoryFactory<'a, C: Clock> {
    transaction: Arc<Mutex<Option<Transaction<'a, Postgres>>>>,
    clock: Arc<C>,
}

impl<'a, C: Clock> RepositoryFactory for SqlxRepositoryFactory<'a, C> {
    fn user_repository(&self) -> Arc<dyn domain::models::user::UserRepository + '_> {
        Arc::new(SqlxUserRepoAdapter::new(
            Arc::clone(&self.transaction),
            Arc::clone(&self.clock),
        ))
    }
}

pub struct SqlxTransactionManager<C: Clock> {
    pool: Pool<Postgres>,
    clock: Arc<C>,
}

impl<C: Clock> SqlxTransactionManager<C> {
    pub fn new(pool: Pool<Postgres>, clock: Arc<C>) -> Self {
        Self { pool, clock }
    }
}

#[async_trait]
impl<C: Clock> TransactionManager for SqlxTransactionManager<C> {
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send,
        E: IntoTxError + Debug + Send + Sync,
        F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send,
    {
        let tx = self.pool.begin().await.map_err(|e| E::into_tx_error(e))?;

        let transaction = Arc::new(Mutex::new(Some(tx)));
        let factory = SqlxRepositoryFactory {
            transaction: Arc::clone(&transaction),
            clock: Arc::clone(&self.clock),
        };

        let result = f(&factory).await;

        // トランザクションを Option から取り出す。
        // これにより、他の Arc 参照が残っていても安全に所有権を回収できる。
        let mut guard = transaction.lock().await;
        if let Some(tx) = guard.take() {
            match &result {
                Ok(_) => {
                    tx.commit().await.map_err(|e| E::into_tx_error(e))?;
                }
                Err(e) => {
                    // ロールバックが失敗しても、元のビジネスエラーを優先して返す
                    if let Err(rollback_err) = tx.rollback().await {
                        tracing::error!(
                            error = ?rollback_err,
                            "Failed to rollback transaction. Original error: {:?}",
                            e
                        );
                    }
                }
            }
        }

        result
    }
}
