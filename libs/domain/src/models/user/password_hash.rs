use crate::SensitiveDebug;
use derive_more::{AsRef, Display};
use sensitive_data::{SecretRule, SensitiveData};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum PasswordError {
    #[error("Password is too short")]
    TooShort,
    #[error("Password lacks required characters")]
    TooWeak,
    #[error("Invalid password hash format: {found}")]
    InvalidFormat { found: String },
}

/// ハッシュ化済みのパスワードを表現する値オブジェクト。
///
/// セキュリティ上の理由から、安易な生成（TryFrom等）は提供せず、
/// 基本的に `PasswordService` を介して生成されることを想定する。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, SensitiveDebug)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// データベース等から取得した文字列を PasswordHash に再構成する。
    ///
    /// 注意: このメソッドは形式チェックを行わない。
    /// ドメインロジック内で生のパスワードを扱うために使用してはならない。
    pub fn from_str_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl SensitiveData for PasswordHash {
    fn to_masked_string(&self) -> String {
        Self::mask_raw(&self.0)
    }

    fn mask_raw(input: &str) -> String {
        // パスワードハッシュも一切の情報を露出させないため、SecretRule（完全隠蔽）を適用
        SecretRule::mask_raw(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_reconstruction() {
        let hash_str = "any_string_from_db";
        let hash = PasswordHash::from_str_unchecked(hash_str);
        assert_eq!(hash.to_string(), hash_str);
    }

    #[test]
    fn test_password_hash_as_ref() {
        let hash_str = "any_string_from_db";
        let hash = PasswordHash::from_str_unchecked(hash_str);
        let s: &str = hash.as_ref();
        assert_eq!(s, hash_str);
    }

    #[test]
    fn test_password_hash_masking() {
        let hash = PasswordHash::from_str_unchecked("v1.longpasswordhashvalue");
        // 完全隠蔽されていることを確認
        assert_eq!(hash.to_masked_string(), "***");
        assert_eq!(format!("{:?}", hash), "\"***\"");
    }
}
