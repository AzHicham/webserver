use rocket::serde::json::{serde_json::json, Value};

#[get("/login")]
pub(crate) async fn login() -> Value {
    json!({})
}
