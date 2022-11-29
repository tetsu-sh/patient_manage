use crate::presentation;
use actix_web::web;
use actix_web::web::{get, post};

// api route definition
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/patient")
                    .route("", get().to(presentation::patient::fetch_patients))
                    .route("", post().to(presentation::patient::create_patient))
                    .route(
                        "with_me",
                        post().to(presentation::patient::create_patient_with_medical_examination),
                    ),
            )
            .service(
                web::scope("/user")
                    .route("login", post().to(presentation::user::sign_in))
                    .route("", post().to(presentation::user::sign_up))
                    .route("assign", post().to(presentation::user::assign)),
            )
            .service(
                web::scope("/medical_examination")
                    .route(
                        "",
                        get().to(presentation::medical_examination::fetch_medical_examinations),
                    )
                    .route(
                        "",
                        post().to(presentation::medical_examination::create_medical_examination),
                    ),
            )
            .service(
                web::scope("/healthcheck").route("", get().to(presentation::healthcheck::index)),
            ),
    );
}
