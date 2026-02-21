use axum::{routing::post, Router};
use domain::models::user::service::UserUniquenessCheckerImpl;
use domain::usecase::auth::{AuthCommandUseCaseImpl, AuthQueryUseCaseImpl};
use infrastructure::auth::password::Argon2PasswordService;
use infrastructure::repository::tx::SqlxTransactionManager;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

pub struct AppState {
    pub auth_command: Arc<AuthCommandUseCaseImpl<SqlxTransactionManager, UserUniquenessCheckerImpl, Argon2PasswordService>>,
    pub auth_query: Arc<AuthQueryUseCaseImpl<SqlxTransactionManager, Argon2PasswordService>>,
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

    // Infrastructure & Domain Services
    let tx_manager = Arc::new(SqlxTransactionManager::new(pool));
    let uniqueness_checker = Arc::new(UserUniquenessCheckerImpl::new());
    let password_service = Arc::new(Argon2PasswordService::new());

    // UseCase instantiation
    let auth_command = Arc::new(AuthCommandUseCaseImpl::new(
        tx_manager.clone(),
        uniqueness_checker,
        password_service.clone(),
    ));
    let auth_query = Arc::new(AuthQueryUseCaseImpl::new(
        tx_manager,
        password_service,
    ));

    let state = Arc::new(AppState {
        auth_command,
        auth_query,
    });

    // Router
    let app = Router::new()
        .route("/api/v1/auth/signup", post(handlers::auth::signup))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
