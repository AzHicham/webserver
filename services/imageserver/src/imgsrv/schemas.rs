use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::{Deserialize, Serialize};
use rocket::{response, Request, Response};
use std::io::Cursor;

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

impl<'r> Responder<'r, 'static> for Image {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(ContentType::PNG)
            .sized_body(self.data.len(), Cursor::new(self.data))
            .ok()
    }
}
