use crate::utils::errors::MyError;
use serde_json::json;

static MIN_USER_PASSWORD_LENGTH: usize = 8;

pub fn verify_user_password(raw_password: &str) -> Result<(), MyError> {
    if raw_password.len() < MIN_USER_PASSWORD_LENGTH {
        return Err(MyError::BadRequest(json!({"error":"password too short"})));
    }
    Ok(())
}
