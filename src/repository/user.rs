use sqlx::{Pool, Sqlite};
use tracing::instrument;

use super::RepositoryError;
use crate::model::{UserEntity, UserId};

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    #[instrument(skip(self, password_hash))]
    pub async fn create(&self, username: &str, password_hash: &str) -> Result<UserId, RepositoryError> {
        Ok(
            sqlx::query_scalar("INSERT INTO Users (username, password) VALUES (?, ?) RETURNING id")
                .bind(username)
                .bind(password_hash)
                .fetch_one(&self.db)
                .await?,
        )
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: UserId) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM Users WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: UserId) -> Result<Option<UserEntity>, RepositoryError> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.db)
            .await?)
    }

    #[instrument(skip(self))]
    pub async fn find_by_name(&self, username: &str) -> Result<Option<UserEntity>, RepositoryError> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.db)
            .await?)
    }
}
