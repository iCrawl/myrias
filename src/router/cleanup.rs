use log::info;
use rocket::post;
use rocket_contrib::{json, json::JsonValue};
use std::process::Command;

#[post("/cleanup")]
pub fn index() -> JsonValue {
    info!("Getting all myrias container");
    let ouput = Command::new("docker")
        .args(&["ps", "--filter", "name=myrias_", "--format", "{{.Names}}"])
        .output()
        .unwrap();
    info!("Finished getting all myrias container");

    let mut res: Vec<String> = if ouput.stderr.is_empty() {
        String::from_utf8_lossy(&ouput.stdout)
            .lines()
            .map(|x| x.trim().to_string())
            .collect()
    } else {
        return json!([] as [String; 0]);
    };

    res.insert(0, "kill".to_string());
    let output = Command::new("docker").args(res).output().unwrap();

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
