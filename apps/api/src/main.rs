use axum::{
    Router,
    routing::{get, post},
};
use domain::models::auth::AuthService;
use domain::models::user::service::UserUniquenessCheckerImpl;
use domain::usecase::auth::{AuthCommandUseCaseImpl, AuthQueryUseCaseImpl};
use infrastructure::auth::jwt::JwtAuthService;
use infrastructure::auth::password::Argon2PasswordService;
use infrastructure::clock::RealClock;
use infrastructure::repository::tx::SqlxTransactionManager;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
pub mod middleware;
mod openapi;
#[cfg(test)]
pub mod tests;

pub struct AppState {
    pub auth_command: Arc<
        AuthCommandUseCaseImpl<
            SqlxTransactionManager<RealClock>,
            UserUniquenessCheckerImpl,
            Argon2PasswordService,
            RealClock,
        >,
    >,
    pub auth_query: Arc<
        AuthQueryUseCaseImpl<SqlxTransactionManager<RealClock>, Argon2PasswordService, RealClock>,
    >,
    pub auth_service: Arc<dyn AuthService>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // JWT Secret
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "debug-secret".to_string());

    // Infrastructure & Domain Services
    let clock = Arc::new(RealClock);
    let tx_manager = Arc::new(SqlxTransactionManager::new(pool, clock.clone()));
    let uniqueness_checker = Arc::new(UserUniquenessCheckerImpl::new());
    let password_service = Arc::new(Argon2PasswordService::new());
    let auth_service: Arc<dyn AuthService> =
        Arc::new(JwtAuthService::new(&jwt_secret, clock.clone()));

    // UseCase instantiation
    let auth_command = Arc::new(AuthCommandUseCaseImpl::new(
        tx_manager.clone(),
        uniqueness_checker,
        password_service.clone(),
        clock.clone(),
    ));
    let auth_query = Arc::new(AuthQueryUseCaseImpl::new(
        tx_manager,
        password_service,
        auth_service.clone(),
        clock,
    ));

    let state = Arc::new(AppState {
        auth_command,
        auth_query,
        auth_service,
    });

    // Router
    let app = Router::new();

    #[cfg(feature = "openapi")]
    let app = {
        use utoipa::OpenApi;
        app.merge(
            utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
        )
    };

    let app = app
        .route("/api/v1/auth/signup", post(handlers::auth::signup::signup))
        .route("/api/v1/auth/login", post(handlers::auth::login::login))
        .route("/api/v1/users/me", get(handlers::users::me::me))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
