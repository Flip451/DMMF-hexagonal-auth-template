pub mod error;

pub use error::AuthError;

use crate::SensitiveDebug;
use crate::models::user::PasswordHash;
use async_trait::async_trait;
use derive_more::{Display, From};
use sensitive_data::{SecretRule, SensitiveData};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordServiceError {
    #[error("Failed to hash password")]
    HashingFailed(#[source] anyhow::Error),

    #[error("Failed to verify password")]
    VerificationFailed(#[source] anyhow::Error),
}

/// 生の（ハッシュ化前の）パスワードを表現する値オブジェクト。
///
/// ## 設計思想
/// この型は「機密情報の漏洩防止」を目的としており、`SensitiveData` による
/// 自動的なログマスキングを提供します。
///
/// 一方で、平文のパスワードを保持しているという事実を隠蔽せず、
/// 中身を取り出す際には明示的な `expose_as_str()` メソッドの呼び出しを要求します。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Display, From, SensitiveDebug)]
pub struct RawPassword(String);

impl RawPassword {
    /// 平文のパスワードを文字列として露出させます。
    ///
    /// ## 注意
    /// このメソッドはハッシュ化やバリデーションなど、平文が必要な
    /// 最小限の箇所でのみ使用してください。
    pub fn expose_as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for RawPassword {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl SensitiveData for RawPassword {
    fn to_masked_string(&self) -> String {
        Self::mask_raw(&self.0)
    }

    fn mask_raw(input: &str) -> String {
        // 生パスワードは一切の情報を露出させないため、SecretRule（完全隠蔽）を適用
        SecretRule::mask_raw(input)
    }
}

/// パスワードのハッシュ化および検証を行うドメインサービス。
#[async_trait]
pub trait PasswordService: Send + Sync {
    /// 生のパスワードがハッシュと一致するか検証する
    async fn verify(
        &self,
        password: &RawPassword,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordServiceError>;

    /// 生のパスワードをハッシュ化する（サインアップ用）
    async fn hash(&self, password: &RawPassword) -> Result<PasswordHash, PasswordServiceError>;
}
