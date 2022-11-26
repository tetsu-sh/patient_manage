use log::info;

use crate::{
    domain::medical_examination::MedicalExaminationRepository,
    domain::patient::{Patient, PatientRepository},
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
        info!("create_patient");
        let patient = Patient::new(name, code)?;
        let _ = self.patient_repository.save(&patient).await?;
        Ok(patient)
    }

    pub async fn fetch_one(&self, id: &String) -> Result<Patient, MyError> {
        self.patient_repository.fetch_one(id).await
    }

    pub async fn fetch_patients(&self) -> Result<Vec<Patient>, MyError> {
        self.patient_repository.fetch_all().await
    }
}
