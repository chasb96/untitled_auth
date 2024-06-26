pub mod error;
mod postgres;

use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::host::jwt::ClaimsUser;

use self::error::{GetUserError, SignUpError};
use super::postgres::PostgresDatabase;

pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
        }
    }
}

impl From<User> for ClaimsUser {
    fn from(user: User) -> Self {
        ClaimsUser {
            id: user.id,
            username: user.username,
        }
    }
}

pub trait UserRepository {
    async fn set_password(&self, user_id: i32, password: &str) -> Result<(), SignUpError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError>;
}

pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
}

impl UserRepository for UserRepositoryOption {
    async fn set_password(&self, user_id: i32, password_hash: &str) -> Result<(), SignUpError> {
        match self {
            Self::Postgres(pg) => pg.set_password(user_id, password_hash).await
        }
    }
    
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Postgres(PostgresDatabase::default())
    }
}