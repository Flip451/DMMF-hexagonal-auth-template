use async_trait::async_trait;
use crate::models::user::{Email, UserError, UserRepository};

#[async_trait]
pub trait UserUniquenessChecker: Send + Sync {
    /// 指定されたメールアドレスが既に使用されていないかチェックする。
    /// トランザクション境界内のリポジトリを使用できるよう、引数で受け取る。
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
    use mockall::mock;

    mock! {
        pub UserRepository {}
        #[async_trait]
        impl UserRepository for UserRepository {
            async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserError>;
            async fn save(&self, user: &User) -> Result<(), UserError>;
        }
    }

    #[tokio::test]
    async fn test_check_email_uniqueness_available() {
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_email().returning(|_| Ok(None));

        let checker = UserUniquenessCheckerImpl::new();
        let email = Email::try_from("test@example.com").unwrap();

        let result = checker.check_email_uniqueness(&mock_repo, &email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_email_uniqueness_already_exists() {
        let mut mock_repo = MockUserRepository::new();
        let email = Email::try_from("exists@example.com").unwrap();
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: PasswordHash::try_from("hash").unwrap(),
        };

        mock_repo
            .expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));

        let checker = UserUniquenessCheckerImpl::new();
        let result = checker.check_email_uniqueness(&mock_repo, &email).await;

        assert!(matches!(result, Err(UserError::AlreadyExists)));
    }
}
