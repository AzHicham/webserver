use actix_web::body::BoxBody;
use actix_web::web::Bytes;
use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TileInfo {
    pub path: String,
    pub col: u32,
    pub row: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub level: u32,
    pub format: String,
    pub quality: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SlideInfo {
    pub mpp_max: Option<f32>,
    pub level_count: u16,
    pub level_dimensions: Vec<(u32, u32)>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Roi {
    pub col: u16,
    pub row: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Image {
    pub data: Vec<u8>,
}

impl Responder for Image {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::png())
            .body(Bytes::from(self.data))
    }
}
