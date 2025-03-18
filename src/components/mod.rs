use leptos::prelude::*;
use leptos_router::{
    components::{Outlet, Redirect},
};

use crate::provider::use_session_user;

#[component]
pub fn RedirectToLogin() -> impl IntoView {
    let user_context = use_session_user();

    view! {
        <Suspense>
            <Show when=move || { user_context.read_value().user.get().is_some_and(|inner| inner.is_none()) }>
                <Redirect path="/login"/>
            </Show>
        </Suspense>
        <Outlet/>
    }
}
