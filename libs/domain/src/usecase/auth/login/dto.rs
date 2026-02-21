use crate::models::user::{User, UserIdentity};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub id: Uuid,
    pub email: String,
    pub token: String,
}

impl LoginResponseDTO {
    pub fn new(user: &User, token: String) -> Self {
        Self {
            id: user.id().into(),
            email: user.email().to_string(),
            token,
        }
    }
}
