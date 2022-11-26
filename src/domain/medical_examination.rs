use crate::utils::errors::MyError;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MedicalExamination {
    pub id: String,
    pub interviewed_at: Option<DateTime<Local>>,
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
    async fn fetch_by_patient_id(
        &self,
        patient_id: &String,
    ) -> Result<Vec<MedicalExamination>, MyError>;
}
