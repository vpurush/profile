#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
pub mod app;
pub mod error_template;
pub mod components;
pub mod common;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod contentful_richtext_renderer;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
