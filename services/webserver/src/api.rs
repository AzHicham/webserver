use celery::broker::RedisBroker;
use celery::Celery;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Arc;
use tracing::info;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/status")]
pub(crate) async fn status() -> Value {
    json!({
        "version": VERSION,
        "status": "ok",
    })
}

#[post("/hello/<id>", format = "application/json", data = "<payload>")]
pub(crate) async fn analyze(
    celery: &State<Arc<Celery<RedisBroker>>>,
    id: String,
    payload: Json<DefaultPayload>,
) -> Value {
    info!("ID, {}!", &id);
    info!("payload, {:?}!", &payload);

    info!("celery hostname, {}!", &celery.hostname);

    let code = 200;
    let features = vec!["serde", "json"];

    let value = json!({
        "code": code,
        "success": code == 200,
        "payload": {
            features[0]: features[1]
        }
    });
    value
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DefaultPayload {
    pub data: String,
    pub id: u16,
}
