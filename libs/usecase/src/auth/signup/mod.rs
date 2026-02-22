pub mod command;
pub mod dto;

use async_trait::async_trait;
use std::sync::Arc;

pub use self::command::SignupCommand;
use self::dto::SignupResponseDTO;
use crate::error::UseCaseResult;
use domain::Clock;
use domain::id::IdGenerator;
use domain::models::auth::{PasswordService, RawPassword};
use domain::models::user::{Email, User, UserId, UserUniquenessChecker};
use domain::repository::tx::TransactionManager;

#[async_trait]
pub trait AuthCommandUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<SignupResponseDTO>;
}

pub struct AuthCommandUseCaseImpl<TM, UC, PS, C, IG>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
    C: Clock,
    IG: IdGenerator<UserId>,
{
    transaction_manager: Arc<TM>,
    user_uniqueness_checker: Arc<UC>,
    password_service: Arc<PS>,
    _clock: Arc<C>,
    id_generator: Arc<IG>,
}

impl<TM, UC, PS, C, IG> AuthCommandUseCaseImpl<TM, UC, PS, C, IG>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
    C: Clock,
    IG: IdGenerator<UserId>,
{
    pub fn new(
        transaction_manager: Arc<TM>,
        user_uniqueness_checker: Arc<UC>,
        password_service: Arc<PS>,
        clock: Arc<C>,
        id_generator: Arc<IG>,
    ) -> Self {
        Self {
            transaction_manager,
            user_uniqueness_checker,
            password_service,
            _clock: clock,
            id_generator,
        }
    }
}

#[async_trait]
impl<TM, UC, PS, C, IG> AuthCommandUseCase for AuthCommandUseCaseImpl<TM, UC, PS, C, IG>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker + 'static,
    PS: PasswordService + 'static,
    C: Clock + 'static,
    IG: IdGenerator<UserId> + 'static,
{
    async fn signup(&self, command: SignupCommand) -> UseCaseResult<SignupResponseDTO> {
        let email = Email::try_from(command.email.into_inner())?;

        let checker = Arc::clone(&self.user_uniqueness_checker);
        let password_service = Arc::clone(&self.password_service);
        let id_generator = Arc::clone(&self.id_generator);

        let password_hash = password_service
            .hash(&RawPassword::from(command.password.into_inner()))
            .await?;

        let user = domain::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            checker.check_email_uniqueness(&*user_repo, &email).await?;

            let user = User::new(id_generator.generate(), email, password_hash);

            user_repo.save(&user).await?;

            Ok::<User, domain::error::DomainError>(user)
        })
        .await?;

        Ok(SignupResponseDTO::from(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::test_utils::utils::*;
    use crate::error::UseCaseError;
    use domain::models::user::UserUniquenessViolation;
    use domain::test_utils::{FixedClock, MockIdGenerator};
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_signup_success(
        valid_email: Email,
        valid_password: String,
        valid_password_hash: domain::models::user::PasswordHash,
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
        let id_generator = Arc::new(MockIdGenerator::<UserId>::with_generated_ids(1));
        let expected_id = id_generator.expected_ids()[0];

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps, clock, id_generator);
        let command = SignupCommand {
            email: valid_email.to_string().into(),
            password: valid_password.into(),
        };

        let result = usecase.signup(command).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.email, valid_email.to_string());
        let expected_uuid: uuid::Uuid = expected_id.into();
        assert_eq!(response.id, expected_uuid);
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
                Ok(domain::models::user::PasswordHash::from_str_unchecked(
                    "hashed",
                ))
            }),
        });
        let clock = Arc::new(FixedClock::new(chrono::Utc::now()));
        let id_generator = Arc::new(MockIdGenerator::<UserId>::with_generated_ids(1));

        let usecase = AuthCommandUseCaseImpl::new(tm, checker, ps, clock, id_generator);
        let command = SignupCommand {
            email: valid_email.to_string().into(),
            password: valid_password.into(),
        };

        let result = usecase.signup(command).await;
        assert!(matches!(result, Err(UseCaseError::Conflict(_))));
    }
}
