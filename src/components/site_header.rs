use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

use crate::components::LogoutButton;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let thaw_theme = Theme::use_rw_theme();

    let theme_icon = Memo::new(move |_| {
        match thaw_theme.get().name.as_str() {
            "light" => icondata::FiMoon,
            "dark" => icondata::FiSun,
            _ => panic!("Unknown theme"),
        }
    });

    let theme_name = Memo::new(move |_| {
        match thaw_theme.get().name.as_str() {
            "light" => "Dark",
            "dark" => "Light",
            _ => panic!("Unknown theme"),
        }
    });

    let change_theme = move |_| {
        match thaw_theme.get().name.as_str() {
            "light" => thaw_theme.set(Theme::dark()),
            "dark" => thaw_theme.set(Theme::light()),
            _ => panic!("Unknown theme"),
        }
    };


    view! {
        <Style>
        "
        .site-header {
            height: 64px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 20px 0 20px;
            border-bottom: 1px solid var(--colorNeutralStroke2);
        }
        "
        </Style>

        <LayoutHeader attr:class="site-header">
            <h1>"Leptos ToDo-List"</h1>

            <Input placeholder="Search" attr:style="width: 375px">
                <InputPrefix slot>
                    <Icon icon=icondata::FiSearch />
                </InputPrefix>
            </Input>

            <Space>
                <Button icon=theme_icon on_click=change_theme>
                    {theme_name}
                </Button>
                <LogoutButton />
            </Space>
        </LayoutHeader>
    }
}
