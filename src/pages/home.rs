use leptos::{logging::log, prelude::*, reactive::graph::ReactiveNode, task::spawn_local};
use leptos_router::{
    components::{ParentRoute, ProtectedRoute, Route, A},
    path,
};
use thaw::*;

use crate::{model::Credentials, provider::use_session_user};
use crate::provider::session::{Login, Logout};

#[server]
pub(crate) async fn create_user() -> Result<String, ServerFnError> {
    use crate::repository::UserRepository;
    use password_auth::generate_hash;

    let users = use_context::<UserRepository>().unwrap();
    users.create("test", &generate_hash("test")).await.unwrap();

    Ok(format!("user test created"))
}

/// Renders the home page of your application.
#[component]
pub(crate) fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let user_context = use_session_user();

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <Suspense fallback=|| view! { <p>"Loading \"/login\"..."</p> }>
            <Show when=move || { user_context.read_value().user.get().is_some_and(|user| user.is_some())}>
                <Button appearance=ButtonAppearance::Primary on:click=on_click>
                    "Click Me: "
                    {count}
                </Button>
            </Show>
        </Suspense>

        <A href="/protected">"Go to protected"</A>
        <A href="/public">"Go to public"</A>

        <div style="display:flex;gap:8px">
            <Button
                appearance=ButtonAppearance::Primary
                on:click=move |_| { spawn_local(async { let _ = create_user().await; }); }
            >
                "create_user"
            </Button>
            <Button
                appearance=ButtonAppearance::Primary
                on:click=move |_| { user_context.read_value().login.dispatch(Login { username: "".to_owned(), password: "".to_owned() } ); }
            >
                "login"
            </Button>
            <Button
                appearance=ButtonAppearance::Primary
                on:click=move |_| { user_context.read_value().logout.dispatch(Logout {}); }
            >
                "logout"
            </Button>
        </div>
    }
}
