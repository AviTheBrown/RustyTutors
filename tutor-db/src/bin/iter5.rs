use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;

use routes::*;
use state::AppState;
#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // set db url
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not found in the dotenv file");
    // connect to db url
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // init shared stat for app
    let shared_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // start application
    let app = move || {
        App::new()
            .app_data(shared_state.clone())
            .configure(genral_routes)
            .configure(course_routes)
    };
    println!("server connected and running");
    let host_port = env::var("HOST_PORT").expect("HOST_PORT IS NOT FOUND IN THE .env FILE");
    HttpServer::new(app).bind(&host_port)?.run().await
}
