pub mod dto;
pub mod query;

use async_trait::async_trait;
use std::sync::Arc;

use self::dto::LoginResponseDto;
pub use self::query::LoginQuery;
use crate::clock::Clock;
use crate::models::auth::{AuthError, PasswordService, RawPassword};
use crate::models::user::{Authenticatable, Email, User, UserIdentity};
use crate::repository::tx::TransactionManager;
use crate::usecase::auth::AuthService;
use crate::usecase::error::UseCaseResult;

#[async_trait]
pub trait AuthQueryUseCase: Send + Sync {
    async fn login(&self, query: LoginQuery) -> UseCaseResult<LoginResponseDto>;
}

pub struct AuthQueryUseCaseImpl<TM, PS, C>
where
    TM: TransactionManager,
    PS: PasswordService,
    C: Clock,
{
    transaction_manager: Arc<TM>,
    password_service: Arc<PS>,
    auth_service: Arc<dyn AuthService>,
    _clock: Arc<C>,
}

impl<TM, PS, C> AuthQueryUseCaseImpl<TM, PS, C>
where
    TM: TransactionManager,
    PS: PasswordService,
    C: Clock,
{
    pub fn new(
        transaction_manager: Arc<TM>,
        password_service: Arc<PS>,
        auth_service: Arc<dyn AuthService>,
        clock: Arc<C>,
    ) -> Self {
        Self {
            transaction_manager,
            password_service,
            auth_service,
            _clock: clock,
        }
    }
}

#[async_trait]
impl<TM, PS, C> AuthQueryUseCase for AuthQueryUseCaseImpl<TM, PS, C>
where
    TM: TransactionManager,
    PS: PasswordService + 'static,
    C: Clock + 'static,
{
    async fn login(&self, query: LoginQuery) -> UseCaseResult<LoginResponseDto> {
        let email = Email::try_from(query.email.into_inner())?;
        let password_service = Arc::clone(&self.password_service);

        let user = crate::tx!(self.transaction_manager, |factory| {
            let user_repo = factory.user_repository();

            let user = user_repo
                .find_by_email(&email)
                .await?
                .ok_or(AuthError::InvalidCredentials)?;

            let is_valid = password_service
                .verify(
                    &RawPassword::from(query.password.into_inner()),
                    user.password_hash(),
                )
                .await?;

            if !is_valid {
                return Err(AuthError::InvalidCredentials.into());
            }

            Ok::<User, crate::error::DomainError>(user)
        })
        .await?;

        // ユースケース内でトークンを発行
        let token = self.auth_service.issue_token(user.id())?;

        Ok(LoginResponseDto::new(&user, token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::IdGenerator;
    use crate::models::user::UserId;
    use crate::test_utils::FixedClock;
    use crate::usecase::auth::AuthToken;
    use crate::usecase::auth::test_utils::utils::*;
    use crate::usecase::error::UseCaseError;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_login_success(
        valid_email: Email,
        valid_password: String,
        valid_password_hash: crate::models::user::PasswordHash,
    ) {
        let id_generator = crate::test_utils::MockIdGenerator::<UserId>::with_generated_ids(1);
        let user = User::new(
            id_generator.generate(),
            valid_email.clone(),
            valid_password_hash.clone(),
        );
        let repo = Arc::new(StubUserRepository {
            found_user: Some(user),
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(true)),
            hash_result: Arc::new(move || Ok(valid_password_hash.clone())),
        });
        let auth_service = Arc::new(StubAuthService {
            issue_token_result: Arc::new(|| Ok(AuthToken::from("test-token".to_string()))),
            verify_token_result: Arc::new(|| unreachable!()),
        });
        let clock = Arc::new(FixedClock::new(chrono::Utc::now()));

        let usecase = AuthQueryUseCaseImpl::new(tm, ps, auth_service, clock);
        let result = usecase
            .login(LoginQuery {
                email: valid_email.to_string().into(),
                password: valid_password.into(),
            })
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.email, valid_email.to_string());
        assert_eq!(response.token.expose_as_str(), "test-token");
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_invalid_credentials(valid_email: Email, valid_password: String) {
        let id_generator = crate::test_utils::MockIdGenerator::<UserId>::with_generated_ids(1);
        let user = User::new(
            id_generator.generate(),
            valid_email.clone(),
            crate::models::user::PasswordHash::from_str_unchecked("hashed"),
        );
        let repo = Arc::new(StubUserRepository {
            found_user: Some(user),
            save_error: None,
        });
        let factory = Arc::new(StubRepositoryFactory { repo });
        let tm = Arc::new(StubTransactionManager { factory });
        let ps = Arc::new(StubPasswordService {
            verify_result: Arc::new(|| Ok(false)), // Password mismatch
            hash_result: Arc::new(|| {
                Ok(crate::models::user::PasswordHash::from_str_unchecked(
                    "hashed",
                ))
            }),
        });
        let auth_service = Arc::new(StubAuthService {
            issue_token_result: Arc::new(|| unreachable!()),
            verify_token_result: Arc::new(|| unreachable!()),
        });
        let clock = Arc::new(FixedClock::new(chrono::Utc::now()));

        let usecase = AuthQueryUseCaseImpl::new(tm, ps, auth_service, clock);
        let result = usecase
            .login(LoginQuery {
                email: valid_email.to_string().into(),
                password: valid_password.into(),
            })
            .await;

        assert!(matches!(result, Err(UseCaseError::Authentication(_))));
    }
}
