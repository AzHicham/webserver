use crate::common::route::status;
use actix_web::{web, Scope};

mod route;
pub(crate) mod schemas;

pub fn config() -> Scope {
    web::scope("").service(status)
}
