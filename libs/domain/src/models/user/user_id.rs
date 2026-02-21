use derive_more::{AsRef, Display, From, Into};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From,
    Into,
    AsRef,
    Display,
    Default,
)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_creation() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::from(uuid);
        let back: Uuid = user_id.into();
        assert_eq!(uuid, back);
    }

    #[test]
    fn test_user_id_display() {
        let uuid = Uuid::new_v4();
        let user_id = UserId::from(uuid);
        assert_eq!(user_id.to_string(), uuid.to_string());
    }
}
