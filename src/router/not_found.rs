use rocket::catch;
use rocket_contrib::{json, json::JsonValue};

#[catch(404)]
pub fn index() -> JsonValue {
    json!({ "message": "Language not found." })
}
