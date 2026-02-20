use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};

/// ハッシュ化済みのパスワードを表現する値オブジェクト。
///
/// セキュリティ上の理由から、安易な生成（TryFrom等）は提供せず、
/// 基本的に `PasswordService` を介して生成されることを想定する。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// データベース等から取得した文字列を PasswordHash に再構成する。
    ///
    /// 注意: このメソッドは形式チェックを行わない。
    /// ドメインロジック内で生のパスワードを扱うために使用してはならない。
    pub fn from_str_unchecked(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn into_inner(self) -> String {
        self.0
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
}
