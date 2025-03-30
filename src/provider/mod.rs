use leptos::prelude::*;
use user::RefreshUser;

use crate::model::User;

mod user;

pub(crate) use user::UserProvider;

#[inline]
pub(crate) fn use_session_user() -> Resource<Option<User>> {
    expect_context::<Resource<Option<User>>>()
}

#[inline]
pub(crate) fn refresh_user() {
    expect_context::<WriteSignal<RefreshUser>>()
        .set(RefreshUser);
}
