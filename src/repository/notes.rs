use sqlx::{Pool, Sqlite};
use tracing::instrument;

use super::RepositoryError;
use crate::model::{NoteEntity, NoteId, UserId};

#[derive(Debug, Clone)]
pub struct NoteRepository {
    db: Pool<Sqlite>,
}

impl NoteRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }

    #[instrument(skip(self, title, content))]
    pub async fn create(
        &self,
        owner: UserId,
        title: &str,
        content: &str,
    ) -> Result<NoteId, RepositoryError> {
        let uuid = NoteId::new_v4();

        sqlx::query("INSERT INTO Notes (id, owner, title, content, checked) VALUES (?, ?, ?, ?, ?)")
            .bind(uuid)
            .bind(owner)
            .bind(title)
            .bind(content)
            .bind(false)
            .execute(&self.db)
            .await?;

        Ok(uuid)
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, owner: UserId, id: NoteId) -> Result<u64, RepositoryError> {
        Ok(sqlx::query("DELETE FROM Notes WHERE owner = ? AND id = ?")
            .bind(owner)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: NoteId) -> Result<NoteEntity, RepositoryError> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await?)
    }

    #[instrument(skip(self))]
    pub async fn find_by_owner(&self, owner: UserId) -> Result<Vec<NoteEntity>, RepositoryError> {
        Ok(sqlx::query_as("SELECT * FROM Notes WHERE owner = ?")
            .bind(owner)
            .fetch_all(&self.db)
            .await?)
    }

    #[instrument(skip(self))]
    pub async fn update_checked(&self, owner: UserId, id: NoteId, checked: bool) -> Result<u64, RepositoryError> {
        Ok(sqlx::query("UPDATE Notes SET checked = ? WHERE owner = ? AND id = ?")
            .bind(checked)
            .bind(owner)
            .bind(id)
            .execute(&self.db)
            .await?
            .rows_affected())
    }
}
