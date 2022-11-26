mod constants;
mod domain;
mod middleware;
mod presentation;
mod repository;
mod usecase;
mod utils;

use actix_web::middleware::Logger;
use actix_web::web::{get, post, Data};
use actix_web::{web, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = utils::db::establish_sqlx_connection().await;
    let app_state = utils::state::AppState { sqlx_db: pool };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(api)
            .app_data(Data::new(app_state.clone()))
    })
    .bind(("127.0.0.1", 8000))?
    .workers(1)
    .run()
    .await
}

// api route definition
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/patient")
                    .route("", get().to(presentation::patient::fetch_patients))
                    .route("", post().to(presentation::patient::create_patient)),
            )
            .service(
                web::scope("/user")
                    .route("login", post().to(presentation::user::sign_in))
                    .route("", post().to(presentation::user::sign_up)),
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
