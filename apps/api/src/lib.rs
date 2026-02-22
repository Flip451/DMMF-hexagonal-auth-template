use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use usecase::auth::{AuthCommandUseCase, AuthQueryUseCase, AuthService};

pub mod error;
pub mod handlers;
pub mod middleware;
pub mod openapi;
pub mod schemas;
#[cfg(test)]
pub mod tests;

/// Webレイヤーの共有状態。具体的な実装型ではなくインターフェース（Trait）に依存する。
pub struct AppState {
    pub auth_command: Arc<dyn AuthCommandUseCase>,
    pub auth_query: Arc<dyn AuthQueryUseCase>,
    pub auth_service: Arc<dyn AuthService>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let app = Router::new();

    #[cfg(feature = "openapi")]
    let app = {
        use utoipa::OpenApi;
        app.merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
        )
    };

    app.route("/api/v1/auth/signup", post(handlers::auth::signup::signup))
        .route("/api/v1/auth/login", post(handlers::auth::login::login))
        .route("/api/v1/users/me", get(handlers::users::me::me))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
