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

    #[test]
    fn test_email_validation() {
        let valid_email = "test@example.com";
        let email = Email::try_from(valid_email).unwrap();
        assert_eq!(email.to_string(), valid_email);

        let invalid_email = "invalid-email";
        let result = Email::try_from(invalid_email);
        assert!(matches!(result, Err(EmailError::InvalidFormat)));

        let empty_email = "";
        let result = Email::try_from(empty_email);
        assert!(matches!(result, Err(EmailError::Empty)));
    }
}
