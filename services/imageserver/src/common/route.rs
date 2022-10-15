use crate::common::schemas::Status;
use actix_web::{get, web::Json};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/status")]
pub(crate) async fn status() -> Json<Status> {
    Json(Status {
        status: "healthy".to_string(),
        version: VERSION.to_string(),
    })
}
