use crate::host::repository::postgres::PostgresDatabase;
use super::error::GetUserError;
use super::error::SignUpError;
use super::User;
use super::UserRepository;

impl UserRepository for PostgresDatabase {
    async fn set_password(&self, user_id: i32, password_hash: &str) -> Result<(), SignUpError> {
        const UPDATE_QUERY: &'static str = r#"
            UPDATE users 
            SET password_hash = $2
            WHERE id = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(UPDATE_QUERY)
            .bind(user_id)
            .bind(password_hash)
            .execute(conn.as_mut())
            .await
            .map(|_| ())
            .map_err(SignUpError::from)
    }
    
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, GetUserError> {
        const GET_BY_USERNAME_QUERY: &'static str = r#"
            SELECT
                id,
                username,
                password_hash
            FROM
                users
            WHERE
                username = $1
        "#;

        let mut conn = self.connection_pool
            .get()
            .await?;

        sqlx::query(GET_BY_USERNAME_QUERY)
            .bind(username)
            .map(Into::into)
            .fetch_optional(conn.as_mut())
            .await
            .map_err(Into::into)
    }
}