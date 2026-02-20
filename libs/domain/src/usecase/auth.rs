use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::DomainResult;
use crate::models::user::{Email, PasswordHash, User, UserId, UserUniquenessChecker};
use crate::repository::tx::TransactionManager;
use crate::tx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupCommand {
    pub email: Email,
    pub password_hash: PasswordHash,
}

#[async_trait]
pub trait AuthUseCase: Send + Sync {
    async fn signup(&self, command: SignupCommand) -> DomainResult<User>;
}

pub struct AuthUseCaseImpl<TM, UC>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
{
    transaction_manager: Arc<TM>,
    user_uniqueness_checker: Arc<UC>,
}

impl<TM, UC> AuthUseCaseImpl<TM, UC>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker,
{
    pub fn new(transaction_manager: Arc<TM>, user_uniqueness_checker: Arc<UC>) -> Self {
        Self {
            transaction_manager,
            user_uniqueness_checker,
        }
    }
}

#[async_trait]
impl<TM, UC> AuthUseCase for AuthUseCaseImpl<TM, UC>
where
    TM: TransactionManager,
    UC: UserUniquenessChecker + 'static, // クロージャにmoveするため 'static を要求
{
    async fn signup(&self, command: SignupCommand) -> DomainResult<User> {
        let checker = Arc::clone(&self.user_uniqueness_checker);
        
        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            // トランザクション境界内で一意性チェックを実行
            checker
                .check_email_uniqueness(&*user_repo, &command.email)
                .await?;

            let user = User {
                id: UserId::new(),
                email: command.email.clone(),
                password_hash: command.password_hash.clone(),
            };

            user_repo.save(&user).await?;

            Ok(user)
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    // ユニットテストは Mockall とジェネリクスの組み合わせにより複雑になるため、
    // ここではコンパイルが通ることを優先し、将来的に統合テスト等でカバーします。
}
