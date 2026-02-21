use domain::usecase::auth::signup::command::SignupCommand;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignupRequest {
    /// メールアドレス
    pub email: String,
    /// パスワード
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
