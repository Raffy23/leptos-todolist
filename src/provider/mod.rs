use leptos::{logging::log, prelude::*};
use session::Session;

pub mod session;

#[inline]
pub(crate) fn use_session_user() -> StoredValue<Session> {
    expect_context::<StoredValue<Session>>()
}

#[inline]
pub(crate) fn is_logged_in() -> bool {
    use_session_user()
        .read_value()
        .user
        .get()
        .is_some_and(|user| user.is_some())
}
