use crate::models::user::{Email, UserError, UserRepository};
use async_trait::async_trait;

#[async_trait]
pub trait UserUniquenessChecker: Send + Sync {
    /// 指定されたメールアドレスが既に使用されていないかチェックする。
    async fn check_email_uniqueness(
        &self,
        user_repository: &dyn UserRepository,
        email: &Email,
    ) -> Result<(), UserError>;
}

pub struct UserUniquenessCheckerImpl;

impl UserUniquenessCheckerImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserUniquenessChecker for UserUniquenessCheckerImpl {
    async fn check_email_uniqueness(
        &self,
        user_repository: &dyn UserRepository,
        email: &Email,
    ) -> Result<(), UserError> {
        match user_repository.find_by_email(email).await? {
            Some(_) => Err(UserError::AlreadyExists),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{PasswordHash, User, UserId};
    use rstest::*;

    pub struct StubUserRepository {
        pub find_result: Result<Option<User>, UserError>,
    }

    #[async_trait]
    impl UserRepository for StubUserRepository {
        async fn find_by_email(&self, _email: &Email) -> Result<Option<User>, UserError> {
            self.find_result.clone()
        }
        async fn save(&self, _user: &User) -> Result<(), UserError> {
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
        let repo = StubUserRepository {
            find_result: Ok(None),
        };
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
            find_result: Ok(Some(user)),
        };

        let result = checker.check_email_uniqueness(&repo, &email).await;
        assert!(matches!(result, Err(UserError::AlreadyExists)));
    }
}
