use crate::error::AuthServiceError;
use async_trait::async_trait;
use derive_more::{Display, From};
use domain::SensitiveDebug;
use domain::models::user::UserId;
use sensitive_data::{SecretRule, SensitiveData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub iat: usize,
    pub exp: usize,
}

/// 認証用トークン（JWT等）を表現する値オブジェクト。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Display, From, SensitiveDebug)]
pub struct AuthToken(String);

impl AuthToken {
    pub fn expose_as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AuthToken {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl SensitiveData for AuthToken {
    fn to_masked_string(&self) -> String {
        Self::mask_raw(&self.0)
    }

    fn mask_raw(input: &str) -> String {
        SecretRule::mask_raw(input)
    }
}

/// 認証・認可に関する外部サービス（JWT発行等）との境界を定義するポート。
#[async_trait]
pub trait AuthService: Send + Sync {
    /// ユーザーIDから認証トークンを発行する
    fn issue_token(&self, user_id: UserId) -> Result<AuthToken, AuthServiceError>;

    /// 認証トークンを検証し、Claimsを返す
    fn verify_token(&self, token: &AuthToken) -> Result<Claims, AuthServiceError>;
}
