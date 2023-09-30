use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.handler_checker_response;
    let visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {}", health_check_response, visit_count);
    let _ = *visit_count + 1;
    HttpResponse::Ok().json(&response)
}
pub async fn post_new_course(
    _course: web::Json<Course>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
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
