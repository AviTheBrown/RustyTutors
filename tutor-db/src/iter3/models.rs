use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub course_name: String,
    pub course_id: i32,
    pub tutor_id: i32,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            course_name: course.course_name.clone(),
            course_id: course.course_id,
            tutor_id: course.tutor_id,
            posted_time: course.posted_time,
        }
    }
}
