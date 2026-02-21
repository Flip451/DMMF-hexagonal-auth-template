pub mod response;

use self::response::MeResponse;
use crate::middleware::auth::AuthenticatedUser;
use axum::Json;

#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    responses(
        (status = 200, description = "User profile retrieved", body = MeResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn me(AuthenticatedUser(claims): AuthenticatedUser) -> Json<MeResponse> {
    Json(MeResponse {
        user_id: claims.sub.to_string(),
    })
}
