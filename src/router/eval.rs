use log::info;
use rocket::{http::Status, post, State};
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};
use rustflake::Snowflake;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::{mpsc, mpsc::RecvTimeoutError};
use std::thread;
use std::time::Duration;

use crate::docker::Docker;
use crate::Config;

#[derive(Serialize, Deserialize)]
pub struct EvalInput {
    language: String,
    code: String,
}

#[post("/eval", format = "json", data = "<eval>")]
pub fn index(eval: Json<EvalInput>, config: State<Config>) -> Result<JsonValue, Status> {
    if !config.languages.contains(&eval.language) {
        return Err(Status::NotFound);
    }

    let snowflake = Snowflake::default().generate();

    info!(
        "Creating unique eval folder in container: myrias_{}",
        eval.language
    );
    Docker::exec(&[
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
    Docker::exec(&[
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
            Docker::exec(&["kill", &format!("myrias_{}", e)]);
            info!("Killed container: myrias_{}", e);
            info!("Restarting container: myrias_{}", e);
            Docker::start_container(&e);
            info!("Restarted container: myrias_{}", e);

            return Err(Status::GatewayTimeout);
        }
        Err(RecvTimeoutError::Disconnected) => {
            return Err(Status::InternalServerError);
        }
    };

    info!("Finished eval in container: myrias_{}", e);

    let res = if output.stderr.is_empty() {
        String::from_utf8_lossy(&output.stdout)
    } else {
        String::from_utf8_lossy(&output.stderr)
    };

    info!("Removing unique eval folder in container: myrias_{}", e);
    Docker::exec(&[
        "exec",
        &format!("myrias_{}", e),
        "rm",
        "-rf",
        &format!("eval/{}", &snowflake),
    ]);
    info!("Removed unique eval folder in container: myrias_{}", e);

    Ok(json!({ "result": res.to_string() }))
}
