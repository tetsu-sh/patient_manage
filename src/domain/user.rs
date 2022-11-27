use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub code: String,
    pub name: String,
    pub hashed_password: String,
}

const NAME_LIMIT: i32 = 30;

impl User {
    pub fn new(
        name: String,
        code: Option<String>,
        hashed_password: String,
    ) -> Result<Self, MyError> {
        let id = Ulid::new().to_string();
        if name.chars().count() as i32 > NAME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"train name must be less than 30 letters"}),
            ));
        };
        let code = if let Some(code) = code {
            code
        } else {
            Ulid::new().to_string()
        };
        Ok(Self {
            id,
            code,
            name,
            hashed_password,
        })
    }
    pub fn from(
        id: String,
        code: String,
        name: String,
        hashed_password: String,
    ) -> Result<User, MyError> {
        let user = User {
            id,
            code,
            name,
            hashed_password,
        };
        Ok(user)
    }
}

#[async_trait]
pub trait UserRepository {
    /// store user to DB.
    async fn save(&self, user: &User) -> Result<(), MyError>;
    /// find one user from DB by primary key. return user. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<User, MyError>;
    async fn find_by_name(&self, name: &String) -> Result<User, MyError>;
}

#[async_trait]
pub trait DoctorInChargeRepository {
    /// store Patient to DB.
    async fn save(&self, user_id: &String, patient_code: &String) -> Result<(), MyError>;
}

#[cfg(test)]

mod tests {

    use crate::utils::hash::hash_password;

    use super::*;
    #[test]
    fn test_user_new() {
        let test_name = "x".to_string().repeat(30);
        let test_code = "y".to_string().repeat(30);
        let test_hashed_password = hash_password("aaaaaaaaa").unwrap();
        let user = User::new(
            test_name.clone(),
            Some(test_code.clone()),
            test_hashed_password.clone(),
        )
        .unwrap();
        assert_eq!(user.name, test_name);
        assert_eq!(user.code, test_code);
        assert_eq!(user.hashed_password, test_hashed_password);
        user.id;
    }

    #[test]
    fn test_user_new_failed() {
        let test_name = "x".to_string().repeat((NAME_LIMIT + 1) as usize);
        let test_code = "y".to_string().repeat((NAME_LIMIT) as usize);
        let test_hashed_password = hash_password("aaaaaaaaa").unwrap();
        let user = User::new(test_name, Some(test_code), test_hashed_password).unwrap_err();
        assert_eq!(
            user,
            MyError::BadRequest(json!({"error":"train name must be less than 30 letters"}))
        );
    }
}
