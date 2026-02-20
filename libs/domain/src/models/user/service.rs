use std::sync::Arc;
use async_trait::async_trait;
use crate::models::user::{Email, UserError, UserRepository};

#[async_trait]
pub trait UserUniquenessChecker: Send + Sync {
    async fn check_email_uniqueness(&self, email: &Email) -> Result<(), UserError>;
}

pub struct UserUniquenessCheckerImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserUniquenessCheckerImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserUniquenessChecker for UserUniquenessCheckerImpl {
    async fn check_email_uniqueness(&self, email: &Email) -> Result<(), UserError> {
        match self.user_repository.find_by_email(email).await? {
            Some(_) => Err(UserError::AlreadyExists),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{User, UserId, PasswordHash};
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
        mock_repo.expect_find_by_email()
            .returning(|_| Ok(None));
        
        let checker = UserUniquenessCheckerImpl::new(Arc::new(mock_repo));
        let email = Email::try_from("test@example.com").unwrap();
        
        let result = checker.check_email_uniqueness(&email).await;
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
        
        mock_repo.expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));
        
        let checker = UserUniquenessCheckerImpl::new(Arc::new(mock_repo));
        let result = checker.check_email_uniqueness(&email).await;
        
        assert!(matches!(result, Err(UserError::AlreadyExists)));
    }
}
