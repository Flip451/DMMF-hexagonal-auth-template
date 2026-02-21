use domain::usecase::auth::login::query::LoginQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginRequest {
    /// メールアドレス
    pub email: String,
    /// パスワード
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
