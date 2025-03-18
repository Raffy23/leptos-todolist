use sqlx::{Pool, Sqlite};

use super::Error;
use crate::model::{UserEntity, UserId};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    pub async fn create(&self, username: &str, password_hash: &str) -> Result<UserId, Error> {
        Ok(
            sqlx::query_scalar("INSERT INTO Users (username, password) VALUES (?, ?) RETURNING id")
                .bind(username)
                .bind(password_hash)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn delete(&self, id: UserId) -> Result<(), Error> {
        sqlx::query("DELETE FROM Users WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<Option<UserEntity>, Error> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.db)
            .await?)
    }

    pub async fn find_by_name(&self, username: &str) -> Result<Option<UserEntity>, Error> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.db)
            .await?)
    }
}
