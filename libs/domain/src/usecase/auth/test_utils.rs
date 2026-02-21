#[cfg(test)]
pub mod utils {
    use crate::models::auth::{AuthError, PasswordService};
    use crate::models::user::{
        Email, PasswordHash, User, UserRepository, UserRepositoryError, UserUniquenessChecker,
        UserUniquenessViolation,
    };
    use crate::repository::tx::{IntoTxError, RepositoryFactory, TransactionManager};
    use async_trait::async_trait;
    use futures_util::future::BoxFuture;
    use rstest::*;
    use std::fmt::Debug;
    use std::sync::Arc;

    // --- Stubs ---

    pub struct StubUserRepository {
        pub found_user: Option<User>,
        pub save_error: Option<fn() -> UserRepositoryError>,
    }
    #[async_trait]
    impl UserRepository for StubUserRepository {
        async fn find_by_email(&self, _email: &Email) -> Result<Option<User>, UserRepositoryError> {
            Ok(self.found_user.clone())
        }
        async fn save(&self, _user: &User) -> Result<(), UserRepositoryError> {
            if let Some(err_fn) = self.save_error {
                Err(err_fn())
            } else {
                Ok(())
            }
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
        pub error_factory: Option<fn() -> UserUniquenessViolation>,
    }
    #[async_trait]
    impl UserUniquenessChecker for StubUserUniquenessChecker {
        async fn check_email_uniqueness(
            &self,
            _repo: &dyn UserRepository,
            _email: &Email,
        ) -> Result<(), UserUniquenessViolation> {
            if let Some(f) = self.error_factory {
                Err(f())
            } else {
                Ok(())
            }
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
    pub fn valid_email() -> Email {
        Email::try_from("test@example.com").unwrap()
    }

    #[fixture]
    pub fn valid_password() -> String {
        "password123".to_string()
    }

    #[fixture]
    pub fn valid_password_hash() -> PasswordHash {
        PasswordHash::from_str_unchecked("hashed_password")
    }
}
