#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

pub mod api;
pub mod app;
pub mod components;
#[cfg(feature = "ssr")]
pub mod core;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod routes;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
