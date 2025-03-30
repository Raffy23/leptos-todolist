use axum::async_trait;
use axum_login::AuthManagerLayer;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use sqlx::{Pool, Sqlite};
use tokio::task;
use tower_sessions::service::SignedCookie;
use tower_sessions_sqlx_store::SqliteStore;

use crate::{
    model::{Credentials, SessionUser},
    repository::{self, UserRepository},
};

impl AuthUser for SessionUser {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.hash
    }
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
    repository: UserRepository,
}

impl AuthBackend {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RepositoryError(#[from] repository::RepositoryError),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = Error;

    #[tracing::instrument(skip_all)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.repository.find_by_name(&creds.username).await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via `spawn_blocking`.
        task::spawn_blocking(|| {
            Ok(user
                .filter(|user| verify_password(creds.password, &user.password).is_ok())
                .map(|user| user.to_session_user()))
        })
        .await?
    }

    #[tracing::instrument(skip_all)]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self
            .repository
            .find_by_id(*user_id)
            .await?
            .map(|user| user.to_session_user()))
    }
}

pub type AuthSession = axum_login::AuthSession<AuthBackend>;

pub async fn new_session_layer(
    db: Pool<Sqlite>,
    user_repository: UserRepository,
) -> AuthManagerLayer<AuthBackend, SqliteStore, SignedCookie> {
    use axum_login::AuthManagerLayerBuilder;
    use tower_sessions::session_store::ExpiredDeletion;
    use tower_sessions::{
        cookie::{time::Duration, Key},
        Expiry, SessionManagerLayer,
    };
    use tower_sessions_sqlx_store::SqliteStore;

    let session_store = SqliteStore::new(db.clone());
    session_store.migrate().await.unwrap();

    // Delete old sessions in background task
    let _ = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    // Generate a cryptographic key to sign the session cookie.
    let key = Key::generate();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)))
        .with_signed(key);

    // Create the auth backend
    let backend = AuthBackend::new(user_repository.clone());

    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
