use crate::utils;
use crate::utils::errors::MyError;
use sqlx::MySqlPool;

/// to transfer state to actix web
#[derive(Clone)]
pub struct AppState {
    pub sqlx_db: sqlx::Pool<sqlx::MySql>,
}

impl AppState {
    pub fn get_sqls_db_conn(&self) -> Result<MySqlPool, MyError> {
        let conn = self.sqlx_db.clone();
        Ok(conn)
    }
}
