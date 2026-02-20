use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UserRepository: Send + Sync {
    // プレースホルダーメソッド
}
