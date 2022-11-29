use actix_web::{http::header::HeaderValue, HttpRequest};
use chrono::Utc;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::errors::MyError;

static SECRETE_KEY: [u8; 16] = *include_bytes!("../../secret.key");
const TOKEN_EXPIRED_WITHIN: i64 = 60 * 60;
const AUTORIZATION_HEADER: &str = "autorization";
const BEARER: &str = "Bearer";

/// extracting user_id from request header.
pub fn get_user_id_from_header(req: &HttpRequest) -> Result<String, MyError> {
    let authorization = req.headers().get(AUTORIZATION_HEADER);
    let token = validate_and_extract_header(authorization)?;
    let decoded_token = decode(token)?;
    Ok(decoded_token.claims.user_id)
}

fn validate_and_extract_header(authorization: Option<&HeaderValue>) -> Result<&str, MyError> {
    // check authorization header,Bearer schema,token exists
    let token = match authorization {
        None => {
            return Err(MyError::Unauthorized(
                json!({"error":"authorization is not found"}),
            ))
        }
        Some(token) => token.to_str().unwrap(),
    };

    let mut splited_token = token.split_whitespace();

    // check Bearer schema
    match splited_token.next() {
        Some(schema) => {
            if schema != BEARER {
                return Err(MyError::Unauthorized(
                    json!({"error":"invalid schema type"}),
                ));
            }
        }
        None => {
            return Err(MyError::Unauthorized(
                json!({"error":"not found schema type"}),
            ))
        }
    }

    // check jwt token
    let jwt = match splited_token.next() {
        Some(jwt) => jwt,
        None => {
            return Err(MyError::Unauthorized(
                json!({"error":"not found jwt token"}),
            ))
        }
    };
    Ok(jwt)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iat: i64,
    exp: i64,
    user_id: String,
}

impl Claims {
    pub fn new(user_id: &String, now: i64) -> Self {
        Claims {
            iat: now,
            exp: now + TOKEN_EXPIRED_WITHIN,
            user_id: user_id.clone(),
        }
    }
}

pub fn make_jwt(user_id: &String) -> Result<String, MyError> {
    let now = Utc::now();
    let claims = Claims::new(user_id, now.timestamp());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&SECRETE_KEY),
    )?;
    Ok(token)
}

pub fn decode(token: &str) -> Result<TokenData<Claims>, MyError> {
    let decoded_token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(&SECRETE_KEY),
        &Validation::default(),
    )?;
    Ok(decoded_token)
}
