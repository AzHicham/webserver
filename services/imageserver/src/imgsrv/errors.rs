use crate::common::schemas::ErrorMessage;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, web::Json, HttpResponse, Responder};
use derive_more::{Display, Error};
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Display, Error)]
pub enum ImageServerError {
    #[display(fmt = "internal error")]
    IoError,
}

impl error::ResponseError for ImageServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ImageServerError::IoError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
