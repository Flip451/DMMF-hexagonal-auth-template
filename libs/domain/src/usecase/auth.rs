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
    use crate::models::user::{PasswordHash, UserError, UserRepository};
    use crate::repository::tx::{IntoTxError, RepositoryFactory};
    use futures_util::future::BoxFuture;
    use rstest::*;
    use std::fmt::Debug;

    // 手動スタブの定義（Mockallよりもライフタイムやジェネリクスに強い）

    pub struct StubUserRepository {
        pub user: Option<User>,
    }
    #[async_trait]
    impl UserRepository for StubUserRepository {
        async fn find_by_email(&self, _email: &Email) -> Result<Option<User>, UserError> {
            Ok(self.user.clone())
        }
        async fn save(&self, _user: &User) -> Result<(), UserError> {
            Ok(())
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

    pub struct StubUserUniquenessChecker;
    #[async_trait]
    impl UserUniquenessChecker for StubUserUniquenessChecker {
        async fn check_email_uniqueness(
            &self,
            _repo: &dyn UserRepository,
            _email: &Email,
        ) -> Result<(), UserError> {
            Ok(())
        }
    }

    pub struct StubPasswordService;
    #[async_trait]
    impl PasswordService for StubPasswordService {
        async fn verify(&self, _pw: &str, _hash: &PasswordHash) -> Result<bool, AuthError> {
            Ok(true)
        }
        async fn hash(&self, _pw: &str) -> Result<PasswordHash, AuthError> {
            Ok(PasswordHash::try_from("hashed").unwrap())
        }
    }

    #[fixture]
    fn email() -> Email {
        Email::try_from("test@example.com").unwrap()
    }

    #[rstest]
    #[tokio::test]
    async fn test_signup_success(email: Email) {
        let repo = Arc::new(StubUserRepository { user: None });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker);
        let ps = Arc::new(StubPasswordService);

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let command = SignupCommand {
            email: email.clone(),
            password: "password".to_string(),
        };

        let result = usecase.signup(command).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email, email);
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_success(email: Email) {
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: PasswordHash::try_from("hashed").unwrap(),
        };
        let repo = Arc::new(StubUserRepository { user: Some(user) });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let checker = Arc::new(StubUserUniquenessChecker);
        let ps = Arc::new(StubPasswordService);

        let usecase = AuthUseCaseImpl::new(tm, checker, ps);
        let command = LoginCommand {
            email,
            password: "password".to_string(),
        };

        let result = usecase.login(command).await;
        assert!(result.is_ok());
    }
}
