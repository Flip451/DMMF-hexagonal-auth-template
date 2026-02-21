#[cfg(feature = "openapi")]
use crate::handlers;
#[cfg(feature = "openapi")]
use utoipa::OpenApi;

#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::signup::signup,
        handlers::auth::login::login,
        handlers::users::me::me,
    ),
    components(
        schemas(
            handlers::auth::signup::request::SignupRequest,
            handlers::auth::login::request::LoginRequest,
            handlers::auth::login::response::LoginResponse,
            handlers::users::me::response::MeResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication and registration"),
        (name = "users", description = "User management and profile")
    )
)]
pub struct ApiDoc;

#[cfg(feature = "openapi")]
struct SecurityAddon;

#[cfg(feature = "openapi")]
impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
