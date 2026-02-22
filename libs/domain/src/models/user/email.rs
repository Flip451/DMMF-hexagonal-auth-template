use sensitive_data::{EmailRule, SensitiveData};
use crate::SensitiveDebug;
use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum EmailError {
    #[error("Email is empty")]
    Empty,
    #[error("Email format is invalid")]
    InvalidFormat,
}

/// ユーザーのメールアドレスを表す値オブジェクト。
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef, SensitiveDebug)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }
        if !value.contains('@') {
            return Err(EmailError::InvalidFormat);
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl SensitiveData for Email {
    fn to_masked_string(&self) -> String {
        Self::mask_raw(&self.0)
    }

    fn mask_raw(input: &str) -> String {
        EmailRule::mask_raw(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test@example.com", Ok("test@example.com"))]
    #[case("", Err(EmailError::Empty))]
    #[case("invalid-email", Err(EmailError::InvalidFormat))]
    fn test_email_validation(#[case] input: &str, #[case] expected: Result<&str, EmailError>) {
        let result = Email::try_from(input);
        match expected {
            Ok(val) => assert_eq!(result.unwrap().to_string(), val),
            Err(e) => assert_eq!(result.unwrap_err(), e),
        }
    }

    #[test]
    fn test_email_as_ref() {
        let email = Email::try_from("test@example.com").unwrap();
        let s: &str = email.as_ref();
        assert_eq!(s, "test@example.com");
    }

    #[test]
    fn test_email_masking() {
        let email = Email::try_from("test@example.com").unwrap();
        assert_eq!(email.to_masked_string(), "t***@example.com");
        // SensitiveDebug マクロによる Debug 出力の検証
        assert_eq!(format!("{:?}", email), "\"t***@example.com\"");
    }
}
