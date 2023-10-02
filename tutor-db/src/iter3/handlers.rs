use super::db::*;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use std::{convert::TryFrom, i32};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.handler_checker_response;
    let visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {}", health_check_response, visit_count);
    let _ = *visit_count + 1;
    HttpResponse::Ok().json(&response)
}
pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into())
        .await
        .unwrap();
    HttpResponse::Ok().json(course)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let path_tuple = params;
    let tutor_id = i32::try_from(path_tuple.0).unwrap();
    let course_id = i32::try_from(path_tuple.1).unwrap();
    let course_details = get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .unwrap();
    HttpResponse::Ok().json(course_details)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tuple: i32 = params.into_inner();
    let tutor_id: i32 = i32::try_from(tuple).unwrap();

    let courses = get_course_for_tutor_db(&app_state.db, tutor_id).await;
    HttpResponse::Ok().json(courses)
}
