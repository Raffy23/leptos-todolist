mod notes;
mod user;

#[derive(Debug, thiserror::Error)]
pub(crate) enum RepositoryError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

pub(crate) use notes::NoteRepository;
pub(crate) use user::UserRepository;
