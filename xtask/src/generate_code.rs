use std::fs;

use fusion_codegen::{
    prelude::{builder::prelude::Project, CodegenContext, CodegenFetchApi},
    CodeType, ProjectCodegen,
};

use crate::{dist_dir, project_root, DynError};

lazy_static! {
    static ref PROJECT_SERVER_URL: &'static str = "http://localhost:8300";
    static ref PROJECT_TOKEN: &'static str = "aV%8PKbB4R";
    static ref PROJECT_NAME: &'static str = "fusion-box-culture";
    //
    static ref PROJECT_SERVER: &'static str = "server";
    static ref PROJECT_WEB: &'static str = "frontend/frontend-wasm";
}

pub async fn gen() -> Result<(), DynError> {
    let codegen_context = CodegenContext::new(&PROJECT_SERVER_URL, &PROJECT_TOKEN);

    let project = match codegen_context.project(&PROJECT_NAME).await {
        Ok(project) => project,
        Err(e) => {
            eprintln!("Failed to fetch project: {:?}", e);
            std::process::exit(-1);
        }
    };

    log::info!("Project: {:#?}", project);

    gen_server(&project)?;
    gen_web(&project)?;

    Ok(())
}

fn gen_server(project: &Project) -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    let generated = match project.implement(CodeType::ServerCode) {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("Failed to generate server code: {:?}", e);
            std::process::exit(-1);
        }
    };

    let server_dir = project_root().join(format!("{}/src/project.rs", &PROJECT_SERVER as &str));

    std::fs::write(server_dir, generated.to_string())
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(())
}

fn gen_web(project: &Project) -> Result<(), DynError> {
    let generated = match project.implement(CodeType::WebCode) {
        Ok(quote) => quote,
        Err(e) => {
            eprintln!("Failed to generate web code: {:?}", e);
            std::process::exit(-1);
        }
    };

    let web_dir = project_root().join(format!("{}/src/project.rs", &PROJECT_WEB as &str));

    std::fs::write(web_dir, generated.to_string())
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(())
}
