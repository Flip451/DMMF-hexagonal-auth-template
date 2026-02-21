use crate::models::user::{Email, UserRepository, UserRepositoryError};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserUniquenessViolation {
    #[error("Email already exists: {0}")]
    EmailAlreadyExists(Email),

    #[error("Infrastructure failure during uniqueness check: {0}")]
    Infrastructure(#[from] Box<UserRepositoryError>),
}

#[async_trait]
pub trait UserUniquenessChecker: Send + Sync {
    /// 指定されたメールアドレスが既に使用されていないかチェックする。
    async fn check_email_uniqueness(
        &self,
        user_repository: &dyn UserRepository,
        email: &Email,
    ) -> Result<(), UserUniquenessViolation>;
}

pub struct UserUniquenessCheckerImpl;

impl UserUniquenessCheckerImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UserUniquenessCheckerImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserUniquenessChecker for UserUniquenessCheckerImpl {
    async fn check_email_uniqueness(
        &self,
        user_repository: &dyn UserRepository,
        email: &Email,
    ) -> Result<(), UserUniquenessViolation> {
        match user_repository.find_by_email(email).await {
            Ok(Some(_)) => Err(UserUniquenessViolation::EmailAlreadyExists(email.clone())),
            Ok(None) => Ok(()),
            Err(e) => Err(UserUniquenessViolation::Infrastructure(Box::new(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{PasswordHash, User, UserId};
    use rstest::*;

    pub struct StubUserRepository {
        // テスト用なので固定の成功値を返すように単純化
        pub found_user: Option<User>,
    }

    #[async_trait]
    impl UserRepository for StubUserRepository {
        async fn find_by_email(&self, _email: &Email) -> Result<Option<User>, UserRepositoryError> {
            Ok(self.found_user.clone())
        }
        async fn save(&self, _user: &User) -> Result<(), UserRepositoryError> {
            Ok(())
        }
    }

    #[fixture]
    fn checker() -> UserUniquenessCheckerImpl {
        UserUniquenessCheckerImpl::new()
    }

    #[fixture]
    fn email() -> Email {
        Email::try_from("test@example.com").unwrap()
    }

    #[rstest]
    #[tokio::test]
    async fn test_check_email_uniqueness_available(
        checker: UserUniquenessCheckerImpl,
        email: Email,
    ) {
        let repo = StubUserRepository { found_user: None };
        let result = checker.check_email_uniqueness(&repo, &email).await;
        assert!(result.is_ok());
    }

    #[rstest]
    #[tokio::test]
    async fn test_check_email_uniqueness_already_exists(
        checker: UserUniquenessCheckerImpl,
        email: Email,
    ) {
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: PasswordHash::from_str_unchecked("hash"),
        };
        let repo = StubUserRepository {
            found_user: Some(user),
        };

        let result = checker.check_email_uniqueness(&repo, &email).await;
        assert!(matches!(
            result,
            Err(UserUniquenessViolation::EmailAlreadyExists(_))
        ));
    }
}
