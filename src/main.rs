mod constants;
mod domain;
mod middleware;
mod presentation;
mod repository;
mod route;
mod usecase;
mod utils;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = utils::db::establish_sqlx_connection().await;
    let app_state = utils::state::AppState { sqlx_db: pool };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(route::api)
            .app_data(Data::new(app_state.clone()))
    })
    .bind(("127.0.0.1", 8000))?
    .workers(1)
    .run()
    .await
}
