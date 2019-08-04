use log::info;
use rocket::get;
use rocket_contrib::{json, json::JsonValue};
use std::process::Command;

#[get("/containers")]
pub fn index() -> JsonValue {
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
        return json!([] as [String; 0]);
    };

    json!(res)
}
