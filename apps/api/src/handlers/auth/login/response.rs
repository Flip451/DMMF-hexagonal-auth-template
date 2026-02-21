use domain::usecase::auth::login::dto::LoginResponseDTO;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
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
