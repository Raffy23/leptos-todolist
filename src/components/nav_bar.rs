use leptos::prelude::*;
use thaw::*;

#[component]
pub(crate) fn NavBar() -> impl IntoView {
    view! {
        <Flex>
            <NavDrawer attr:style="top:1px">
                <NavItem icon=icondata::FiChrome value="chrome">
                    "Chrome"
                </NavItem>
            </NavDrawer>
        </Flex>
    }
}
