use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, ProtectedRoute, Redirect, Route, Router, Routes}, path, ParamSegment, PathSegment, StaticSegment, WildcardSegment
};
use thaw::ssr::SSRMountStyleProvider;
use thaw::*;

use crate::{components::RedirectToLogin, pages::{HomePage, LoginPage, NotFound}, provider::{is_logged_in, session::SessionProvider}};

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
            <Title text="TodoList" />

            <SessionProvider>
                <Router>
                    <main>
                        <Routes fallback=NotFound >
                            <Route path=StaticSegment("") view=HomePage />
                            <Route path=StaticSegment("/login") view=LoginPage />
                            <ParentRoute path=StaticSegment("/notes") view=RedirectToLogin>
                                <Route path=StaticSegment("") view=move || view! { <h2>"Notes"</h2> }/>
                                <Route path=ParamSegment("id") view=move || view! { <h2>"Note: - abc"</h2> }/>
                            </ParentRoute>
                        </Routes>
                    </main>
                </Router>
            </SessionProvider>
        </ConfigProvider>
    }
}
