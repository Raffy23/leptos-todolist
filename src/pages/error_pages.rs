use leptos::{logging::log, prelude::*};
use leptos_router::components::A;
use tracing::warn;

#[server]
pub async fn report_error() -> Result<(), ServerFnError> {
    warn!("Client encountered an error!");

    Ok(())
}


#[component]
pub fn NotFound() -> impl IntoView {
    let action = Action::new(move |_: &()| async { report_error().await });

    Effect::new_isomorphic(move || {
        log!("effect in NotFound component");
        action.dispatch(());
    });

    view! {
        <h1>"Not Found"</h1>
        <A href="/">"Home"</A>
    }
}
