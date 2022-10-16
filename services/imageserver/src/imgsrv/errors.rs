use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error, Clone)]
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
