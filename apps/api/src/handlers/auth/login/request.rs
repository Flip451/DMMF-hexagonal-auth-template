use domain::usecase::auth::login::query::LoginQuery;
use sensitive_data::{EmailRule, SecretRule, Sensitive};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct LoginRequest {
    /// ユーザーのメールアドレス
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub email: Sensitive<String, EmailRule>,
    /// パスワード
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub password: Sensitive<String, SecretRule>,
}

impl From<LoginRequest> for LoginQuery {
    fn from(req: LoginRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}
