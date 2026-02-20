use crate::models::user::error::PasswordError;
use derive_more::{AsRef, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRef)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for PasswordHash {
    type Error = PasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PasswordError::TooShort);
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for PasswordHash {
    type Error = PasswordError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_creation() {
        let hash_str = "$argon2id$v=19$m=4096,t=3,p=1$...";
        let hash = PasswordHash::try_from(hash_str).unwrap();
        assert_eq!(hash.to_string(), hash_str);

        let result = PasswordHash::try_from("");
        assert!(matches!(result, Err(PasswordError::TooShort)));
    }
}
