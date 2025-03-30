use leptos::prelude::*;

use crate::model::User;

#[server]
#[tracing::instrument(name = "GetUser", skip_all)]
pub(crate) async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::UserRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let session: AuthSession = extract().await?;
    let response_options = expect_context::<leptos_axum::ResponseOptions>();

    if let Some(session_user) = session.user {
        let users = expect_context::<UserRepository>();
        let user = users.find_by_id(session_user.id).await?;

        Ok(user.map(|user| user.to_user()))
    } else {
        response_options.set_status(StatusCode::UNAUTHORIZED);
        Err(ServerFnError::new("Unauthorized"))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RefreshUser;

impl PartialEq for RefreshUser {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[component]
pub(crate) fn UserProvider(children: Children) -> impl IntoView {
    let (refresh_user, set_refresh_user) = signal(RefreshUser);

    let user_resource = Resource::new_blocking(
        move || refresh_user.get(),
        move |_| async move { get_user().await.unwrap_or(None) },
    );

    provide_context(user_resource);
    provide_context(set_refresh_user);

    view! {
        {children()}
    }
}
