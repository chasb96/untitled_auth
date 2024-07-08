mod postgres;
mod cache;
mod mongo;

use cache::UsersCachingRepository;
use prost::Message;
use serde::Deserialize;
use sqlx::Row;
use sqlx::postgres::PgRow;
use crate::host::jwt::ClaimsUser;

use super::{error::QueryError, mongo::MongoDatabase, postgres::PostgresDatabase};

#[derive(Deserialize, Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub username: String,
    #[prost(string, tag = "3")]
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
    async fn set_password(&self, user_id: &str, password: &str) -> Result<(), QueryError>;

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError>;
}

#[allow(dead_code)]
pub enum UserRepositoryOption {
    Postgres(PostgresDatabase),
    CachedPostgres(UsersCachingRepository<PostgresDatabase>),
    Mongo(MongoDatabase),
    CachedMongo(UsersCachingRepository<MongoDatabase>),
}

impl UserRepository for UserRepositoryOption {
    async fn set_password(&self, user_id: &str, password_hash: &str) -> Result<(), QueryError> {
        match self {
            Self::Postgres(pg) => pg.set_password(user_id, password_hash).await,
            Self::CachedPostgres(cached_pg) => cached_pg.set_password(user_id, password_hash).await,
            Self::Mongo(mongo) => mongo.set_password(user_id, password_hash).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.set_password(user_id, password_hash).await,
        }
    }
    
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        match self {
            Self::Postgres(pg) => pg.get_by_username(username).await,
            Self::CachedPostgres(cached_pg) => cached_pg.get_by_username(username).await,
            Self::Mongo(mongo) => mongo.get_by_username(username).await,
            Self::CachedMongo(cached_mongo) => cached_mongo.get_by_username(username).await,
        }
    }
}

impl Default for UserRepositoryOption {
    fn default() -> Self {
        Self::Mongo(Default::default())
    }
}