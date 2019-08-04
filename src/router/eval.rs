use actix_web::{web, HttpResponse, Responder};
use log::info;
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::sync::{mpsc, mpsc::RecvTimeoutError};
use std::thread;
use std::time::Duration;

use crate::docker::docker_exec;

#[derive(Deserialize)]
pub struct EvalInput {
    language: String,
    code: String,
}

#[derive(Serialize, Deserialize)]
struct DockerOutput {
    result: String,
}

pub fn index(eval: web::Json<EvalInput>) -> impl Responder {
    let snowflake = Snowflake::default().generate();

    info!(
        "Creating unique eval folder in container: myrias_{}",
        eval.language
    );
    docker_exec(&[
        "exec",
        &format!("myrias_{}", eval.language),
        "mkdir",
        &format!("eval/{}", &snowflake),
    ]);
    info!(
        "Created unique eval folder in container: myrias_{}",
        eval.language
    );

    info!(
        "Chmod unique eval directory to 711 in container: myrias_{}",
        eval.language
    );
    docker_exec(&[
        "exec",
        &format!("myrias_{}", eval.language),
        "chmod",
        "777",
        &format!("eval/{}", &snowflake),
    ]);
    info!(
        "Chmod' unique eval directory in container: myrias_{}",
        eval.language
    );

    info!("Eval in container: myrias_{}", eval.language);
    let e = eval.language.clone();
    let (sender, recv) = mpsc::channel();
    thread::spawn(move || {
        if let Ok(()) = sender.send(
            Command::new("docker")
                .args(&[
                    "exec",
                    "-u1001:1001",
                    &format!("-w/tmp/eval/{}", &snowflake),
                    &format!("myrias_{}", eval.language),
                    "/bin/sh",
                    "/var/run/run.sh",
                    &eval.code,
                ])
                .output()
                .unwrap(),
        ) {}
    });

    let output = match recv.recv_timeout(Duration::from_secs(15)) {
        Ok(res) => res,
        Err(RecvTimeoutError::Timeout) => {
            info!("Eval timed out in container: myrias_{}", e);
            info!("Killing container: myrias_{}", e);
            Command::new("docker")
                .args(&["kill", &format!("myrias_{}", e)])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap();
            info!("Killed container: myrias_{}", e);

            return HttpResponse::GatewayTimeout().into();
        }
        Err(RecvTimeoutError::Disconnected) => {
            return HttpResponse::InternalServerError().into();
        }
    };

    info!("Finished eval in container: myrias_{}", e);

    let res = if output.stderr.is_empty() {
        String::from_utf8_lossy(&output.stdout)
    } else {
        String::from_utf8_lossy(&output.stderr)
    };

    info!("Removing unique eval folder in container: myrias_{}", e);
    docker_exec(&[
        "exec",
        &format!("myrias_{}", e),
        "rm",
        "-rf",
        &format!("eval/{}", &snowflake),
    ]);
    info!("Removed unique eval folder in container: myrias_{}", e);

    HttpResponse::Ok().json(DockerOutput {
        result: res.to_string(),
    })
}
