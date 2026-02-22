use domain::usecase::auth::signup::command::SignupCommand;
use sensitive_data::{EmailRule, SecretRule, Sensitive};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct SignupRequest {
    /// ユーザーのメールアドレス
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub email: Sensitive<String, EmailRule>,
    /// パスワード
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub password: Sensitive<String, SecretRule>,
}

impl From<SignupRequest> for SignupCommand {
    fn from(req: SignupRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
        }
    }
}
