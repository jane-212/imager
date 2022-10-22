use serde_json::json;

use thiserror::Error;

use actix_web::{
    ResponseError,
    http::StatusCode,
    HttpResponse
};

use std::result;

#[derive(Error, Debug)]
pub enum IError {
    #[error("database error")]
    Database,
}

pub type IResult<T> = result::Result<T, IError>;

impl ResponseError for IError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok()
            .json(json!({
                "code": -1,
                "msg": self.to_string(),
            }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            IError::Database => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
