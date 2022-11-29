use crate::utils::errors::MyError;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// 問診情報
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MedicalExamination {
    pub id: String,
    // 問診日
    pub interviewed_at: Option<DateTime<Local>>,
    // 症状
    pub symptom: String,
}

impl MedicalExamination {
    pub fn new(symptom: String, interviewed_at: Option<DateTime<Local>>) -> Self {
        let id = Ulid::new().to_string();
        Self {
            id,
            interviewed_at,
            symptom,
        }
    }
    pub fn from(
        id: String,
        symptom: String,
        interviewed_at: Option<DateTime<Local>>,
    ) -> Result<MedicalExamination, MyError> {
        let medical_examination = MedicalExamination {
            id,
            symptom,
            interviewed_at,
        };
        Ok(medical_examination)
    }
}

#[async_trait]
pub trait MedicalExaminationRepository {
    /// store MedicalExamination to DB.
    async fn save(
        &self,
        user_id: &String,
        patient_code: &String,
        medical_examination: &MedicalExamination,
    ) -> Result<(), MyError>;
    /// find one MedicalExamination from DB by primary key. return MedicalExamination. if not exist,None.
    async fn fetch_one(&self, id: &String) -> Result<MedicalExamination, MyError>;
    async fn fetch_by_patient_code(
        &self,
        patient_code: &String,
    ) -> Result<Vec<MedicalExamination>, MyError>;
}

#[cfg(test)]

mod tests {

    use chrono::TimeZone;

    use crate::utils::datetime;

    use super::*;
    #[test]
    fn test_patient_new() {
        let test_symptom = "x".to_string().repeat(30);
        let test_str_interviewed_at = "2022-12-12 12:12:12";
        let test_interviewed_at = Local
            .datetime_from_str(test_str_interviewed_at, datetime::DATETIME_FMT)
            .unwrap();
        let medical_examination =
            MedicalExamination::new(test_symptom.clone(), Some(test_interviewed_at));
        assert_eq!(medical_examination.symptom, test_symptom);
    }
}
