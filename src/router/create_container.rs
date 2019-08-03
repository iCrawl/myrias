use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

use crate::docker::docker_exec;

#[derive(Deserialize)]
pub struct CreateContainerInput {
    language: String,
}

pub fn index(create: web::Json<CreateContainerInput>) -> impl Responder {
    info!("Building container: myrias_{}", create.language);
    docker_exec(&[
        "run",
        "--rm",
        &format!("--name=myrias_{}", create.language),
        "-u1000:1000",
        "-w/tmp/",
        "-dt",
        "--net=none",
        &format!("myrias_{}:latest", create.language),
        "/bin/sh",
    ]);
    info!("Built container: myrias_{}", create.language);

    info!(
        "Creating eval directory in container: myrias_{}",
        create.language
    );
    docker_exec(&[
        "exec",
        &format!("myrias_{}", create.language),
        "mkdir",
        "eval",
    ]);
    info!(
        "Created eval directory in container: myrias_{}",
        create.language
    );

    info!(
        "Chmod eval directory to 711 in container: myrias_{}",
        create.language
    );
    docker_exec(&[
        "exec",
        &format!("myrias_{}", create.language),
        "chmod",
        "711",
        "eval",
    ]);
    info!(
        "Chmod' eval directory in container: myrias_{}",
        create.language
    );

    HttpResponse::Created()
}
