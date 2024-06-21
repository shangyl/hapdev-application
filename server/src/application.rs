use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    web::{self, ServiceConfig},
    App, Error,
};

use web_framework::prelude::{controller, AppState, WebApiScope};

use crate_config::prelude::Configs;
use crate_user::{application::ApplicationToken, auth::AuthService, AdminToken};

use crate::project::controller::FusionBoxCultureApi;

fn admin_service_config(config: &mut ServiceConfig) {
    controller::system::system_service(config);

    crate_config::service_config(config);
    crate_resource::service_config(config);
    crate_user::user::principal_config(config);

    crate_user::service_config(config);
    crate_user::user::service_config(config);
    crate_user::application::service_config(config);

    crate_weixin::admin_service_config(config);

    // config.model_service();
}

fn auth_service_config(config: &mut ServiceConfig) {
    crate_user::auth::struct_service_config(config);
}

pub fn build_app(
    _app_state: AppState,
    configs: web::Data<Configs>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let admin_api_prefix = "/api/v1/admin";

    let ws_prefix = "/ws/v1";

    let app_prefix = "/api/v1/app";

    let app = App::new()
        .app_data(configs)
        //admin prefix
        .service(
            web::scope(admin_api_prefix)
                .configure(auth_service_config)
                .configure(admin_service_config)
                .api_controller::<FusionBoxCultureApi>(),
        )
        .service(web::scope("").configure(crate_resource::public_resource_access_config))
        .wrap(
            AuthService::<AdminToken>::new(admin_api_prefix)
                .ignore_auth(admin_api_prefix)
                .ignore("/res/dir/{tail}*")
                .ignore("/res/static/{tail}*")
                .ignore(format!("{}/{}", ws_prefix, "{tail}*").as_str())
                .ignore(format!("{}/login/admin", admin_api_prefix).as_str())
                .ignore(format!("{}/login/dev", admin_api_prefix).as_str())
                .ignore(format!("{}/{}", app_prefix, "{tail}*").as_str()),
        );
    app
}
