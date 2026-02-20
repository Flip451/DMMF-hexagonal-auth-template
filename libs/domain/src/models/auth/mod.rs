pub mod error;

pub use error::AuthError;

use async_trait::async_trait;

#[async_trait]
pub trait AuthService: Send + Sync {
    // 認証に関連するドメイン知識のインターフェース
    // （JWT生成の実装などはインフラ層で行う）
}
