use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let visit_count = app_state.visit_count.lock().unwrap();
    // the respondse to send to the client.
    let response: String = format!(" {} {}", health_check_response, visit_count);
    *visit_count + 1;
    HttpResponse::Ok().json(&response)
}
// pub async fn course_routes(
//     _app_state: web::Data<AppState>,
//     _params: web::Path<i32>,
// ) -> HttpResponse {
//     HttpResponse::Ok().json("successful")
// }
pub async fn post_new_course(
    _app_state: web::Data<AppState>,
    _params: web::Path<i32>,
) -> HttpResponse {
    HttpResponse::Ok().json("successful")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("succsseful")
}

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<i32>,
) -> HttpResponse {
    HttpResponse::Ok().json("successful")
}
