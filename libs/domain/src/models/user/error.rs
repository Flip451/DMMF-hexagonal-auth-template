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
}

#[derive(Debug, Clone, Error)]
pub enum UserError {
    #[error(transparent)]
    Email(#[from] EmailError),

    #[error(transparent)]
    Password(#[from] PasswordError),

    #[error("User not found")]
    NotFound,

    #[error("User already exists")]
    AlreadyExists,
}
