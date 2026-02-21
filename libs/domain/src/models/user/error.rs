use crate::models::user::{
    EmailError, PasswordError, UserRepositoryError, UserUniquenessViolation,
};
use thiserror::Error;

#[derive(Debug, Error)]
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
