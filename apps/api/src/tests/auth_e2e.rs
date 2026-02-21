#[cfg(test)]
mod e2e_tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use domain::models::user::service::UserUniquenessCheckerImpl;
    use domain::usecase::auth::{AuthCommandUseCaseImpl, AuthQueryUseCaseImpl};
    use infrastructure::auth::jwt::JwtAuthService;
    use infrastructure::auth::password::Argon2PasswordService;
    use infrastructure::repository::tx::SqlxTransactionManager;
    use serde_json::{Value, json};
    use std::sync::Arc;
    use tower::ServiceExt; // for `oneshot`

    use crate::AppState;
    use crate::handlers::auth::login::response::LoginResponse;

    async fn setup_app(pool: sqlx::PgPool) -> axum::Router {
        let tx_manager = Arc::new(SqlxTransactionManager::new(pool));
        let uniqueness_checker = Arc::new(UserUniquenessCheckerImpl::new());
        let password_service = Arc::new(Argon2PasswordService::new());
        let auth_service = Arc::new(JwtAuthService::new("test-secret"));

        let auth_command = Arc::new(AuthCommandUseCaseImpl::new(
            tx_manager.clone(),
            uniqueness_checker,
            password_service.clone(),
        ));
        let auth_query = Arc::new(AuthQueryUseCaseImpl::new(
            tx_manager,
            password_service,
            auth_service.clone(),
        ));

        let state = Arc::new(AppState {
            auth_command,
            auth_query,
            auth_service,
        });

        axum::Router::new()
            .route(
                "/api/v1/auth/signup",
                axum::routing::post(crate::handlers::auth::signup::signup),
            )
            .route(
                "/api/v1/auth/login",
                axum::routing::post(crate::handlers::auth::login::login),
            )
            .route(
                "/api/v1/users/me",
                axum::routing::get(crate::handlers::users::me::me),
            )
            .with_state(state)
    }

    #[sqlx::test(migrations = "../../migrations")]
    async fn test_auth_flow_e2e(pool: sqlx::PgPool) {
        let app = setup_app(pool).await;

        let email = "e2e@example.com";
        let password = "Password123!";

        // 1. Signup
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/auth/signup")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({ "email": email, "password": password }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        // 2. Login
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/auth/login")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({ "email": email, "password": password }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let login_res: LoginResponse = serde_json::from_slice(&body_bytes).unwrap();
        let token = login_res.token;
        assert!(!token.is_empty());

        // 3. Get Me (Success)
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/users/me")
                    .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body["user_id"], login_res.id.to_string());

        // 4. Get Me (Unauthorized - No Header)
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/users/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
