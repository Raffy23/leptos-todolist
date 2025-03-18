use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;
use uuid::Uuid;

pub type NoteId = Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Note {
    pub uuid: NoteId,
    pub text: String,
}

