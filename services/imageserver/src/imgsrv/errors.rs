use crate::common::schemas::ErrorMessage;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{response, Response};
use std::error::Error;
use std::io::Cursor;
use std::{fmt, io};

#[derive(Debug, Clone, Responder)]
pub enum ImageServerError {
    #[response(status = 404)]
    IoError(Json<ErrorMessage>),
}

pub(crate) fn from_io_error(err: io::Error) -> ImageServerError {
    ImageServerError::IoError(Json(ErrorMessage {
        message: err.to_string(),
    }))
}
