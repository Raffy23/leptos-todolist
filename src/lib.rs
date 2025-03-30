pub mod app;
#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "ssr")]
pub(crate) mod auth;
#[cfg(feature = "ssr")]
pub(crate) mod database;
#[cfg(feature = "ssr")]
pub(crate) mod repository;

pub(crate) mod components;
pub(crate) mod model;
pub(crate) mod pages;
pub(crate) mod provider;
pub(crate) mod routes;
pub(crate) mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;

    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}

//#[cfg(feature = "hydrate")]
//#[wasm_bindgen::prelude::wasm_bindgen(module = "/js/utils.js")]
//extern "C" {
//    fn decodeJWT(token: String) -> String;
//    fn emailRegex(email: &str) -> bool;
//}
