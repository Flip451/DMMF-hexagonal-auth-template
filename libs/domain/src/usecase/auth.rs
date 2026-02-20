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
    pub password: String, // 生のパスワードを受け取る
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

        // パスワードをハッシュ化
        let password_hash = password_service.hash(&command.password).await?;

        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            // トランザクション境界内で一意性チェックを実行
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

        // 1. ユーザーを検索（トランザクション不要な読み取り）
        // Repositoryを直接使うか、Factory経由で取得するかは設計次第ですが、
        // ここでは単純化のためFactoryの実装を想定するか、あるいは個別のRepo Portを受け取るようにします。
        // ※ 本来は UseCase に UserRepository も持たせるのが一般的です。

        // 今回は TransactionManager を通じて Repo にアクセスする方針に合わせ、
        // 読み取り専用の execute (あるいは単発Repo利用) を想定します。
        tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let user = user_repo
                .find_by_email(&command.email)
                .await?
                .ok_or(AuthError::InvalidCredentials)?;

            // 2. パスワード検証
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
    // ユニットテストは今後の課題
}
