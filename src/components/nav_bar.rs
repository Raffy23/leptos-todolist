use leptos::prelude::*;
use thaw::*;

#[component]
pub(crate) fn NavBar() -> impl IntoView {
    view! {
        <Flex>
            <NavDrawer attr:style="top:1px">
                <NavItem icon=icondata::FiBook value="notes">
                    "Notes"
                </NavItem>
            </NavDrawer>
        </Flex>
    }
}
