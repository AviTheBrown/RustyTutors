use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum TutorError {
    DBError(String),
    ActixServerError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    error_message: String,
}

impl fmt::Display for TutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_response())
    }
}
impl From<SQLxError> for TutorError {
    fn from(value: SQLxError) -> Self {
        TutorError::DBError(value.to_string())
    }
}
impl TutorError {
    fn error_response(&self) -> String {
        match self {
            TutorError::DBError(msg) => {
                println!("Datbase Error Occurred {:?}", msg);
                msg.into()
            }
            TutorError::ActixServerError(msg) => {
                println!("Interneral Server Error Occurred {:?}", msg);
                msg.into()
            }
            TutorError::NotFound(msg) => {
                println!("Page Not Found {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for TutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            TutorError::DBError(msg) | TutorError::ActixServerError(msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TutorError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorMessage {
            error_message: self.error_response(),
        })
    }
}
