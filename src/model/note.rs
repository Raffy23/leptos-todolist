use leptos::prelude::RwSignal;
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
    pub checked: bool,
}

impl NoteEntity {
    pub fn to_dto(self) -> NoteDto {
        NoteDto {
            id: self.id,
            title: self.title,
            content: self.content,
            checked: self.checked,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDto {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub content: String,
    pub checked: RwSignal<bool>,
}

impl Note {
    pub fn from_dto(dto: &NoteDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title.clone(),
            content: dto.content.clone(),
            checked: RwSignal::new(dto.checked),
        }
    }
}
