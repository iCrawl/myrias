use actix_web::{HttpResponse, Responder};
use log::info;
use std::process::Command;

pub fn index() -> impl Responder {
    info!("Listing all myrias container");
    let output = Command::new("docker")
        .args(&["ps", "--filter", "name=myrias_", "--format", "{{.Names}}"])
        .output()
        .unwrap();
    info!("Finished listing all myrias container");

    let res: Vec<String> = if output.stderr.is_empty() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|x| x.trim().to_string())
            .collect()
    } else {
        return HttpResponse::Ok().json([] as [String; 0]);
    };

    HttpResponse::Ok().json(res)
}
