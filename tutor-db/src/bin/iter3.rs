use actix_web::dev::Service;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
// use actix_web::web:
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
    dotenv().ok();
    // set db url
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not found in the dotenv file");
    // connect to db url
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // init shared stat for app
    let shared_state: web::Data<AppState> = web::Data::new(AppState {
        health_checker_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // start application
    let app = move || {
        App::new()
            .service(web::resource("/").to(|_req| {
                log::info!("Recieved a request!");
                HttpResponse::Ok().finish()
            }))
            .app_data(shared_state.clone())
            .configure(genral_routes)
            .configure(course_routes)
    };
    println!("server connected and running");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
