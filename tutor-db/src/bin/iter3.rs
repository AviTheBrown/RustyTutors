use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::database;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter3/handlers.rs"]
mod handlers;

#[path = "../iter3/routes.rs"]
mod routes;

#[path = "../iter3/models.rs"]
mod models;

#[path = "../iter3/db_access.rs"]
mod db;

#[path = "../iter3/state.rs"]
mod state;

use routes::*;
use state::AppState;
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // fkdkfjk
}
