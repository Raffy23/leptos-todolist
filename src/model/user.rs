use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

pub type UserId = i64;

#[derive(Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct UserEntity {
    pub id: UserId,
    pub username: String,
    pub password: String,
}

impl UserEntity {
    #[cfg(feature = "ssr")]
    pub fn to_session_user(&self) -> SessionUser {
        use blake2::{Blake2s256, Digest};

        let mut hasher = Blake2s256::new();
        hasher.update(&self.password);

        SessionUser {
            id: self.id,
            hash: hasher.finalize().into(),
        }
    }

    pub fn to_user(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
        }
    }
}

impl std::fmt::Debug for UserEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionUser {
    pub id: UserId,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub username: String,
}
