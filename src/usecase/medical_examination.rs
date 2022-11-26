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
        let _ = self.patient_repository.fetch_one(&patient_code).await?;
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

    pub async fn fetch_by_patient_id(
        &self,
        patient_id: String,
    ) -> Result<Vec<MedicalExamination>, MyError> {
        let medical_examinations = self
            .medical_examination_repository
            .fetch_by_patient_id(&patient_id)
            .await?;
        Ok(medical_examinations)
    }
}
