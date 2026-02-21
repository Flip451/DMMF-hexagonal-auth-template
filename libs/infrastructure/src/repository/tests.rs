use crate::repository::tx::SqlxTransactionManager;
use domain::models::user::{Email, PasswordHash, User, UserId, UserRepositoryError};
use domain::repository::tx::TransactionManager;

#[sqlx::test(migrations = "../../migrations")]
async fn test_save_and_find_user(pool: sqlx::PgPool) {
    let tm = SqlxTransactionManager::new(pool);

    let user_id = UserId::new();
    let email = Email::try_from("test@example.com").unwrap();
    let password_hash = PasswordHash::from_str_unchecked("hashed_pw");
    let user = User {
        id: user_id,
        email: email.clone(),
        password_hash,
    };

    // 1. 新規保存 (INSERT)
    let user_to_save = user.clone();
    let result: Result<(), domain::error::DomainError> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        repo.save(&user_to_save).await?;
        Ok::<(), domain::error::DomainError>(())
    })
    .await;
    assert!(result.is_ok());

    // 2. 検索して検証
    let email_to_find = email.clone();
    let found_user: Option<User> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        let res = repo.find_by_email(&email_to_find).await?;
        Ok::<Option<User>, domain::error::DomainError>(res)
    })
    .await
    .unwrap();

    assert!(found_user.is_some());
    let found = found_user.unwrap();
    assert_eq!(found.id, user_id);
    assert_eq!(found.email, email);

    // 3. 更新 (UPDATE / ON CONFLICT)
    let mut updated_user = found;
    updated_user.password_hash = PasswordHash::from_str_unchecked("new_hash");
    let user_to_update = updated_user.clone();
    let update_result: Result<(), domain::error::DomainError> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        repo.save(&user_to_update).await?;
        Ok::<(), domain::error::DomainError>(())
    })
    .await;
    assert!(update_result.is_ok());

    // 4. 更新結果の確認
    let found_after_update: Option<User> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        let res = repo.find_by_email(&email).await?;
        Ok::<Option<User>, domain::error::DomainError>(res)
    })
    .await
    .unwrap();
    assert_eq!(
        found_after_update.unwrap().password_hash.to_string(),
        "new_hash"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_duplicate_email_error(pool: sqlx::PgPool) {
    let tm = SqlxTransactionManager::new(pool);

    let email = Email::try_from("duplicate@example.com").unwrap();
    let user1 = User {
        id: UserId::new(),
        email: email.clone(),
        password_hash: PasswordHash::from_str_unchecked("hash1"),
    };
    let user2 = User {
        id: UserId::new(),
        email: email.clone(),
        password_hash: PasswordHash::from_str_unchecked("hash2"),
    };

    // 一人目を保存
    let res1: Result<(), domain::error::DomainError> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        repo.save(&user1).await?;
        Ok::<(), domain::error::DomainError>(())
    })
    .await;
    assert!(res1.is_ok());

    // 二人目を同じメールアドレスで保存（別のID）
    let res2: Result<(), domain::error::DomainError> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        repo.save(&user2).await?;
        Ok::<(), domain::error::DomainError>(())
    })
    .await;

    // UNIQUE INDEX によりエラーになるはず
    assert!(res2.is_err());
    if let Err(domain::error::DomainError::User(domain::models::user::UserError::Repository(
        UserRepositoryError::QueryFailed(_),
    ))) = res2
    {
        // 期待通りのエラー
    } else {
        panic!(
            "Expected QueryFailed error due to unique constraint, got {:?}",
            res2
        );
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn test_transaction_rollback(pool: sqlx::PgPool) {
    let tm = SqlxTransactionManager::new(pool);

    let email = Email::try_from("rollback@example.com").unwrap();
    let user = User {
        id: UserId::new(),
        email: email.clone(),
        password_hash: PasswordHash::from_str_unchecked("hash"),
    };

    // エラーを返してロールバックを誘発
    let user_to_save = user.clone();
    let result: Result<(), domain::error::DomainError> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        repo.save(&user_to_save).await?;
        Err(domain::error::DomainError::LogicViolation(
            "Intentional rollback",
        ))
    })
    .await;

    assert!(result.is_err());

    // 保存されていないことを確認
    let found_user: Option<User> = domain::tx!(tm, |factory| {
        let repo = factory.user_repository();
        let res = repo.find_by_email(&email).await?;
        Ok::<Option<User>, domain::error::DomainError>(res)
    })
    .await
    .unwrap();

    assert!(found_user.is_none());
}
