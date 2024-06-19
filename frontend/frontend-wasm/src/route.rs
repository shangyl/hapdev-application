use yew::prelude::*;
use yew_router::prelude::*;

use admin_webui::prelude::*;
use webui::derive::*;

use web_config::ConfigModuleRoute;
use web_user::pages::UserProfilePage;
use web_user::structure::prelude::*;

use crate::pages::HomePage;

#[derive(Routable, Debug, Clone, PartialEq, RouteMenu)]
pub enum AppRoute {
    #[at("/user/:s")]
    #[menu(
        label = "用户管理",
        child = "StructUserAdminRoute",
        order = 300,
        svg = "/assets/icons/sprites/free.svg#cil-group"
    )]
    StructUserAdminRoute,
    #[at("/sys/config/:s")]
    #[menu(
        label = "系统配置",
        child = "ConfigModuleRoute",
        order = 999,
        svg = "/assets/icons/sprites/free.svg#cil-sitemap"
    )]
    ConfigModuleRoute,

    #[at("/profile")]
    #[menu(
        label = "账户信息",
        hidden,
        svg = "/assets/icons/sprites/free.svg#cil-featured-playlist"
    )]
    Profile,

    #[at("/message")]
    #[menu(
        label = "消息",
        hidden,
        svg = "/assets/icons/sprites/free.svg#cil-featured-playlist"
    )]
    Message,
    #[at("/sys/message")]
    #[menu(label = "消息", hidden, icon = "", svg = "", badge = "")]
    SystemMessage,

    #[at("/")]
    #[menu(
        label = "首页",
        order = 0,
        svg = "/assets/icons/sprites/free.svg#cil-speedometer",
        badge = ""
    )]
    Home,
}

impl NavMenuSwitch for AppRoute {
    fn switch(&self, app_state: &AppState) -> Html {
        match self {
            AppRoute::Home => {
                html!(<HomePage app_state={app_state.clone()} />)
            }
            AppRoute::StructUserAdminRoute => {
                let app_state = app_state.clone();
                let render =
                    move |route: StructUserAdminRoute| -> Html { route.switch(&app_state) };
                html! (
                    <Switch<StructUserAdminRoute> render={render} />
                )
            }
            // AppRoute::StructAdminRoute => {
            AppRoute::ConfigModuleRoute => {
                let app_state = app_state.clone();
                let render = move |route: ConfigModuleRoute| -> Html { route.switch(&app_state) };
                html! (
                    <Switch<ConfigModuleRoute> render={render} />
                )
            }

            AppRoute::Profile => {
                html!(<UserProfilePage app_state={app_state.clone()} />)
            }

            AppRoute::Message => {
                html!()
            }
            AppRoute::SystemMessage => {
                html!()
            }
        }
    }
}
