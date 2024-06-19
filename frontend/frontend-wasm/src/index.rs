use std::rc::Rc;

use yew::prelude::*;
use yew_router::{prelude::*, AnyRoute};

use admin_webui::prelude::*;
use webui::prelude::{RouteMenu, *};

use web_user::{structure::prelude::*, user::prelude::Permission};

use crate::route::AppRoute;

#[derive(Debug)]
pub struct Index {
    pub(crate) app_state: AppState,
    pub(crate) show_sidebar: bool,
}

impl Component for Index {
    type Message = IndexMsg<CurrentUser>;
    type Properties = IndexPageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            app_state: ctx.props().app_state.clone(),
            show_sidebar: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IndexMsg::Logout => ctx.props().on_logout.emit(()),
            IndexMsg::FetchCurrentUser(mut user) => {
                match user.account.account_type {
                    AccountType::Admin => {
                        user.permissions.push(Permission {
                            code: "perm_customer".to_string(),
                            ..Permission::default()
                        });

                        user.permissions.push(Permission {
                            code: "perm_team".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_project".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_oa".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_perform".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_user".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_account_manager".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_role_manager".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_permission_manager".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_struct".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_struct_manager".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_employee_manager".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_system".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_config".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_admin_system_config".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_log".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_resource_catalog".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_weixin_account".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_weixin_menu".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_weixin_message".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_app".to_string(),
                            ..Permission::default()
                        });
                        user.permissions.push(Permission {
                            code: "perm_app_version".to_string(),
                            ..Permission::default()
                        });
                    }
                    AccountType::User => {
                        user.permissions.push(Permission {
                            code: "perm_mine".to_string(),
                            ..Permission::default()
                        });
                    }
                    _ => {}
                }
                self.app_state.set_principal(Rc::new(user));
            }
            IndexMsg::None => {}
            IndexMsg::ShowSideBar => {
                self.show_sidebar = !self.show_sidebar;
            }
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_pages(ctx, "....".to_string())
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if _first_render {
            if !ctx.props().app_state.has_user() {
                let app_state = ctx.props().app_state.clone();
                let web_api = app_state.web_api().borrow();
                self.fetch_principal(ctx, web_api)
            }
        }
    }
}

impl Index {
    fn view_pages(&self, ctx: &Context<Self>, _token: String) -> Html {
        let app_state = self.app_state.clone();
        let render = move |route: AppRoute| -> Html { route.switch(&app_state.clone()) };

        let app_state = self.app_state.clone();

        let nav_menu = NavRouteMenuState::new(AppRoute::menus());

        let message = RouteMenuItem::new("message", AnyRoute::new(AppRoute::Message.to_path()));
        let sys_message = RouteMenuItem::new_with_icon(
            "",
            HtmlIcon::Icon("/assets/icons/sprites/free.svg#cil-pencil".to_string()),
            AnyRoute::new(AppRoute::SystemMessage.to_path()),
        );
        html!(
            <Container>
                <ToastViewer />
                <BrowserRouter>
                    <SideBar>
                        <SideBarNav app_state={app_state.clone()} route_menus={nav_menu}/>
                    </SideBar>
                    <div class="wrapper d-flex flex-column min-vh-100 bg-light">
                        <Header app_state={app_state.clone()} on_logout={ctx.link().callback(|_|IndexMsg::Logout)}>
                            <Slot<HeaderSlot> slot={HeaderSlot::LeftNavItem}>
                                <NavRouteItem icon_class={"icon icon-lg"} nav_route={message}/>
                            </Slot<HeaderSlot>>
                            <Slot<HeaderSlot> slot={HeaderSlot::RightNavItem}>
                                <NavRouteItem icon_class={"icon icon-lg"} nav_route={sys_message}/>
                            </Slot<HeaderSlot>>
                            <Slot<HeaderSlot> slot={HeaderSlot::PrincipalMenu}>
                                {self.view_account_menu(ctx)}
                            </Slot<HeaderSlot>>
                        </Header>
                        <PageContent>
                            <Switch<AppRoute>
                                render={render}/>
                        </PageContent>
                        <Footer/>
                    </div>
                </BrowserRouter>
            </Container>
        )
    }

    pub fn view_account_menu(&self, ctx: &Context<Self>) -> Html {
        // let menu_item = MenuItem::new("退出", ctx.link().callback(|_| IndexMsg::Logout));
        let on_logout = ctx.link().callback(|_| IndexMsg::Logout);

        let profile = AnyRoute::new(AppRoute::Profile.to_path());
        let route = AnyRoute::new(AppRoute::Message.to_path());
        html!(
            <Dropdown>
                <DropdownItem item={DropdownItemType::Header("账户")}/>
                <DropdownItem item={DropdownItemType::Item("账户信息")} on_route ={profile} />
                <DropdownItem item={DropdownItemType::Item("消息")} on_route = {route}/>
                <DropdownItem item={DropdownItemType::Divider} />
                <DropdownItem item={DropdownItemType::Item("退出")} on_click={on_logout} />
            </Dropdown>
        )
    }
}

impl IndexPage<AppRoute, CurrentUser> for Index {}
