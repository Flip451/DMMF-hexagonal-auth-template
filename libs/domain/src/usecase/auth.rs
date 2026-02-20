use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::DomainResult;
use crate::models::auth::{AuthError, PasswordService};
use crate::models::user::{Email, User, UserId, UserUniquenessChecker};
use crate::repository::tx::TransactionManager;
use crate::tx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: Email,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCommand {
    pub email: Email,
    pub password: String,
}

#[async_trait]
pub trait AuthUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> DomainResult<User>;
    async fn login(&self, command: LoginCommand) -> DomainResult<User>;
}

pub struct AuthUseCaseImpl<TM, UC, PS>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
    PS: PasswordService,
{
    transaction_manager: Arc<TM>,
    user_uniqueness_checker: Arc<UC>,
    password_service: Arc<PS>,
}

impl<TM, UC, PS> AuthUseCaseImpl<TM, UC, PS>
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
impl<TM, UC, PS> AuthUseCase for AuthUseCaseImpl<TM, UC, PS>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker + 'static,
    PS: PasswordService + 'static,
{
    async fn signup(&self, command: SignupCommand) -> DomainResult<User> {
        let checker = Arc::clone(&self.user_uniqueness_checker);
        let password_service = Arc::clone(&self.password_service);

        let password_hash = password_service.hash(&command.password).await?;

        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            checker
                .check_email_uniqueness(&*user_repo, &command.email)
                .await?;

            let user = User {
                id: UserId::new(),
                email: command.email.clone(),
                password_hash,
            };

            user_repo.save(&user).await?;

            Ok(user)
        })
        .await
    }

    async fn login(&self, command: LoginCommand) -> DomainResult<User> {
        let password_service = Arc::clone(&self.password_service);

        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let user = user_repo
                .find_by_email(&command.email)
                .await?
                .ok_or(AuthError::InvalidCredentials)?;

            let is_valid = password_service
                .verify(&command.password, &user.password_hash)
                .await?;

            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }

            Ok(user)
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::error::UserUniquenessViolation;
    use crate::models::user::{PasswordHash, UserRepository, UserRepositoryError};
    use crate::repository::tx::{IntoTxError, RepositoryFactory};
    use futures_util::future::BoxFuture;
    use rstest::*;
    use std::fmt::Debug;

    // --- Stubs ---

    pub struct StubUserRepository {
        pub find_result: Result<Option<User>, UserRepositoryError>,
        pub save_result: Result<(), UserRepositoryError>,
    }
    #[async_trait]
    impl UserRepository for StubUserRepository {
        async fn find_by_email(&self, _email: &Email) -> Result<Option<User>, UserRepositoryError> {
            self.find_result.clone()
        }
        async fn save(&self, _user: &User) -> Result<(), UserRepositoryError> {
            self.save_result.clone()
        }
    }

    pub struct StubRepositoryFactory {
        pub repo: Arc<StubUserRepository>,
    }
    impl RepositoryFactory for StubRepositoryFactory {
        fn user_repository(&self) -> Arc<dyn UserRepository> {
            self.repo.clone()
        }
    }

    pub struct StubTransactionManager {
        pub factory: Arc<StubRepositoryFactory>,
    }
    #[async_trait]
    impl TransactionManager for StubTransactionManager {
        async fn execute<T, E, F>(&self, f: F) -> Result<T, E>
        where
            T: Send,
            E: IntoTxError + Debug + Send + Sync,
            F: for<'a> FnOnce(&'a dyn RepositoryFactory) -> BoxFuture<'a, Result<T, E>> + Send,
        {
            f(&*self.factory).await
        }
    }

    pub struct StubUserUniquenessChecker {
        pub result: Result<(), UserUniquenessViolation>,
    }
    #[async_trait]
    impl UserUniquenessChecker for StubUserUniquenessChecker {
        async fn check_email_uniqueness(
            &self,
            _repo: &dyn UserRepository,
            _email: &Email,
        ) -> Result<(), UserUniquenessViolation> {
            self.result.clone()
        }
    }

    pub struct StubPasswordService {
        pub verify_result: Result<bool, AuthError>,
        pub hash_result: Result<PasswordHash, AuthError>,
    }
    #[async_trait]
    impl PasswordService for StubPasswordService {
        async fn verify(&self, _pw: &str, _hash: &PasswordHash) -> Result<bool, AuthError> {
            self.verify_result.clone()
        }
        async fn hash(&self, _pw: &str) -> Result<PasswordHash, AuthError> {
            self.hash_result.clone()
        }
    }

    // --- Fixtures ---

    #[fixture]
    fn email() -> Email {
        Email::try_from("test@example.com").unwrap()
    }

    #[fixture]
    fn password() -> String {
        "password123".to_string()
    }

    #[fixture]
    fn password_hash() -> PasswordHash {
        PasswordHash::from_str_unchecked("hashed_password")
    }

    // --- Tests ---

    #[rstest]
    #[tokio::test]
    async fn test_signup_success(email: Email, password: String, password_hash: PasswordHash) {
        let repo = Arc::new(StubUserRepository {
            find_result: Ok(None),
            save_result: Ok(()),
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker { result: Ok(()) });
        let ps = Arc::new(StubPasswordService {
            verify_result: Ok(true),
            hash_result: Ok(password_hash),
        });

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let command = SignupCommand {
            email: email.clone(),
            password,
        };

        let result = usecase.signup(command).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, email);
    }

    #[rstest]
    #[tokio::test]
    async fn test_signup_duplicate_email(email: Email, password: String) {
        let repo = Arc::new(StubUserRepository {
            find_result: Ok(None),
            save_result: Ok(()),
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        // 一意性チェックで失敗
        let checker = Arc::new(StubUserUniquenessChecker {
            result: Err(UserUniquenessViolation::EmailAlreadyExists(email.clone())),
        });
        let ps = Arc::new(StubPasswordService {
            verify_result: Ok(true),
            hash_result: Ok(PasswordHash::from_str_unchecked("hashed")),
        });

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let command = SignupCommand {
            email: email.clone(),
            password,
        };

        let result = usecase.signup(command).await;
        assert!(matches!(
            result,
            Err(crate::error::DomainError::User(
                crate::models::user::error::UserError::Uniqueness(
                    UserUniquenessViolation::EmailAlreadyExists(_)
                )
            ))
        ));
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_success(email: Email, password: String, password_hash: PasswordHash) {
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: password_hash.clone(),
        };
        let repo = Arc::new(StubUserRepository {
            find_result: Ok(Some(user)),
            save_result: Ok(()),
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker { result: Ok(()) });
        let ps = Arc::new(StubPasswordService {
            verify_result: Ok(true),
            hash_result: Ok(password_hash),
        });

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let result = usecase.login(LoginCommand { email, password }).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_invalid_credentials(email: Email, password: String) {
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: PasswordHash::from_str_unchecked("hashed"),
        };
        let repo = Arc::new(StubUserRepository {
            find_result: Ok(Some(user)),
            save_result: Ok(()),
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker { result: Ok(()) });
        let ps = Arc::new(StubPasswordService {
            verify_result: Ok(false), // パスワード不一致
            hash_result: Ok(PasswordHash::from_str_unchecked("hashed")),
        });

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let result = usecase.login(LoginCommand { email, password }).await;

        assert!(matches!(
            result,
            Err(crate::error::DomainError::Auth(
                AuthError::InvalidCredentials
            ))
        ));
    }
}
