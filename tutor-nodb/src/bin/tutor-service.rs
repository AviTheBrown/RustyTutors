use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;
#[actix_rt::main]

async fn main() -> io::Result<()> {
    // data that will shared across threads
    let shared_data = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });
    // creates a new actix web application
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
