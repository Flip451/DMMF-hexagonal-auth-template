use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct MeResponse {
    /// ユーザーID
    pub user_id: String,
}
