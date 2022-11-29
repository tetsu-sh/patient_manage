use serde_json::json;

use crate::{
    domain::user::{DoctorInChargeRepository, User, UserRepository},
    utils::errors::MyError,
};
use async_trait::async_trait;

use sqlx::MySqlPool;

pub struct UserRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl<'_> {
    async fn save(&self, user: &User) -> Result<(), MyError> {
        sqlx::query!(
            "insert into users(id,code,name,password)
            values(?,?,?,?)
            ",
            user.id,
            user.code,
            user.name,
            user.hashed_password
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }

    async fn fetch_one(&self, id: &String) -> Result<User, MyError> {
        let record = sqlx::query!(
            "select id, code,name, password
            from users 
            where users.id=? 
            ",
            id
        )
        .fetch_optional(self.conn)
        .await?;
        if let Some(record) = record {
            let user = User::from(record.id, record.code, record.name, record.password)?;
            Ok(user)
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }

    async fn find_by_code(&self, code: &String) -> Result<User, MyError> {
        let record = sqlx::query!(
            "select id, code, name, password
            from users
            where users.code=?",
            code
        )
        .fetch_one(self.conn)
        .await?;
        let user = User::from(record.id, record.code, record.name, record.password)?;
        Ok(user)
    }
}

pub struct DoctorInChargeRepositoryImpl<'a> {
    pub conn: &'a MySqlPool,
}

#[async_trait]
impl DoctorInChargeRepository for DoctorInChargeRepositoryImpl<'_> {
    async fn save(&self, user_id: &String, patient_code: &String) -> Result<(), MyError> {
        sqlx::query!(
            "insert into doctor_in_charges(user_id,patient_code)
            values(?,?)
            ",
            user_id,
            patient_code,
        )
        .execute(self.conn)
        .await?;
        Ok(())
    }
}

pub struct UserRepositoryMockImpl {}

#[async_trait]
impl UserRepository for UserRepositoryMockImpl {
    /// nothing is done.
    async fn save(&self, user: &User) -> Result<(), MyError> {
        Ok(())
    }
    /// return User::from("test_id", "test_code", "test_name", "test_password")
    /// id is not correct then return Error
    async fn fetch_one(&self, id: &String) -> Result<User, MyError> {
        // let yaml_file = std::fs::read("/repository/fixtures/user.yaml");
        // let yaml = serde_yaml::Deserializer::from_slice(yaml_file);

        let user = get_data();
        if id == &user.id {
            return Ok(user);
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of id={}.", id)
            })));
        }
    }

    /// return User::from("test_id", "test_code", "test_name", "test_password")
    /// name is not correct then return Error
    async fn find_by_code(&self, code: &String) -> Result<User, MyError> {
        let user = get_data();
        if code == &user.code {
            return Ok(user);
        } else {
            return Err(MyError::BadRequest(json!({
                "error": format!("no record of code={}.", code)
            })));
        }
    }
}

fn get_data() -> User {
    User::from(
        "test_id".to_string(),
        "test_code".to_string(),
        "test_name".to_string(),
        "test_password".to_string(),
    )
    .unwrap()
}

pub struct DoctorInChargeRepositoryMockImpl {}

#[async_trait]
impl DoctorInChargeRepository for DoctorInChargeRepositoryMockImpl {
    async fn save(&self, user_id: &String, patient_id: &String) -> Result<(), MyError> {
        Ok(())
    }
}
