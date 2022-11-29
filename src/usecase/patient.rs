use chrono::{DateTime, Local};

use crate::{
    domain::medical_examination::MedicalExaminationRepository,
    domain::{
        medical_examination::{self, MedicalExamination},
        patient::{Patient, PatientRepository},
    },
    utils::errors::MyError,
};

pub struct PatientUsecase<P: PatientRepository, M: MedicalExaminationRepository> {
    pub patient_repository: P,
    pub medical_examination_repository: M,
}

impl<T: PatientRepository, M: MedicalExaminationRepository> PatientUsecase<T, M> {
    pub fn new(patient_repository: T, medical_examination_repository: M) -> Self {
        Self {
            patient_repository,
            medical_examination_repository,
        }
    }

    /// create new patient.
    pub async fn create_patient(
        &self,
        name: String,
        code: Option<String>,
    ) -> Result<Patient, MyError> {
        let patient = Patient::new(name, code)?;
        let _ = self.patient_repository.save(&patient).await?;
        Ok(patient)
    }
    /// create new patient.
    pub async fn create_patient_with_medical_examination(
        &self,
        name: String,
        code: Option<String>,
        interviewed_at: Option<DateTime<Local>>,
        user_id: String,
        symptom: String,
    ) -> Result<Patient, MyError> {
        let patient = Patient::new(name, code.clone())?;
        let medical_examination = MedicalExamination::new(symptom, interviewed_at);
        self.patient_repository.save(&patient).await?;

        self.medical_examination_repository
            .save(&user_id, &patient.code, &medical_examination)
            .await?;

        Ok(patient)
    }

    pub async fn fetch_one(&self, id: &String) -> Result<Patient, MyError> {
        self.patient_repository.fetch_one(id).await
    }

    pub async fn fetch_patients(&self) -> Result<Vec<Patient>, MyError> {
        self.patient_repository.fetch_all().await
    }
}

#[cfg(test)]

mod tests {

    use crate::{
        repository::{
            medical_examination_repository::MedicalExaminationRepositoryMockImpl,
            patient_repository::{get_patients, PatientRepositoryMockImpl},
        },
        utils::datetime::DATETIME_FMT,
    };
    use chrono::{Local, TimeZone};

    use super::*;

    #[tokio::test]
    async fn test_create_patient() {
        let name = "test_name".to_string();
        let code = "test_code".to_string();
        let mock_patient_repository = PatientRepositoryMockImpl {};
        let mock_medical_examination_repository = MedicalExaminationRepositoryMockImpl {};
        let patient_usecase = PatientUsecase {
            patient_repository: mock_patient_repository,
            medical_examination_repository: mock_medical_examination_repository,
        };
        patient_usecase
            .create_patient(name, Some(code))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_patient_with_medical_examination() {
        let name = "test_name".to_string();
        let code = "test_code".to_string();
        let interviewed_at = Local
            .datetime_from_str("2022-12-12 12:12:12", DATETIME_FMT)
            .unwrap();
        let user_id = "1".to_string();
        let symptom = "headache".to_string();
        let mock_patient_repository = PatientRepositoryMockImpl {};
        let mock_medical_examination_repository = MedicalExaminationRepositoryMockImpl {};
        let patient_usecase = PatientUsecase {
            patient_repository: mock_patient_repository,
            medical_examination_repository: mock_medical_examination_repository,
        };
        patient_usecase
            .create_patient_with_medical_examination(
                name,
                Some(code),
                Some(interviewed_at),
                user_id,
                symptom,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_fetch_patients() {
        let mock_patient_repository = PatientRepositoryMockImpl {};
        let mock_medical_examination_repository = MedicalExaminationRepositoryMockImpl {};
        let patient_usecase = PatientUsecase {
            patient_repository: mock_patient_repository,
            medical_examination_repository: mock_medical_examination_repository,
        };
        let patients = patient_usecase.fetch_patients().await.unwrap();
        assert_eq!(patients, get_patients())
    }
}
