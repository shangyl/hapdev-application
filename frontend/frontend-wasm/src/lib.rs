#![recursion_limit = "512"]

use url::Url;
use wasm_bindgen::prelude::*;

use crate::index::Index;
use crate::route::AppRoute;

use admin_webui::prelude::*;
use webui::support::prelude::*;

use web_user::structure::prelude::*;

use admin_webui::wasm::*;

mod index;
mod pages;
mod project;
mod route;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    let app_config = AppConfig {
        title: "fusion-creator".to_string(),
    };

    let web_api = if cfg!(debug_assertions) {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
        let url: Url = "http://localhost:8300/api/v1/admin".parse().unwrap();
        WebApi::new(url)
    } else {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
        let url: Url = "http://fire.isolovr.com/api/v1/admin".parse().unwrap();
        WebApi::new(url)
    };

    let app_props = WebAppProps {
        app_state: AppState::new(app_config, web_api),
        login_api: "login/admin".to_string(),
        ui_event_callback: None,
    };
    yew::Renderer::<WebApp<AppRoute, AuthLoginPage, Index, CurrentUser>>::with_props(app_props)
        .render();
    Ok(())
}

#[wasm_bindgen]
pub fn define_custom_elements() {
    ComponentWrapper::<Footer>::define("footer-wrapper");
}
