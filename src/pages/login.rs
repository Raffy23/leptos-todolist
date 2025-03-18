use leptos::{ev, prelude::*};
use thaw::*;

use crate::provider::use_session_user;

#[component]
pub fn LoginPage() -> impl IntoView {
    let user_context = use_session_user();

    view! {
        <div class="login-background">
            <Card>
                <CardHeader>
                    <Body1>
                        <h1 style="margin-top:0">Login</h1>
                    </Body1>
                </CardHeader>
                <ActionForm action=user_context.read_value().login>
                    <FieldContextProvider>
                        <Space vertical=true>
                            <Field label="Username" name="username">
                                <Input rules=vec![InputRule::required(true.into())] />
                            </Field>
                            <Field label="Password" name="password">
                                <Input input_type=InputType::Password rules=vec![InputRule::required(true.into())]/>
                            </Field>

                            <div class="login-from-button-bar">
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
            </Card>
        </div>
    }
}
