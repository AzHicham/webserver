use actix_web::body::BoxBody;
use actix_web::web::Bytes;
use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TileRequestInput {
    pub path: String,
    pub col: u32,
    pub row: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub level: u32,
    pub format: String,
    pub quality: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ThumbnailRequestInput {
    pub path: String,
    pub width: u32,
    pub height: u32,
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
    pub col: u32,
    pub row: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct EncodedImage {
    pub data: Vec<u8>,
    pub format: EncodeType,
}

#[derive(Debug)]
pub enum EncodeType {
    Png,
    Jpeg,
}

#[derive(Debug)]
pub enum EncodeQuality {
    Low,
    Medium,
    High,
}

impl Responder for EncodedImage {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::png())
            .body(Bytes::from(self.data))
    }
}

impl FromStr for EncodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<EncodeType, Self::Err> {
        match input {
            "png" => Ok(EncodeType::Png),
            "jpg" | "jpeg" => Ok(EncodeType::Jpeg),
            _ => Err(()),
        }
    }
}
