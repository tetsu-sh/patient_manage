use actix_web::error::ContentTypeError;
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use bcrypt::BcryptError;
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::json;
use serde_json::Value as JsonValue;
use sqlx::Error as SqlxError;
use strum::ParseError as StrumParseError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MyError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Not Found")]
    NotFound(JsonValue),
    #[error("Bad Request")]
    BadRequest(JsonValue),
    #[error("Unprocessable Entity")]
    UnprocessableEntity(JsonValue),
    #[error("Unauthorized")]
    Unauthorized(JsonValue),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            MyError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            MyError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            MyError::BadRequest(ref msg) => HttpResponse::BadRequest().json(msg),
            MyError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MyError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::BadRequest(_) => StatusCode::BAD_REQUEST,
            MyError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            MyError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<ContentTypeError> for MyError {
    fn from(err: ContentTypeError) -> Self {
        match err {
            ContentTypeError::ParseError => {
                MyError::NotFound(json!({"error":"invalid content type"}))
            }
            ContentTypeError::UnknownEncoding => MyError::NotFound(json!({"error":"dencode"})),
            _ => MyError::InternalServerError,
        }
    }
}

impl From<SqlxError> for MyError {
    fn from(err: SqlxError) -> Self {
        println!("error:{}", err);
        MyError::InternalServerError
    }
}

impl From<StrumParseError> for MyError {
    fn from(err: StrumParseError) -> Self {
        MyError::BadRequest(json!({ "error": err.to_string() }))
    }
}

impl From<JwtError> for MyError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            JwtErrorKind::InvalidToken => {
                MyError::Unauthorized(json!({"error":"Token is invalid"}))
            }
            JwtErrorKind::InvalidIssuer => {
                MyError::Unauthorized(json!({"error":"Issur is invalid"}))
            }
            _ => MyError::Unauthorized(json!({
                "error": format!("problem except token and issue {}", err.to_string())
            })),
        }
    }
}

impl From<BcryptError> for MyError {
    fn from(err: BcryptError) -> Self {
        MyError::Unauthorized(json!({
            "error": err.to_string()
        }))
    }
}
