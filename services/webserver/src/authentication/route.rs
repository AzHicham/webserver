use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;

#[get("/login")]
pub(crate) async fn login() -> Value {
    json!({})
}
