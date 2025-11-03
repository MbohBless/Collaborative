use sqlx::PgPool;
use uuid::Uuid;
use models::{User, CreateUser, Role};
use crate::errors::DbError;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: CreateUser, password_hash: String) -> Result<User, DbError> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, role)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, email, password_hash, role as "role: Role", created_at, updated_at
            "#,
            Uuid::new_v4(),
            user.username,
            user.email,
            password_hash,
            user.role as Role,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role as "role: Role", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DbError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role as "role: Role", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}