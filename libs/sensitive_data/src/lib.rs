use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

/// マスキングの有効/無効を管理するグローバルフラグ。
static MASK_ENABLED: AtomicBool = AtomicBool::new(true);

/// システム全体のマスキング動作を制御します。
pub struct MaskingControl;

impl MaskingControl {
    /// マスキングが有効かどうかを返します。
    pub fn is_enabled() -> bool {
        MASK_ENABLED.load(Ordering::Relaxed)
    }

    /// マスキングの有効/無効を設定します。
    pub fn set_enabled(enabled: bool) {
        MASK_ENABLED.store(enabled, Ordering::Relaxed);
    }
}

/// 機密情報の隠蔽ルールを定義するトレイト。
pub trait SensitiveData {
    /// インスタンスの値を隠蔽した文字列を返します。
    fn to_masked_string(&self) -> String;

    /// 隠蔽ルールを静的に適用します。
    fn mask_raw(input: &str) -> String;
}

/// 汎用的な機密情報ラッパー。
/// 境界領域（DTO等）で、隠蔽ルール `S` を任意のデータ `T` に適用します。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sensitive<T, S = PlainRule> {
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

    pub fn as_inner(&self) -> &T {
        &self.inner
    }
}

// T が AsRef<str> を実装している場合（String や &str など）のユーティリティ
impl<T: AsRef<str>, S> Sensitive<T, S> {
    pub fn is_empty(&self) -> bool {
        self.inner.as_ref().is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.as_ref().len()
    }
}

impl<T, S> From<T> for Sensitive<T, S> {
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T: fmt::Display, S: SensitiveData> fmt::Debug for Sensitive<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if MaskingControl::is_enabled() {
            write!(f, "\"{}\"", S::mask_raw(&self.inner.to_string()))
        } else {
            write!(f, "{:?}", self.inner.to_string())
        }
    }
}

impl<T: fmt::Display, S: SensitiveData> fmt::Display for Sensitive<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if MaskingControl::is_enabled() {
            write!(f, "{}", S::mask_raw(&self.inner.to_string()))
        } else {
            write!(f, "{}", self.inner)
        }
    }
}

// --- 具体的な隠蔽ルール（マーカー型） ---

/// 汎用的な隠蔽ルール（最初と最後だけ残す）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlainRule;

impl SensitiveData for PlainRule {
    fn to_masked_string(&self) -> String {
        String::new()
    }

    fn mask_raw(input: &str) -> String {
        mask_generic(input)
    }
}

/// メールアドレス用の隠蔽ルール。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmailRule;

impl SensitiveData for EmailRule {
    fn to_masked_string(&self) -> String {
        String::new()
    }

    fn mask_raw(input: &str) -> String {
        mask_email(input)
    }
}

/// 完全に隠蔽するルール（常に ***）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecretRule;

impl SensitiveData for SecretRule {
    fn to_masked_string(&self) -> String {
        String::new()
    }

    fn mask_raw(_input: &str) -> String {
        "***".to_string()
    }
}

/// 完全に隠蔽するルール（SecretRule のエイリアスまたは同等の振る舞い）。
/// トークン等に明示的に使用します。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRule;

impl SensitiveData for TokenRule {
    fn to_masked_string(&self) -> String {
        String::new()
    }

    fn mask_raw(input: &str) -> String {
        SecretRule::mask_raw(input)
    }
}

// --- 隠蔽アルゴリズム（ユーティリティ） ---

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
    use rstest::rstest;

    #[rstest]
    #[case("test@example.com", "t***@example.com")]
    #[case("a@b.com", "a***@b.com")]
    #[case("info@sub.domain.com", "i***@sub.domain.com")]
    #[case("@no-local", "@***l")]
    #[case("no-at-sign", "n***n")]
    #[case("", "")]
    fn test_mask_email(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(mask_email(input), expected);
    }

    #[rstest]
    #[case("", "")]
    #[case("a", "*")]
    #[case("ab", "**")]
    #[case("abc", "***")]
    #[case("abcd", "a***d")]
    #[case("1234567890", "1***0")]
    #[case("12345678901", "123***901")]
    #[case("verylongsecretvalue", "ver***lue")]
    #[case("あ", "*")]
    #[case("あいう", "***")]
    #[case("あいうえ", "あ***え")]
    #[case("あいうえおかきくけこさ", "あいう***けこさ")]
    fn test_mask_generic(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(mask_generic(input), expected);
    }

    #[test]
    fn test_masking_control_toggle() {
        let sensitive = Sensitive::<String, PlainRule>::new("secretvalue".to_string());

        MaskingControl::set_enabled(true);
        assert_eq!(format!("{:?}", sensitive), "\"sec***lue\"");

        MaskingControl::set_enabled(false);
        assert_eq!(format!("{:?}", sensitive), "\"secretvalue\"");
    }

    #[test]
    fn test_serde_transparency() {
        let sensitive = Sensitive::<String, PlainRule>::new("secret".to_string());
        let json = serde_json::to_string(&sensitive).unwrap();
        assert_eq!(json, "\"secret\"");

        let back: Sensitive<String, PlainRule> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.into_inner(), "secret");
    }

    #[rstest]
    #[case::plain(Sensitive::<String, PlainRule>::new("12345678".to_string()), "\"1***8\"")]
    #[case::email(Sensitive::<String, EmailRule>::new("test@example.com".to_string()), "\"t***@example.com\"")]
    #[case::secret(Sensitive::<String, SecretRule>::new("topsecret".to_string()), "\"***\"")]
    #[case::token(Sensitive::<String, TokenRule>::new("token123".to_string()), "\"***\"")]
    fn test_rules_via_wrapper(
        #[case] sensitive: Sensitive<String, impl SensitiveData>,
        #[case] expected: &str,
    ) {
        MaskingControl::set_enabled(true);
        assert_eq!(format!("{:?}", sensitive), expected);
    }

    #[test]
    fn test_sensitive_utils() {
        let s = Sensitive::<String, PlainRule>::new("abc".to_string());
        assert_eq!(s.len(), 3);
        assert!(!s.is_empty());

        let empty = Sensitive::<String, PlainRule>::new("".to_string());
        assert!(empty.is_empty());
    }
}
