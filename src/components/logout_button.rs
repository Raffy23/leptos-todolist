use leptos::prelude::*;
use thaw::*;

use crate::provider::use_session_user;

#[server(Logout)]
#[tracing::instrument(name = "Logout", skip_all)]
pub(crate) async fn logout() -> Result<(), ServerFnError> {
    use crate::auth::AuthSession;
    use leptos_axum::extract;

    let mut session: AuthSession = extract().await?;
    session.logout().await?;

    Ok(())
}

#[component]
pub(crate) fn LogoutButton() -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();
    let user_context = use_session_user();

    Effect::new_isomorphic(move |_| {
        match logout_action.value().get() {
            Some(Ok(_)) => {
                user_context.set(Some(None));
            }
            Some(Err(_)) => {
                user_context.set(Some(None));
            }
            None => { /* action has not been invoked yet, not overwriting existing user */ }
        }
    });

    view! {
        <ActionForm action=logout_action>
            <Button button_type=ButtonType::Submit icon=icondata::FiLogOut>
                "Logout"
            </Button>
        </ActionForm>
    }
}
