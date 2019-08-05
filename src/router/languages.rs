use log::info;
use rocket::{get, State};
use rocket_contrib::{json, json::JsonValue};

use crate::Config;

#[get("/languages", format = "json")]
pub fn index(config: State<Config>) -> JsonValue {
    info!("Listing all myrias enabled languages");
    json!(&config.languages)
}
