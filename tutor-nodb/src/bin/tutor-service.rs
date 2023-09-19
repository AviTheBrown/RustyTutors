use std::io;
use std::sync::Mutex;
use Actix::web::{web, App, HttpServer};

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;
#[actix_rt::main]

async fn main() -> io::Result<()> {
    let share_data = web::Data::new(AppState {
        health_check_response: "All is good in the hood".to_string(),
        visit_counte: Mutex::new(0),
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_route)
    };
    HttpServer::new(app).bind("127.0.0.1:3000").run().await
}
