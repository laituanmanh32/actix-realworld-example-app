use actix_web::{
    error::ResponseError,
    HttpResponse,
};
use diesel::{
    result::{
        DatabaseErrorKind,
        Error as DieselError,
    },
    r2d2::PoolError,
};
use libreauth::pass::ErrorCode as PassErrorCode;
use validator::{
    ValidationError,
    ValidationErrors
};
use std::convert::From;

// more error types can be found at below link but we should only need these for now
// https://actix.rs/actix-web/actix_web/struct.HttpResponse.html
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError
            }
            _ => Error::InternalServerError
        }
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(error: PassErrorCode) -> Self {
        Error::BadRequest(format!("Invalid password provided.\n{:?}", error))
    }
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Error::BadRequest(format!("Validation failed on some constraint.\n{:?}", error))
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        Error::BadRequest(format!("Validation failed on some fields.\n{:?}", errors))
    }
}
