use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::fmt::Debug;
use std::sync::Arc;

use crate::models::user::UserRepository;

/// DB等のシステムエラーを、そのドメインのエラー型に変換するためのトレイト
pub trait IntoTxError {
    fn into_tx_error(error: impl Into<anyhow::Error>) -> Self;
}

pub trait RepositoryFactory: Send + Sync {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    // 将来的な拡張:
    // fn outbox_repository(&self) -> Arc<dyn OutboxRepository>;
}

#[async_trait]
pub trait TransactionManager: Send + Sync {
    async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
    where
        T: Send,
        E: IntoTxError + Debug + Send + Sync,
        F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send;
}

// 便利なマクロはそのまま
#[macro_export]
macro_rules! tx {
    ($tm:expr, |$factory:ident| $body:expr) => {
        $tm.execute::<_, _, _>(move |$factory| std::boxed::Box::pin(async move { $body }))
    };
}
