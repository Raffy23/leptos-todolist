use leptos::{ev, prelude::*};
use thaw::*;

use crate::routes;

#[server(Register)]
#[tracing::instrument(skip_all)]
pub(crate) async fn create_user(
    username: String,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::repository::UserRepository;
    use password_auth::generate_hash;

    let users = use_context::<UserRepository>().unwrap();
    users.create(&username, &generate_hash(password)).await.unwrap();

    // TODO: directly do a login and redirect to home

    leptos_axum::redirect(routes::LOGIN.into());
    Ok(())
}

#[component]
pub(crate) fn RegisterPage() -> impl IntoView {
    let register_action = ServerAction::<Register>::new();

    view! {
        <ActionForm action=register_action>
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
                        "Register"
                    </Button>
                </Space>
            </FieldContextProvider>
        </ActionForm>
    }
}
