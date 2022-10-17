use crate::imgsrv::route::{
    compatible_file_extensions, regions_of_interest, slide_info, slide_size, thumbnail, tile,
};
use actix_web::{web, Scope};

mod route;
mod schemas;
mod utils;

pub fn config() -> Scope {
    web::scope("/imgsrv")
        .service(tile)
        .service(slide_size)
        .service(regions_of_interest)
        .service(slide_info)
        .service(compatible_file_extensions)
        .service(thumbnail)
}
