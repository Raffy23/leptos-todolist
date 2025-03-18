use sqlx::{Pool, Sqlite};

use super::Error;
use crate::model::{Note, NoteId};

pub struct NotesRepository {
    db: Pool<Sqlite>,
}

impl NotesRepository {
    pub async fn create(&self, username: &str, password_hash: &str) -> Result<NoteId, Error> {
        Ok(
            sqlx::query_scalar("INSERT INTO Users (username, password) VALUES (?, ?) RETURNING id")
                .bind(username)
                .bind(password_hash)
                .fetch_one(&self.db)
                .await?,
        )
    }

    pub async fn delete(&self, id: NoteId) -> Result<(), Error> {
        sqlx::query("DELETE FROM Users WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: NoteId) -> Result<Note, Error> {
        Ok(sqlx::query_as("SELECT * FROM Users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await?)
    }
}
