use chrono::{DateTime, Local};

use crate::{
    domain::{
        medical_examination::{MedicalExamination, MedicalExaminationRepository},
        patient::PatientRepository,
    },
    utils::errors::MyError,
};

pub struct MedicalExaminationUsecase<M: MedicalExaminationRepository, P: PatientRepository> {
    pub medical_examination_repository: M,
    pub patient_repository: P,
}

impl<M: MedicalExaminationRepository, P: PatientRepository> MedicalExaminationUsecase<M, P> {
    pub fn new(medical_examination_repository: M, patient_repository: P) -> Self {
        Self {
            medical_examination_repository,
            patient_repository,
        }
    }

    pub async fn create_medical_examination(
        &self,
        interviewed_at: Option<DateTime<Local>>,
        user_id: String,
        patient_code: String,
        symptom: String,
    ) -> Result<(), MyError> {
        let medical_examination = MedicalExamination::new(symptom, interviewed_at);
        let _ = self.patient_repository.fetch_by_code(&patient_code).await?;
        let _ = self
            .medical_examination_repository
            .save(&user_id, &patient_code, &medical_examination)
            .await?;
        Ok(())
    }

    pub async fn fetch_one(&self, id: &String) -> Result<MedicalExamination, MyError> {
        let medical_examination = self.medical_examination_repository.fetch_one(id).await?;
        Ok(medical_examination)
    }

    pub async fn fetch_by_patient_code(
        &self,
        patient_code: String,
    ) -> Result<Vec<MedicalExamination>, MyError> {
        let medical_examinations = self
            .medical_examination_repository
            .fetch_by_patient_code(&patient_code)
            .await?;
        Ok(medical_examinations)
    }
}

#[cfg(test)]

mod tests {

    use crate::{
        repository::{
            medical_examination_repository::{
                get_medical_examinations, MedicalExaminationRepositoryMockImpl,
            },
            patient_repository::{get_patients, PatientRepositoryMockImpl},
        },
        utils::datetime::DATETIME_FMT,
    };
    use chrono::{Local, TimeZone};

    use super::*;

    #[tokio::test]
    async fn test_create_medical_examination() {
        let code = "test_code".to_string();
        let interviewed_at = Local
            .datetime_from_str("2022-12-12 12:12:12", DATETIME_FMT)
            .unwrap();
        let user_id = "1".to_string();
        let symptom = "headache".to_string();
        let mock_patient_repository = PatientRepositoryMockImpl {};
        let mock_medical_examination_repository = MedicalExaminationRepositoryMockImpl {};
        let medical_examination_usecase = MedicalExaminationUsecase {
            patient_repository: mock_patient_repository,
            medical_examination_repository: mock_medical_examination_repository,
        };
        medical_examination_usecase
            .create_medical_examination(Some(interviewed_at), user_id, code, symptom)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_fetch_by_patient_code() {
        let code = "test_code".to_string();
        let mock_patient_repository = PatientRepositoryMockImpl {};
        let mock_medical_examination_repository = MedicalExaminationRepositoryMockImpl {};
        let medical_examination_usecase = MedicalExaminationUsecase {
            patient_repository: mock_patient_repository,
            medical_examination_repository: mock_medical_examination_repository,
        };
        let medical_examinations = medical_examination_usecase
            .fetch_by_patient_code(code)
            .await
            .unwrap();
        assert_eq!(medical_examinations, get_medical_examinations())
    }
}
