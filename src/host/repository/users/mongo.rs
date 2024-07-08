use mongodb::bson::doc;

use crate::host::repository::{error::QueryError, mongo::MongoDatabase};

use super::{User, UserRepository};

impl UserRepository for MongoDatabase {
    async fn set_password(&self, user_id: &str, password: &str) -> Result<(), QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<User>("users")
            .update_one(
                doc! { "id": user_id }, 
                doc! { "$set": { "password_hash": password } }
            )
            .await?;

        Ok(())
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<super::User>, QueryError> {
        let conn = self.connection_pool
            .get()
            .await?;

        conn.collection::<User>("users")
            .find_one(doc! { "username": username })
            .await
            .map_err(Into::into)
    }
}