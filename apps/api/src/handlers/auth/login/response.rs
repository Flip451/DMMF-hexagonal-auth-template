use domain::usecase::auth::login::dto::LoginResponseDTO;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginResponse {
    /// ユーザーID (UUID)
    pub id: Uuid,
    /// メールアドレス
    pub email: String,
    /// アクセストークン (JWT)
    pub token: String,
}

impl From<LoginResponseDTO> for LoginResponse {
    fn from(dto: LoginResponseDTO) -> Self {
        Self {
            id: dto.id,
            email: dto.email,
            token: dto.token,
        }
    }
}
