use crate::models::user::error::EmailError;
use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef)]
pub struct Email(String);

impl Email {
    pub fn into_inner(self) -> String {
        self.0
    }
}

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
}
