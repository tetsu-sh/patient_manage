use actix_web::{web, HttpRequest};
use chrono::{DateTime, Local};

use crate::repository::medical_examination_repository::MedicalExaminationRepositoryImpl;
use crate::repository::patient_repository::PatientRepositoryImpl;
use crate::usecase::medical_examination::MedicalExaminationUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::medical_examination::MedicalExamination, middleware::authn};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateMedicalExaminationRequest {
    symptom: String,
    patient_code: String,
    interviewed_at: Option<DateTime<Local>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateMedicalExaminationResponse {}

impl CreateMedicalExaminationResponse {
    fn from() -> Self {
        Self {}
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchMedicalExaminationsParameter {
    patient_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchMedicalExaminationsResponse {
    medical_examinations: Vec<FetchMedicalExamination>,
}

#[derive(Deserialize, Serialize)]
pub struct FetchMedicalExamination {
    interviewed_at: Option<DateTime<Local>>,
    symptom: String,
}

impl FetchMedicalExamination {
    fn from(medical_examination: MedicalExamination) -> Self {
        Self {
            interviewed_at: medical_examination.interviewed_at,
            symptom: medical_examination.symptom,
        }
    }
}

impl FetchMedicalExaminationsResponse {
    fn from(medical_examinaions: Vec<MedicalExamination>) -> Self {
        let medical_examinations = medical_examinaions
            .into_iter()
            .map(|medical_examination| FetchMedicalExamination::from(medical_examination))
            .collect::<Vec<FetchMedicalExamination>>();
        Self {
            medical_examinations,
        }
    }
}

pub async fn create_medical_examination(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateMedicalExaminationRequest>,
) -> ApiResponse {
    // object setting.
    let conn = state.get_sqls_db_conn()?;
    let user_id = authn::get_user_id_from_header(&req)?;
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_usecase = MedicalExaminationUsecase {
        medical_examination_repository,
        patient_repository,
    };

    medical_examination_usecase
        .create_medical_examination(
            form.interviewed_at.clone(),
            user_id.clone(),
            form.patient_code.clone(),
            form.symptom.clone(),
        )
        .await?;
    let create_medical_examination_response = CreateMedicalExaminationResponse::from();
    Ok(HttpResponse::Ok().json(create_medical_examination_response))
}

pub async fn fetch_medical_examinations(
    state: web::Data<AppState>,
    req: HttpRequest,
    params: web::Query<FetchMedicalExaminationsParameter>,
) -> ApiResponse {
    // object setting.
    let conn = state.get_sqls_db_conn()?;
    let _ = authn::get_user_id_from_header(&req)?;
    let medical_examination_repository = MedicalExaminationRepositoryImpl { conn: &conn };
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let medical_examination_usecase = MedicalExaminationUsecase {
        medical_examination_repository,
        patient_repository,
    };

    let medical_examinations = medical_examination_usecase
        .fetch_by_patient_code(params.patient_code.clone())
        .await?;
    let fetch_medical_examination_response =
        FetchMedicalExaminationsResponse::from(medical_examinations);
    Ok(HttpResponse::Ok().json(fetch_medical_examination_response))
}
