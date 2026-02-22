use domain::models::user::{User, UserIdentity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupResponseDTO {
    pub id: Uuid,
    pub email: String,
}

impl From<User> for SignupResponseDTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id().into(),
            email: user.email().to_string(),
        }
    }
}
