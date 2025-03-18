mod notes;
mod user;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

pub(crate) use notes::NotesRepository;
pub(crate) use user::UserRepository;
