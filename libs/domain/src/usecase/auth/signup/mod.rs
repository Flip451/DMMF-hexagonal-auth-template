pub mod command;
pub mod dto;

use async_trait::async_trait;
use std::sync::Arc;

use self::command::SignupCommand;
use self::dto::SignupResponseDTO;
use crate::clock::Clock;
use crate::models::auth::PasswordService;
use crate::models::user::{Email, User, UserId, UserUniquenessChecker};
use crate::repository::tx::TransactionManager;
use crate::usecase::error::UseCaseResult;

#[async_trait]
pub trait AuthCommandUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<SignupResponseDTO>;
}

pub struct AuthCommandUseCaseImpl<TM, UC, PS, C>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
    C: Clock,
{
    transaction_manager: Arc<TM>,
    user_uniqueness_checker: Arc<UC>,
    password_service: Arc<PS>,
    _clock: Arc<C>,
}

impl<TM, UC, PS, C> AuthCommandUseCaseImpl<TM, UC, PS, C>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
    C: Clock,
{
    pub fn new(
        transaction_manager: Arc<TM>,
        user_uniqueness_checker: Arc<UC>,
        password_service: Arc<PS>,
        clock: Arc<C>,
    ) -> Self {
        Self {
            transaction_manager,
            user_uniqueness_checker,
            password_service,
            _clock: clock,
        }
    }
}

#[async_trait]
impl<TM, UC, PS, C> AuthCommandUseCase for AuthCommandUseCaseImpl<TM, UC, PS, C>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker + 'static,
    PS: PasswordService + 'static,
    C: Clock + 'static,
{
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<SignupResponseDTO> {
        let email = Email::try_from(command.email)?;

        let checker = Arc::clone(&self.user_uniqueness_checker);
        let password_service = Arc::clone(&self.password_service);

        let password_hash = password_service.hash(&command.password).await?;

        let user = crate::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            checker.check_email_uniqueness(&*user_repo, &email).await?;

            let user = User::new(UserId::new(), email, password_hash);

            user_repo.save(&user).await?;

            Ok::<User, crate::error::DomainError>(user)
        })
        .await?;

        Ok(SignupResponseDTO::from(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserUniquenessViolation;
    use crate::test_utils::FixedClock;
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
        let clock = Arc::new(FixedClock::new(chrono::Utc::now()));

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps, clock);
        let command = SignupCommand {
            email: valid_email.to_string(),
            password: valid_password,
        };

        let result = usecase.signup(command).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, valid_email.to_string());
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
        let clock = Arc::new(FixedClock::new(chrono::Utc::now()));

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps, clock);
        let command = SignupCommand {
            email: valid_email.to_string(),
            password: valid_password,
        };

        let result = usecase.signup(command).await;
        assert!(matches!(result, Err(UseCaseError::Conflict(_))));
    }
}
