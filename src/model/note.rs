use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;
use uuid::Uuid;

use super::UserId;

pub type NoteId = Uuid;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct NoteEntity {
    pub id: NoteId,
    pub owner: UserId,
    pub title: String,
    pub content: String,
}

impl NoteEntity {
    pub fn to_note(self) -> Note {
        Note {
            id: self.id,
            title: self.title,
            content: self.content,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: String,
}

