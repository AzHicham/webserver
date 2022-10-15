use crate::common::schemas::DefaultPayload;
use rocket::serde::json::{serde_json::json, Json, Value};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[post("/hello/<id>", format = "application/json", data = "<payload>")]
pub(crate) async fn analyze(id: String, payload: Json<DefaultPayload>) -> Json<DefaultPayload> {
    info!("ID, {}!", &id);
    info!("payload, {:?}!", &payload);

    payload
}

#[get("/status")]
pub(crate) async fn status() -> Value {
    json!({
        "version": VERSION,
        "status": "ok",
    })
}
