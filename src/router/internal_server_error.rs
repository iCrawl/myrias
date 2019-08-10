use rocket::catch;
use rocket_contrib::{json, json::JsonValue};

#[catch(504)]
pub fn index() -> JsonValue {
    json!({ "message": "Evaluation failed." })
}
