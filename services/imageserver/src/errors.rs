use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::sync::PoisonError;
use thiserror::Error;

use openslide_rs::errors::OpenSlideError;

#[derive(Error, Debug)]
pub enum ImageServerError {
    #[error("Requested file was not found")]
    NotFound,
    #[error("You are forbidden to access requested file.")]
    Forbidden,
    #[error("Invalid level")]
    InvalidSlideLevel,
    #[error("UnsupportedFormat")]
    UnsupportedFormat,
    #[error("Internal Server Error")]
    Internal,
    #[error("Config Error")]
    ConfigError,
    #[error("Unknown Internal Server Error")]
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct ResponseError {
    code: u16,
    message: String,
}

impl error::ResponseError for ImageServerError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ResponseError {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_openslide_error(_err: OpenSlideError) -> ImageServerError {
    ImageServerError::NotFound
}

pub fn map_lock_error<T>(_: PoisonError<T>) -> ImageServerError {
    ImageServerError::Internal
}

pub fn map_io_error(err: std::io::Error) -> ImageServerError {
    match err.kind() {
        std::io::ErrorKind::NotFound => ImageServerError::NotFound,
        std::io::ErrorKind::PermissionDenied => ImageServerError::Forbidden,
        _ => ImageServerError::Unknown,
    }
}
