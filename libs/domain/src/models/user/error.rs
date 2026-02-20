use crate::models::user::Email;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum EmailError {
    #[error("Email is empty")]
    Empty,
    #[error("Email format is invalid")]
    InvalidFormat,
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum PasswordError {
    #[error("Password is too short")]
    TooShort,
    #[error("Password lacks required characters")]
    TooWeak,
    #[error("Invalid password hash format: {found}")]
    InvalidFormat { found: String },
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum UserRepositoryError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Database query failed: {0}")]
    QueryFailed(String),

    #[error("Data mapping failed: {0}")]
    MappingFailed(String),

    #[error("Unknown repository error: {0}")]
    Unknown(String),
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum UserUniquenessViolation {
    #[error("Email already exists: {0}")]
    EmailAlreadyExists(Email),

    #[error("Infrastructure failure during uniqueness check: {0}")]
    Infrastructure(#[from] Box<UserRepositoryError>),
}

#[derive(Debug, Clone, Error)]
pub enum UserError {
    #[error(transparent)]
    Email(#[from] EmailError),

    #[error(transparent)]
    Password(#[from] PasswordError),

    #[error(transparent)]
    Uniqueness(#[from] UserUniquenessViolation),

    #[error(transparent)]
    Repository(#[from] UserRepositoryError),

    #[error("User not found")]
    NotFound,
}
