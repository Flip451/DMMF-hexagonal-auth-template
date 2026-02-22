use serde::{Deserialize, Serialize};
use usecase::auth::login::dto::LoginResponseDto;
use usecase::auth::login::query::LoginQuery;
use usecase::auth::signup::command::SignupCommand;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

impl From<SignupRequest> for SignupCommand {
    fn from(req: SignupRequest) -> Self {
        Self {
            email: req.email.into(),
            password: req.password.into(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for LoginQuery {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email.into(),
            password: req.password.into(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
    pub token: String,
}

impl From<LoginResponseDto> for LoginResponse {
    fn from(dto: LoginResponseDto) -> Self {
        Self {
            id: dto.id,
            email: dto.email,
            token: dto.token.expose_as_str().to_string(),
        }
    }
}
