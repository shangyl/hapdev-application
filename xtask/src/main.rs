#[macro_use]
extern crate lazy_static;

mod command;
mod generate_code;

use std::{
    env,
    path::{Path, PathBuf},
};

use log::Level;
use xshell::Shell;

use generate_code::gen;

type DynError = Box<dyn std::error::Error>;

pub struct ProjectDirectory {
    pub server: PathBuf,
    pub web: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let sh = &Shell::new()?;
    sh.change_dir(project_root());

    if let Err(e) = try_main().await {
        eprintln!("{}", e);
        std::process::exit(-1);
    }

    Ok(())
}

async fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("gen") => gen().await?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

gen            generate code
"
    )
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
