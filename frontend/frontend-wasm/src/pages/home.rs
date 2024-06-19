use yew::prelude::*;

use admin_webui::prelude::dashboard::SystemOverview;
use admin_webui::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HomePageProps {
    pub app_state: AppState,
}

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = HomePageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let query_api = "system/profile";

        let app_state = _ctx.props().app_state.clone();
        html!( <Content>
                <div class="container-fluid">
                    <div class="row">
                        <div class="col-6">
                            <SystemOverview app_state={app_state.clone()} query_api={query_api} montpoint_filter={vec!["/".to_string()]}/>
                        </div>
                        <div class="col-6">

                        </div>
                    </div>

                </div>
            </Content>
        )
    }
}
