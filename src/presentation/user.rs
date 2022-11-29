use actix_web::{web, HttpRequest};

use crate::repository::patient_repository::PatientRepositoryImpl;
use crate::repository::user_repository::{DoctorInChargeRepositoryImpl, UserRepositoryImpl};
use crate::usecase::user::UserUsecase;
use crate::utils::errors::MyError;
use crate::utils::state::AppState;
use crate::{domain::user::User, middleware::authn};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub type ApiResponse = Result<HttpResponse, MyError>;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUpRequest {
    name: String,
    password: String,
    code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SignUpResponse {
    code: String,
    token: String,
}

impl SignUpResponse {
    fn from(user: User, token: String) -> Self {
        Self {
            code: user.code,
            token,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    code: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignInResponse {
    token: String,
}

impl SignInResponse {
    fn from(token: String) -> Self {
        Self { token }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssignRequest {
    patient_code: String,
}

#[derive(Deserialize, Serialize)]
pub struct AssignResponse {}

impl AssignResponse {
    fn from() -> Self {
        Self {}
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FetchUserParameter {
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchUserResponse {
    id: String,
    name: String,
    code: String,
}

impl FetchUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            code: user.code,
        }
    }
}

pub async fn sign_up(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<SignUpRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let doctor_in_charge_repository = DoctorInChargeRepositoryImpl { conn: &conn };

    let user_usecase = UserUsecase {
        user_repository,
        patient_repository,
        doctor_in_charge_repository,
    };

    let (user, token) = user_usecase
        .sign_up(form.name.clone(), form.code.clone(), form.password.clone())
        .await?;
    let create_muscle_response = SignUpResponse::from(user, token);
    Ok(HttpResponse::Ok().json(create_muscle_response))
}

pub async fn sign_in(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<SignInRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_repository = UserRepositoryImpl { conn: &conn };
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let doctor_in_charge_repository = DoctorInChargeRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase {
        user_repository,
        patient_repository,
        doctor_in_charge_repository,
    };

    let token = user_usecase
        .sign_in(form.code.clone(), form.password.clone())
        .await?;
    let fetch_user_response = SignInResponse::from(token);

    Ok(HttpResponse::Ok().json(fetch_user_response))
}

pub async fn assign(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<AssignRequest>,
) -> ApiResponse {
    let conn = state.get_sqls_db_conn()?;
    let user_id = authn::get_user_id_from_header(&req).unwrap();
    let user_repository = UserRepositoryImpl { conn: &conn };
    let patient_repository = PatientRepositoryImpl { conn: &conn };
    let doctor_in_charge_repository = DoctorInChargeRepositoryImpl { conn: &conn };
    let user_usecase = UserUsecase {
        user_repository,
        patient_repository,
        doctor_in_charge_repository,
    };

    user_usecase
        .assign(user_id.clone(), form.patient_code.clone())
        .await?;
    let assign_response = AssignResponse::from();

    Ok(HttpResponse::Ok().json(assign_response))
}
