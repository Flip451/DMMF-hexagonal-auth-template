use domain::usecase::auth::login::dto::LoginResponseDto;
use sensitive_data::{EmailRule, SecretRule, Sensitive};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginResponse {
    /// ユーザーID (UUID)
    pub id: Uuid,
    /// メールアドレス
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub email: Sensitive<String, EmailRule>,
    /// アクセストークン (JWT)
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub token: Sensitive<String, SecretRule>,
}

impl From<LoginResponseDto> for LoginResponse {
    fn from(dto: LoginResponseDto) -> Self {
        Self {
            id: dto.id,
            email: dto.email.into(),
            token: dto.token.expose_as_str().to_string().into(),
        }
    }
}
