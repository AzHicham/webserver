use crate::imgsrv::route::{compatible_file_extensions, slide_size, tile};
use actix_web::{web, Scope};

mod errors;
mod route;
mod schemas;

pub fn config() -> Scope {
    web::scope("/imgsrv")
        .service(compatible_file_extensions)
        .service(slide_size)
        .service(tile)
}
