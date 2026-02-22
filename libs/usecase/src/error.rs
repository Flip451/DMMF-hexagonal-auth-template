use domain::error::DomainError;
use domain::models::auth::PasswordServiceError;
use domain::models::auth::error::AuthError;
use domain::models::user::{
    EmailError, PasswordError, UserError, UserRepositoryError, UserUniquenessViolation,
};
use thiserror::Error;

/// ユースケース層のエラー。API層に対する保護層として機能し、
/// 内部のドメインやインフラの詳細を意味的なバリアントに変換して隠蔽します。
#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Access denied: {0}")]
    Forbidden(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal system error")]
    Internal(#[from] anyhow::Error),
}

pub type UseCaseResult<T> = Result<T, UseCaseError>;

/// 認証サービス（トークン発行・検証等）に関連するエラー。
/// ユースケース層のポート（AuthService）で使用されます。
#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Failed to issue token: {0}")]
    IssuanceFailed(#[source] anyhow::Error),

    #[error("Failed to verify token: {0}")]
    VerificationFailed(#[source] anyhow::Error),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,
}

// --- 階層的な From 実装 (カプセル化の維持) ---

impl From<DomainError> for UseCaseError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::User(e) => e.into(),
            DomainError::Auth(e) => e.into(),
            DomainError::Infrastructure(e) => UseCaseError::Internal(e),
            DomainError::LogicViolation(msg) => {
                UseCaseError::Internal(anyhow::anyhow!("Logic violation: {}", msg))
            }
        }
    }
}

impl From<UserError> for UseCaseError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::Email(e) => e.into(),
            UserError::Password(e) => e.into(),
            UserError::Uniqueness(e) => e.into(),
            UserError::Repository(e) => e.into(),
            UserError::NotFound => UseCaseError::NotFound("User not found".into()),
        }
    }
}

impl From<EmailError> for UseCaseError {
    fn from(error: EmailError) -> Self {
        UseCaseError::InvalidInput(format!("Invalid email: {}", error))
    }
}

impl From<PasswordError> for UseCaseError {
    fn from(error: PasswordError) -> Self {
        UseCaseError::InvalidInput(format!("Invalid password: {}", error))
    }
}

impl From<UserUniquenessViolation> for UseCaseError {
    fn from(error: UserUniquenessViolation) -> Self {
        match error {
            UserUniquenessViolation::EmailAlreadyExists(email) => {
                UseCaseError::Conflict(format!("Email '{}' already exists", email))
            }
            UserUniquenessViolation::Infrastructure(e) => (*e).into(),
        }
    }
}

impl From<UserRepositoryError> for UseCaseError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::ConnectionFailed => {
                UseCaseError::Internal(anyhow::anyhow!("Database connection failed"))
            }
            UserRepositoryError::QueryFailed(e) => UseCaseError::Internal(e),
            UserRepositoryError::MappingFailed(e) => UseCaseError::Internal(e),
            UserRepositoryError::Unexpected(e) => UseCaseError::Internal(e),
        }
    }
}

impl From<AuthError> for UseCaseError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InvalidCredentials => {
                UseCaseError::Authentication("Invalid credentials".into())
            }
            AuthError::Forbidden => {
                UseCaseError::Forbidden("Access denied: insufficient permissions".into())
            }
            AuthError::PasswordService(e) => e.into(),
        }
    }
}

impl From<AuthServiceError> for UseCaseError {
    fn from(error: AuthServiceError) -> Self {
        match error {
            AuthServiceError::IssuanceFailed(e) => UseCaseError::Internal(e),
            AuthServiceError::VerificationFailed(e) => UseCaseError::Internal(e),
            AuthServiceError::TokenExpired => {
                UseCaseError::Authentication("Token has expired".into())
            }
            AuthServiceError::InvalidToken => UseCaseError::Authentication("Invalid token".into()),
        }
    }
}

impl From<PasswordServiceError> for UseCaseError {
    fn from(error: PasswordServiceError) -> Self {
        match error {
            PasswordServiceError::HashingFailed(e) => UseCaseError::Internal(e),
            PasswordServiceError::VerificationFailed(e) => UseCaseError::Internal(e),
        }
    }
}
