use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

/// 機密情報（個人情報、認証資格情報など）を保持する型を表すトレイト。
pub trait SensitiveData {
    /// 情報を部分的に隠蔽した文字列を返します。
    fn to_masked_string(&self) -> String;

    /// 静的な隠蔽ルールを提供します。
    fn mask_raw(input: &str) -> String;
}

/// 汎用的な機密情報ラッパー。
/// 境界領域（DTO等）で、ドメイン層の隠蔽ルール `S` を任意のデータ `T` に適用します。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sensitive<T, S = Plain> {
    inner: T,
    #[serde(skip)]
    _marker: PhantomData<S>,
}

impl<T, S> Sensitive<T, S> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T, S> From<T> for Sensitive<T, S> {
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: fmt::Display, S: SensitiveData> fmt::Debug for Sensitive<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", S::mask_raw(&self.inner.to_string()))
    }
}

impl<T: fmt::Display, S: SensitiveData> fmt::Display for Sensitive<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", S::mask_raw(&self.inner.to_string()))
    }
}

/// 汎用隠蔽用のマーカー型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plain;

impl SensitiveData for Plain {
    fn to_masked_string(&self) -> String {
        String::new()
    }

    fn mask_raw(input: &str) -> String {
        mask_generic(input)
    }
}

/// メールアドレス用の部分隠蔽ロジック（ユーティリティ）。
pub fn mask_email(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    if let Some(at_index) = input.find('@').filter(|&idx| idx > 0) {
        let local_part = &input[..at_index];
        let domain_part = &input[at_index..];
        let first_char = local_part.chars().next().unwrap();
        return format!("{}***{}", first_char, domain_part);
    }
    mask_generic(input)
}

/// 汎用的な部分隠蔽ロジック（ユーティリティ）。
pub fn mask_generic(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    if len == 0 {
        return String::new();
    }
    if len <= 3 {
        return "*".repeat(len);
    }

    let visible_count = if len > 10 { 3 } else { 1 };
    let start: String = chars.iter().take(visible_count).collect();
    let end: String = chars.iter().skip(len - visible_count).collect();

    format!("{}***{}", start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("test@example.com"), "t***@example.com");
        assert_eq!(mask_email("a@b.com"), "a***@b.com");
    }

    #[test]
    fn test_mask_generic() {
        assert_eq!(mask_generic("v1.abc123456789xyz"), "v1.***xyz");
        assert_eq!(mask_generic("short"), "s***t");
        assert_eq!(mask_generic("ab"), "**");
    }

    #[test]
    fn test_sensitive_wrapper_delegation() {
        // Plain ルール（汎用隠蔽）
        let sensitive = Sensitive::<String, Plain>::new("v1.secretvalue".to_string());
        assert_eq!(format!("{:?}", sensitive), "\"v1.***lue\"");
    }
}
