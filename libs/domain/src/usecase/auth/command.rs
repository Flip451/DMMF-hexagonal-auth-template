use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::auth::PasswordService;
use crate::models::user::{Email, User, UserId, UserUniquenessChecker};
use crate::repository::tx::TransactionManager;
use crate::usecase::error::UseCaseResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: Email,
    pub password: String,
}

#[async_trait]
pub trait AuthCommandUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<User>;
}

pub struct AuthCommandUseCaseImpl<TM, UC, PS>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
{
    transaction_manager: Arc<TM>,
    user_uniqueness_checker: Arc<UC>,
    password_service: Arc<PS>,
}

impl<TM, UC, PS> AuthCommandUseCaseImpl<TM, UC, PS>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
{
    pub fn new(
        transaction_manager: Arc<TM>,
        user_uniqueness_checker: Arc<UC>,
        password_service: Arc<PS>,
    ) -> Self {
        Self {
            transaction_manager,
            user_uniqueness_checker,
            password_service,
        }
    }
}

#[async_trait]
impl<TM, UC, PS> AuthCommandUseCase for AuthCommandUseCaseImpl<TM, UC, PS>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker + 'static,
    PS: PasswordService + 'static,
{
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<User> {
        let checker = Arc::clone(&self.user_uniqueness_checker);
        let password_service = Arc::clone(&self.password_service);

        let password_hash = password_service.hash(&command.password).await?;

        let user = crate::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            checker
                .check_email_uniqueness(&*user_repo, &command.email)
                .await?;

            let user = User::new(UserId::new(), command.email.clone(), password_hash);

            user_repo.save(&user).await?;

            Ok::<User, crate::error::DomainError>(user)
        })
        .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{UserIdentity, UserUniquenessViolation};
    use crate::usecase::auth::test_utils::utils::*;
    use crate::usecase::error::UseCaseError;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_signup_success(
        valid_email: Email,
        valid_password: String,
        valid_password_hash: crate::models::user::PasswordHash,
    ) {
        let repo = Arc::new(StubUserRepository {
            found_user: None,
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker {
            error_factory: None,
        });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(true)),
            hash_result: Arc::new(move || Ok(valid_password_hash.clone())),
        });

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps);
        let command = SignupCommand {
            email: valid_email.clone(),
            password: valid_password,
        };

        let result = usecase.signup(command).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email(), &valid_email);
    }

    #[rstest]
    #[tokio::test]
    async fn test_signup_duplicate_email(valid_email: Email, valid_password: String) {
        let repo = Arc::new(StubUserRepository {
            found_user: None,
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker {
            error_factory: Some(|| {
                UserUniquenessViolation::EmailAlreadyExists(
                    Email::try_from("test@example.com").unwrap(),
                )
            }),
        });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(true)),
            hash_result: Arc::new(|| {
                Ok(crate::models::user::PasswordHash::from_str_unchecked(
                    "hashed",
                ))
            }),
        });

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps);
        let command = SignupCommand {
            email: valid_email.clone(),
            password: valid_password,
        };

        let result = usecase.signup(command).await;
        assert!(matches!(result, Err(UseCaseError::Conflict(_))));
    }
}
