use leptos::{logging::log, prelude::*};
use leptos_router::components::A;
use tracing::warn;

use crate::routes;

#[server]
pub async fn report_error() -> Result<(), ServerFnError> {
    warn!("Client encountered an error!");

    Ok(())
}

#[component]
pub fn NotFound() -> impl IntoView {
    let action = Action::new(move |_: &()| async { report_error().await });

    Effect::new(move || {
        log!("Effect in NotFound component");
        action.dispatch(());
    });

    view! {
        <h1>"Not Found"</h1>
        <A href=routes::HOME.0>"Home"</A>
    }
}
