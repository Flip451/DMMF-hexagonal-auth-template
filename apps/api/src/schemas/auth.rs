use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use domain::usecase::auth::signup::command::SignupCommand;
use domain::usecase::auth::login::query::LoginQuery;
use domain::usecase::auth::login::dto::LoginResponseDTO;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

impl From<SignupRequest> for SignupCommand {
    fn from(req: SignupRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
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
            email: req.email,
            password: req.password,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub id: Uuid,
    pub email: String,
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
