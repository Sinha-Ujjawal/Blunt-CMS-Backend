use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum MyError {
    DieselError(diesel::result::Error),

    #[display(fmt = "User Already Exists!")]
    UserAlreadyExists,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, json) = match serde_json::to_string(&ErrorResponse {
            error: self.to_string(),
        }) {
            Ok(json) => (self.status_code(), json),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                r#"{"error": "Error parsing the response!"}"#.to_string(),
            ),
        };

        HttpResponse::build(status_code)
            .insert_header(ContentType::json())
            .body(json)
    }

    fn status_code(&self) -> StatusCode {
        use MyError::*;
        match *self {
            DieselError(_) => StatusCode::BAD_REQUEST,
            UserAlreadyExists => StatusCode::BAD_REQUEST,
        }
    }
}
