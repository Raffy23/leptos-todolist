use crate::{provider::use_session_user, routes};
use leptos::prelude::*;
use leptos_router::components::Redirect;

#[component]
pub fn RedirectAuthenticated(path: routes::Route) -> impl IntoView {
    let user_context = use_session_user();

    view! {
        <Suspense>
            <Show when=move || { user_context.get().is_some_and(|inner| inner.is_some()) }>
                <Redirect path={path.0}/>
            </Show>
        </Suspense>
    }
}

#[component]
pub fn RedirectUnauthenticated() -> impl IntoView {
    let user_context = use_session_user();

    view! {
        <Suspense>
            <Show when=move || { user_context.get().is_some_and(|inner| inner.is_none()) }>
                <Redirect path={routes::LOGIN.0} />
            </Show>
        </Suspense>
    }
}
