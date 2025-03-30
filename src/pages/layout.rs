use leptos::prelude::*;
use leptos_router::components::Outlet;
use thaw::*;

use crate::components::{NavBar, RedirectUnauthenticated, SiteHeader};

#[component]
pub(crate) fn AppLayout() -> impl IntoView {
    view! {
        <RedirectUnauthenticated />

        <Layout position=LayoutPosition::Absolute>
            <SiteHeader />
            <Layout has_sider=true position=LayoutPosition::Absolute attr:style="top: 64px;">
                <LayoutSider>
                    <NavBar />
                </LayoutSider>
                <Layout>
                    <Layout attr:style="padding: 20px;">
                        <Outlet />
                    </Layout>
                </Layout>
            </Layout>
        </Layout>
    }
}
