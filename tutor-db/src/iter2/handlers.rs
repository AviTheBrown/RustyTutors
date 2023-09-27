use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let visit_count = app_state.visit_count.lock().unwrap();
    // the respondse to send to the client.
    let response: String = format!(" {} {}", health_check_response, visit_count);
    let _ = *visit_count + 1;
    HttpResponse::Ok().json(&response)
}

pub async fn post_new_course(
    _app_state: web::Data<AppState>,
    _new_course: web::Json<Course>,
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

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::{NaiveDate, NaiveDateTime};
    // use chrono::{NaiveDa}
    use dotenv::dotenv;
    use sqlx::database;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    //  post_new_course()
    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL WAS NOT FOUND IN .env FILE");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let expected_course: Course = Course {
            course_id: 1,
            course_name: "Elixir Time".to_string(),
            tutor_id: 1,
            posted_time: Some(NaiveDate::from_ymd(2023, 9, 27).and_hms(2, 30, 11)),
        };
        let course_info = web::Json(expected_course);
        let resp = post_new_course(app_state, course_info).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    // get_course_details()
    async fn get_all_details_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND IN .env FILE");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
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
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
