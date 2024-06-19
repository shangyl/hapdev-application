mod project;
mod result;

use std::env;

use hapdev_app::prelude::FusionContext;

use result::AppResult;

fn main() -> AppResult<()> {
    let _cur_dir = env::current_dir().unwrap();

    let mut context = FusionContext::default();

    Ok(())
}
