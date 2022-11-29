use crate::utils::errors::MyError;
use async_trait::async_trait;
use serde_json::json;
use ulid::Ulid;

/// represent training detail and strength.
#[derive(Debug, Clone, PartialEq)]
pub struct Patient {
    pub id: String,
    pub code: String,
    pub name: String,
}
const NAME_LIMIT: i32 = 30;

impl Patient {
    pub fn new(name: String, code: Option<String>) -> Result<Self, MyError> {
        if name.chars().count() as i32 > NAME_LIMIT {
            return Err(MyError::BadRequest(
                json!({"error":"patient name must be less than 30 letters"}),
            ));
        };
        let id = Ulid::new().to_string();
        let code = if let Some(code) = code {
            code
        } else {
            Ulid::new().to_string()
        };
        Ok(Self { id, code, name })
    }

    pub fn from(id: String, code: String, name: String) -> Patient {
        Self { id, code, name }
    }
}
#[async_trait]
pub trait PatientRepository {
    /// store Patient to DB.
    async fn save(&self, patient: &Patient) -> Result<(), MyError>;
    /// find one Patient from DB by primary key. return Patient. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<Patient, MyError>;
    async fn fetch_by_code(&self, code: &String) -> Result<Patient, MyError>;
    async fn fetch_all(&self) -> Result<Vec<Patient>, MyError>;
}

#[cfg(test)]

mod tests {

    use super::*;
    #[test]
    fn test_patient_new() {
        let test_name = "x".to_string().repeat(30);
        let patient = Patient::new(test_name.clone(), None).unwrap();
        assert_eq!(patient.name, test_name);

        let test_code = "y".to_string().repeat(30);
        let patient = Patient::new(test_name.clone(), Some(test_code.clone())).unwrap();
        assert_eq!(patient.name, test_name);
        assert_eq!(patient.code, test_code);
    }

    #[test]
    fn test_patient_new_failed() {
        let test_name = "x".to_string().repeat((NAME_LIMIT + 1) as usize);
        let patient = Patient::new(test_name, None).unwrap_err();
        assert_eq!(
            patient,
            MyError::BadRequest(json!({"error":"patient name must be less than 30 letters"}))
        );
    }
}
