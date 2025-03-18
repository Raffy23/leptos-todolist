use leptos::{logging::log, prelude::*};

use crate::model::{Credentials, User};

pub(crate) struct Session {
    pub user: Resource<Option<User>>,

    pub login: ServerAction<Login>,//Action<(), Result<Option<User>, ServerFnError>>,
    pub logout: ServerAction<Logout>,//Action<(), Result<(), ServerFnError>>,
}

#[server]
pub(crate) async fn me() -> Result<Option<User>, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::UserRepository;
    use leptos_axum::extract;

    let session: AuthSession = extract().await?;
    if let Some(session_user) = session.user {
        let users = expect_context::<UserRepository>();
        let user = users.find_by_id(session_user.id).await?;

        Ok(user.map(|user| user.to_user()))
    } else {
        Err(ServerFnError::new("Unauthorized"))
    }
}

#[server(Login)]
pub(crate) async fn login(username: String, password: String) -> Result<Option<User>, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::UserRepository;
    use leptos_axum::extract;

    log!("username: {}, password: {}", username, password);

    let users = expect_context::<UserRepository>();
    let user = users.find_by_name("test").await.unwrap().unwrap();
    let session_user = user.to_session_user();

    let mut session: AuthSession = extract().await?;
    session.login(&session_user).await?;

    leptos_axum::redirect("/notes");
    Ok(Some(user.to_user()))
}

#[server(Logout)]
pub(crate) async fn logout() -> Result<(), ServerFnError> {
    use crate::auth::AuthSession;
    use leptos_axum::extract;

    let mut session: AuthSession = extract().await?;
    session.logout().await?;

    Ok(())
}

#[component]
pub(crate) fn SessionProvider(children: Children) -> impl IntoView {
    let login_action = ServerAction::<Login>::new();//Action::new(move |_: &()| async { login().await });
    let login_changed = login_action.value();

    let logout_action = ServerAction::<Logout>::new();//Action::new(move |_: &()| async { logout().await });
    let logout_changed = logout_action.value();

    Effect::watch(
        move || logout_changed.get(),
        move |_, _, _| {
            log!("logout called");
            login_changed.set(Some(Ok(None)));
        },
        false,
    );

    let user_resource = Resource::new_blocking(
        move || login_changed.get(),
        move |action_result| async move {
            match action_result {
                Some(Ok(session_user)) => session_user,
                Some(Err(_)) => None,
                None => me().await.unwrap_or(None)
            }
        },
    );

    let session_data = StoredValue::new(Session {
        login: login_action,
        logout: logout_action,
        user: user_resource
    });

    provide_context(session_data);

    view! {
        {children()}
    }
}
