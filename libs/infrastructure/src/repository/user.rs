use chrono::{DateTime, Utc};
use domain::models::user::{Email, PasswordHash, User, UserId, UserRepositoryError};
use sqlx::{Postgres, query, query_as};
use uuid::Uuid;

/// SQLx を使用したユーザーリポジトリの低レベル操作。
pub struct SqlxUserRepository;

impl SqlxUserRepository {
    pub async fn find_by_email<'e, E>(executor: E, email: &Email) -> Result<Option<User>, UserRepositoryError>
    where
        E: sqlx::Executor<'e, Database = Postgres>,
    {
        let row = query_as::<Postgres, UserRow>("SELECT * FROM users WHERE email = $1")
            .bind(email.as_ref())
            .fetch_optional(executor)
            .await
            .map_err(|e| UserRepositoryError::QueryFailed(e.into()))?;

        Ok(row.map(User::from))
    }

    pub async fn save<'e, E>(executor: E, user: &User) -> Result<(), UserRepositoryError>
    where
        E: sqlx::Executor<'e, Database = Postgres>,
    {
        let now = Utc::now();
        let system_name = "auth-system";
        let pgm_cd = "auth-user-mgmt";
        let tx_id = "tx-none";

        query(
            r#"
            INSERT INTO users (
                id, email, password_hash,
                created_at, created_by, created_pgm_cd, created_tx_id,
                updated_at, updated_by, updated_pgm_cd, updated_tx_id,
                lock_no
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                updated_at = $8,
                updated_by = $9,
                updated_pgm_cd = $10,
                updated_tx_id = $11,
                lock_no = users.lock_no + 1
            WHERE users.lock_no = $12
            "#,
        )
        .bind(Uuid::from(user.id))
        .bind(user.email.as_ref())
        .bind(user.password_hash.as_ref())
        .bind(now)
        .bind(system_name)
        .bind(pgm_cd)
        .bind(tx_id)
        .bind(now)
        .bind(system_name)
        .bind(pgm_cd)
        .bind(tx_id)
        .bind(1)
        .execute(executor)
        .await
        .map_err(|e| UserRepositoryError::QueryFailed(e.into()))?;

        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    created_by: String,
    created_pgm_cd: String,
    created_tx_id: String,
    updated_at: DateTime<Utc>,
    updated_by: String,
    updated_pgm_cd: String,
    updated_tx_id: String,
    lock_no: i32,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: UserId::from(row.id),
            email: Email::try_from(row.email).expect("Invalid email in database"),
            password_hash: PasswordHash::from_str_unchecked(row.password_hash),
        }
    }
}
