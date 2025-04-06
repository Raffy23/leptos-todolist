use leptos::{ev, logging::log, prelude::*};
use leptos_meta::Style;
use leptos_router::components::A;
use thaw::*;
use tracing::{trace, warn};

use crate::{
    model::{Credentials, User},
    provider::use_session_user,
    routes,
};

#[server(Login)]
#[tracing::instrument(name = "Login", skip_all)]
pub(crate) async fn login(username: String, password: String) -> Result<User, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::UserRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();
    let users = expect_context::<UserRepository>();

    let mut session: AuthSession = extract().await?;
    if session.user.is_some() {
        response_options.set_status(StatusCode::BAD_REQUEST);
        return Err(ServerFnError::new("Already logged in"));
    }

    match session
        .authenticate(Credentials { username, password })
        .await
    {
        Ok(Some(user)) => {
            session.login(&user).await?;

            // Session user does contain only important information for the current
            // session. However, user wants to have full details of the user
            let user_entity = users.find_by_id(user.id).await?.unwrap();

            leptos_axum::redirect(routes::HOME.into());
            Ok(user_entity.to_user())
        }
        Ok(None) => {
            trace!("No user with name found!");

            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
        Err(error) => {
            warn!("Failed logging in: {:?}", error);

            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
    }
}

#[component]
pub(crate) fn LoginForm() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();
    let user_context = use_session_user();

    Effect::new_isomorphic(move |_| {
        match login_action.value().get() {
            Some(Ok(user)) => {
                user_context.set(Some(Some(user)));
            }
            Some(Err(_)) => {
                user_context.set(Some(None));
            }
            None => { /* action has not been invoked yet, not overwriting existing user */ }
        };
    });

    view! {
        <Style>
            "
            .login-from__button-bar {
                display:flex;
                justify-content:end;
                margin-top:24px;
                gap:8px;
            }
            "
        </Style>

        <ActionForm action=login_action>
            <FieldContextProvider>
                <Space vertical=true>
                    <Field label="Username" name="username">
                        <Input
                            input_type=InputType::Text
                            rules=vec![InputRule::required(true.into())]
                        />
                    </Field>
                    <Field label="Password" name="password">
                        <Input
                            input_type=InputType::Password
                            rules=vec![InputRule::required(true.into())]
                        />
                    </Field>

                    <div class="login-from__button-bar">
                        <A attr:class="thaw-button thaw-button--rounded thaw-button--medium thaw-button--secondary" href="/register">
                            "Register"
                        </A>
                        <Button
                            appearance=ButtonAppearance::Primary
                            button_type=ButtonType::Submit
                            on_click={
                                let field_context = FieldContextInjection::expect_context();
                                move |e: ev::MouseEvent| {
                                    if !field_context.validate() {
                                        e.prevent_default();
                                    }
                                }
                            }
                        >
                            "Login"
                        </Button>
                    </div>
                </Space>
            </FieldContextProvider>
        </ActionForm>
    }
}
