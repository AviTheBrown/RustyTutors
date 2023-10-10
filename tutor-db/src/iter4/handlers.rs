use super::db::*;
use super::errors::TutorError;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use std::{convert::TryFrom, i32};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_checker_response;
    let visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {}", health_check_response, visit_count);
    let _ = *visit_count + 1;
    HttpResponse::Ok().json(&response)
}
pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let path_tuple = params;
    let tutor_id = i32::try_from(path_tuple.0).unwrap();
    let course_id = i32::try_from(path_tuple.1).unwrap();
    let course_details = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course_details)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, TutorError> {
    let tuple: i32 = params.into_inner();
    let tutor_id: i32 = i32::try_from(tuple).unwrap();

    get_course_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    // use chrono::{NaiveDa}
    use dotenv::dotenv;
    use sqlx::database;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    // get_course_details()
    async fn get_all_details_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND IN .env FILE");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_checker_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let course_details: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp: HttpResponse = get_course_details(app_state, course_details).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    // get_courses_for_tutor()
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT IN .env FILE");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_checker_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.unwrap().status(), StatusCode::OK);
    }

    // #[actix_rt::test]
    // async fn post_course_success() {
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    //     let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
    //     let app_state: web::Data<AppState> = web::Data::new(AppState {
    //         health_checker_response: "".to_string(),
    //         visit_count: Mutex::new(0),
    //         db: pool,
    //     });
    //     let new_course_msg = Course {
    //         course_id: 1,
    //         tutor_id: 1,
    //         course_name: "This is the next course".into(),
    //         posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
    //     };
    //     let course_param = web::Json(new_course_msg);
    //     let resp = post_new_course(course_param, app_state).await;
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }
}
