use super::models::Course;
use super::state::AppState;
use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Recieved new course");
    let course_count_for_user = app_state
        // take the AppState instance of course
        .courses
        // locks to take control of the mutex
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        // counts the number of course the user has and counts them
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1) as i32),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    // pushed the new course to the AppState
    app_state.courses.lock().unwrap().push(new_course);
    // json to indicate that the course has been added
    HttpResponse::Ok().json("Added course")
}
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    // gains acces to AppState feilds
    let health_check_response = &app_state.health_check_response;
    // give acces to the mutex<visit_count>
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} loopy times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_id: None,
            course_name: "Hello new course".into(),
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp: HttpResponse = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
