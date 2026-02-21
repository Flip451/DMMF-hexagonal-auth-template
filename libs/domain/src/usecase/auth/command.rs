use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::DomainResult;
use crate::models::auth::PasswordService;
use crate::models::user::{Email, User, UserId, UserUniquenessChecker};
use crate::repository::tx::TransactionManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: Email,
    pub password: String,
}

#[async_trait]
pub trait AuthCommandUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> DomainResult<User>;
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
    async fn signup(&self, command: SignupCommand) -> DomainResult<User> {
        let checker = Arc::clone(&self.user_uniqueness_checker);
        let password_service = Arc::clone(&self.password_service);

        let password_hash = password_service.hash(&command.password).await?;

        crate::tx!(self.transaction_manager, |factory| {
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
}
