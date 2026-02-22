use crate::models::user::{User, UserIdentity};
use crate::usecase::auth::AuthToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub id: uuid::Uuid,
    pub email: String,
    pub token: AuthToken,
}

impl LoginResponseDto {
    pub fn new(user: &User, token: AuthToken) -> Self {
        Self {
            id: user.id().into(),
            email: user.email().to_string(),
            token,
        }
    }
}
