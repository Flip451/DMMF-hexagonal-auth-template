use api::{AppState, create_router};
use domain::models::user::service::UserUniquenessCheckerImpl;
use infrastructure::auth::jwt::JwtAuthService;
use infrastructure::auth::password::Argon2PasswordService;
use infrastructure::clock::RealClock;
use infrastructure::id::UuidV7Generator;
use infrastructure::repository::tx::SqlxTransactionManager;
use infrastructure::telemetry::init_telemetry;
use sensitive_data::MaskingControl;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use usecase::auth::{AuthCommandUseCaseImpl, AuthQueryUseCaseImpl};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize telemetry
    init_telemetry("server");

    // Configure masking control
    let mask_enabled = env::var("MASK_SENSITIVE_DATA")
        .map(|v| v.to_lowercase() != "false")
        .unwrap_or(true);
    MaskingControl::set_enabled(mask_enabled);

    // Database connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // JWT Secret
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Infrastructure & Domain Services
    let clock = Arc::new(RealClock);
    let id_generator = Arc::new(UuidV7Generator::new());
    let tx_manager = Arc::new(SqlxTransactionManager::new(pool, clock.clone()));
    let uniqueness_checker = Arc::new(UserUniquenessCheckerImpl::new());
    let password_service = Arc::new(Argon2PasswordService::new());
    let auth_service = Arc::new(JwtAuthService::new(&jwt_secret, clock.clone()));

    // UseCase instantiation (Implementations from infrastructure/domain are injected here)
    let auth_command = Arc::new(AuthCommandUseCaseImpl::new(
        tx_manager.clone(),
        uniqueness_checker,
        password_service.clone(),
        clock.clone(),
        id_generator,
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

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
