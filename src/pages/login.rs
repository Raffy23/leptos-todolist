use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

use crate::{
    components::{LoginForm, RedirectAuthenticated},
    routes,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <RedirectAuthenticated path=routes::HOME />

        <Style>
            "
            .login-page {
                height:100vh;
                display:flex;
                align-items:center;
                justify-content:center;
                background-color:var(--colorNeutralBackgroundDisabled, #f0f0f0);
            }

            .login-page .thaw-card {
                max-width: 525px;
            }

            .login-page h1 {
                background-color: var(--colorNeutralBackground, #fff);
                border-bottom: 1px solid var(--colorNeutralStroke2);
            }
            "
        </Style>

        <div class="login-page">
            <Card>
                <CardHeader>
                    <Body1>
                        <h1 style="margin-top:0">Login</h1>
                    </Body1>
                </CardHeader>
                <LoginForm />
            </Card>
        </div>
    }
}
