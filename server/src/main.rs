mod application;
mod project;
mod result;

use clap::Parser;
use futures::FutureExt;
use std::sync::Arc;

use hapdev_app::prelude::{
    ApplicationScheduler, FusionApplication, FusionConfigArgs, FusionContext, FusionState,
};
use web_framework::prelude::{service::ModelService, AppState, WebAppState, WebServerFactory};

use crate_config::prelude::{ApplicationLoadConfig, RequestConfig};
use crate_user::prelude::model::application::Application;

use crate::application::build_app;
use result::AppServerResult;

fn main() -> AppServerResult<()> {
    let args: FusionConfigArgs = FusionConfigArgs::parse();
    let mut context = FusionContext::new_with_args(args);

    context.initialize();

    let mut application = FusionApplication::new(context);
    application.load_config()?;

    application.initialize_with_callback(|application| {
        let app_state = application.app_state();
        let applications =
            application.block_on(async { ModelService::<Application>::list(&app_state).await });

        log::info!("applications: {:?}", applications);

        Ok(())
    })?;

    let app_state = application.app_state();

    // 定时任务
    // application.enable_scheduler()?;

    // let ws_app_state = app_state.clone();
    // application.push_task("1/100 * * * * *".parse().unwrap(), false, move |context| {
    //     test(context.clone(), ws_app_state.clone()).boxed()
    // });

    let configs = application.configs().ok_or("configs not found")?;
    let app_factory = move |app_state| build_app(app_state, configs.clone());
    let factory = WebServerFactory::new("web-server", app_state, app_factory);

    application.build_server(factory);

    application.start();

    // application.wait(Some(Duration::from_secs(10)));
    application.wait(None);
    log::info!("application.join(None)");

    application.stop();
    log::info!("application.stop()");

    Ok(())
}
