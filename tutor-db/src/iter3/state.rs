use sqlx::postgres::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_checker_response: String,
    pub visit_count: Mutex<i32>,
    pub db: PgPool,
}
