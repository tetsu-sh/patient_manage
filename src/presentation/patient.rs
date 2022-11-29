use actix_web::{web, HttpRequest};
use chrono::{DateTime, Local};

use crate::domain::patient::Patient;
use crate::repository::medical_examination_repository::MedicalExaminationRepositoryImpl;
use crate::repository::patient_repository::PatientRepositoryImpl;
use crate::usecase::patient::PatientUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::medical_examination::MedicalExamination, middleware};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePatientRequest {
    name: String,
    code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePatientResponse {
    code: String,
}

impl From<Patient> for CreatePatientResponse {
    fn from(patient: Patient) -> Self {
        Self { code: patient.code }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePatientWithMedicalExaminationRequest {
    name: String,
    code: Option<String>,
    interviewed_at: Option<DateTime<Local>>,
    symptom: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePatientWithMedicalExaminationResponse {
    code: String,
}

impl From<Patient> for CreatePatientWithMedicalExaminationResponse {
    fn from(patient: Patient) -> Self {
        Self { code: patient.code }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchPatientParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPatientResponse {
    name: String,
    code: String,
}

impl FetchPatientResponse {
    fn from(patient: Patient) -> Self {
        Self {
            code: patient.code,
            name: patient.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchPatientsParameter {}

#[derive(Deserialize, Serialize)]
pub struct FetchPatientsResponse {
    patients: Vec<FetchPatientResponse>,
}

impl FetchPatientsResponse {
    fn from(patients: Vec<Patient>) -> Self {
        let patients = patients
            .into_iter()
            .map(|patient| FetchPatientResponse::from(patient))
            .collect::<Vec<FetchPatientResponse>>();
        Self { patients }
    }
}

pub type ApiResponse = Result<HttpResponse, MyError>;

pub async fn create_patient(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreatePatientRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let _ = middleware::authn::get_user_id_from_header(&req)?;

    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let patient_usecase = PatientUsecase {
        patient_repository,
        medical_examination_repository,
    };

    let patient = patient_usecase
        .create_patient(form.name.clone(), form.code.clone())
        .await?;
    let create_patient_response = CreatePatientResponse::from(patient);
    Ok(HttpResponse::Ok().json(create_patient_response))
}

pub async fn create_patient_with_medical_examination(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreatePatientWithMedicalExaminationRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_id = middleware::authn::get_user_id_from_header(&req)?;
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let patient_usecase = PatientUsecase {
        patient_repository,
        medical_examination_repository,
    };

    let patient = patient_usecase
        .create_patient_with_medical_examination(
            form.name.clone(),
            form.code.clone(),
            form.interviewed_at,
            user_id,
            form.symptom.clone(),
        )
        .await?;
    let create_patient_response = CreatePatientWithMedicalExaminationResponse::from(patient);
    Ok(HttpResponse::Ok().json(create_patient_response))
}

pub async fn fetch_patient(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchPatientParameter>,
) -> ApiResponse {
    let mut conn = state.get_sqls_db_conn()?;
    let _ = middleware::authn::get_user_id_from_header(&req)?;
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let patient_usecase = PatientUsecase::new(patient_repository, medical_examination_repository);
    let patient = patient_usecase.fetch_one(&params.id).await?;
    let res = FetchPatientResponse::from(patient);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn fetch_patients(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchPatientsParameter>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let _ = middleware::authn::get_user_id_from_header(&req)?;
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let train_usecase = PatientUsecase {
        patient_repository,
        medical_examination_repository,
    };
    let patients = train_usecase.fetch_patients().await?;
    let res = FetchPatientsResponse::from(patients);
    Ok(HttpResponse::Ok().json(res))
}
