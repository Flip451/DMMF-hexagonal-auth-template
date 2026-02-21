use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::DomainResult;
use crate::models::auth::{AuthError, PasswordService};
use crate::models::user::{Email, User};
use crate::repository::tx::TransactionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginQuery {
    pub email: Email,
    pub password: String,
}

#[async_trait]
pub trait AuthQueryUseCase: Send + Sync {
    async fn login(&self, query: LoginQuery) -> DomainResult<User>;
}

pub struct AuthQueryUseCaseImpl<TM, PS>
where
    TM: TransactionManager,
    PS: PasswordService,
{
    transaction_manager: Arc<TM>,
    password_service: Arc<PS>,
}

impl<TM, PS> AuthQueryUseCaseImpl<TM, PS>
where
    TM: TransactionManager,
    PS: PasswordService,
{
    pub fn new(transaction_manager: Arc<TM>, password_service: Arc<PS>) -> Self {
        Self {
            transaction_manager,
            password_service,
        }
    }
}

#[async_trait]
impl<TM, PS> AuthQueryUseCase for AuthQueryUseCaseImpl<TM, PS>
where
    TM: TransactionManager,
    PS: PasswordService + 'static,
{
    async fn login(&self, query: LoginQuery) -> DomainResult<User> {
        let password_service = Arc::clone(&self.password_service);

        crate::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let user = user_repo
                .find_by_email(&query.email)
                .await?
                .ok_or(AuthError::InvalidCredentials)?;

            let is_valid = password_service
                .verify(&query.password, &user.password_hash)
                .await?;

            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }

            Ok(user)
        })
        .await
    }
}
