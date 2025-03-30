use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    ParamSegment, StaticSegment,
};
use thaw::ssr::SSRMountStyleProvider;
use thaw::*;

use crate::{
    pages::{AppLayout, HomePage, LoginPage, NotFound, RegisterPage},
    provider::UserProvider,
    routes,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <ConfigProvider>
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/leptos-todolist.css" />

            // sets the document title
            <Title text="Leptos ToDo-List" />

            <UserProvider>
                <Router>
                    <main>
                        <Routes fallback=NotFound >
                            <ParentRoute path=StaticSegment(routes::HOME) view=AppLayout>
                                <Route path=StaticSegment("") view=HomePage />
                                <Route path=ParamSegment("id") view=move || view! { <h2>"Note: - abc"</h2> }/>
                            </ParentRoute>
                            <Route path=StaticSegment(routes::LOGIN) view=LoginPage />
                            <Route path=StaticSegment(routes::REGISTER) view=RegisterPage />
                        </Routes>
                    </main>
                </Router>
            </UserProvider>
        </ConfigProvider>
    }
}
