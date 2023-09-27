use actix_web::HttpResponse;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::database;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter2/handlers.rs"]
mod handlers;

#[path = "../iter2/routes.rs"]
mod routes;

#[path = "../iter2/models.rs"]
mod models;

#[path = "../iter2/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_web::main]

async fn main() -> io::Result<()> {
    let server_addr: &str = "127.0.0.1:3000";
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL was not located in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "All is good you asked me before".into(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    println!("Server connected");
    HttpServer::new(app).bind(server_addr)?.run().await
}
