/// 機密情報（個人情報、認証資格情報など）を保持する型を表すトレイト。
pub trait SensitiveData {
    /// 情報を部分的に隠蔽した文字列を返します。
    fn to_masked_string(&self) -> String;
}

/// メールアドレス用の部分隠蔽ロジック。
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

/// 汎用的な部分隠蔽ロジック。最初と最後の数文字を残します。
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

impl SensitiveData for String {
    fn to_masked_string(&self) -> String {
        mask_generic(self)
    }
}

impl SensitiveData for &str {
    fn to_masked_string(&self) -> String {
        mask_generic(self)
    }
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
}
