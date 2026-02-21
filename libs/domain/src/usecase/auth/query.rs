use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::auth::{AuthError, PasswordService};
use crate::models::user::{Authenticatable, Email, User};
use crate::repository::tx::TransactionManager;
use crate::usecase::error::UseCaseResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginQuery {
    pub email: Email,
    pub password: String,
}

#[async_trait]
pub trait AuthQueryUseCase: Send + Sync {
    async fn login(&self, query: LoginQuery) -> UseCaseResult<User>;
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
    async fn login(&self, query: LoginQuery) -> UseCaseResult<User> {
        let password_service = Arc::clone(&self.password_service);

        let user = crate::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let user = user_repo
                .find_by_email(&query.email)
                .await?
                .ok_or(AuthError::InvalidCredentials)?;

            let is_valid = password_service
                .verify(&query.password, user.password_hash())
                .await?;

            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }

            Ok::<User, crate::error::DomainError>(user)
        })
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserId;
    use crate::usecase::auth::test_utils::utils::*;
    use crate::usecase::error::UseCaseError;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_login_success(
        valid_email: Email,
        valid_password: String,
        valid_password_hash: crate::models::user::PasswordHash,
    ) {
        let user = User::new(
            UserId::new(),
            valid_email.clone(),
            valid_password_hash.clone(),
        );
        let repo = Arc::new(StubUserRepository {
            found_user: Some(user),
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(true)),
            hash_result: Arc::new(move || Ok(valid_password_hash.clone())),
        });

        let usecase = AuthQueryUseCaseImpl::new(tm, ps);
        let result = usecase
            .login(LoginQuery {
                email: valid_email,
                password: valid_password,
            })
            .await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_invalid_credentials(valid_email: Email, valid_password: String) {
        let user = User::new(
            UserId::new(),
            valid_email.clone(),
            crate::models::user::PasswordHash::from_str_unchecked("hashed"),
        );
        let repo = Arc::new(StubUserRepository {
            found_user: Some(user),
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(false)), // Password mismatch
            hash_result: Arc::new(|| {
                Ok(crate::models::user::PasswordHash::from_str_unchecked(
                    "hashed",
                ))
            }),
        });

        let usecase = AuthQueryUseCaseImpl::new(tm, ps);
        let result = usecase
            .login(LoginQuery {
                email: valid_email,
                password: valid_password,
            })
            .await;

        assert!(matches!(result, Err(UseCaseError::Authentication(_))));
    }
}
