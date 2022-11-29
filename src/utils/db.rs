use crate::constants::env_key;
use dotenv::dotenv;
use sqlx::{mysql::MySqlConnectOptions, MySqlConnection, MySqlPool};
use std::env;

/// pre: set DATABASE env
/// make sqlx connection pool
pub async fn establish_sqlx_connection() -> MySqlPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("db connection error");
    pool
}
