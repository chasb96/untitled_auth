use bytes::Bytes;
use prost::Message;
use redis::AsyncCommands;

use crate::host::repository::{error::QueryError, redis::RedisCache};

use super::{User, UserRepository};

pub struct UsersCachingRepository<T> {
    pub repository: T,
    pub cache: RedisCache,
}

impl<T> UserRepository for UsersCachingRepository<T>
where
    T: UserRepository,
{
    async fn set_password(&self, user_id: &str, password: &str) -> Result<(), QueryError> {
        self.repository
            .set_password(user_id, password)
            .await
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, QueryError> {
        let cache_key = format!("auth:{}", username);

        let mut conn = self.cache
            .connection_pool
            .get()
            .await?;

        if let Some(bytes) = conn.get(&cache_key).await? {
            let user = User::decode::<Bytes>(bytes)?;

            return Ok(Some(user))
        }

        if let Some(user) = self.repository.get_by_username(username).await? {
            let _: () = conn.set(cache_key, user.encode_to_vec()).await?;

            return Ok(Some(user))
        }

        Ok(None)
    }
}

impl<T> Default for UsersCachingRepository<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            repository: T::default(),
            cache: RedisCache::default(),
        }
    }
}