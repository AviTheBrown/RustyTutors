use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub course_id: u32,
    pub course_name: String,
    pub tutor_id: u32,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            tutor_id: course.tutor_id,
            posted_time: course.posted_time,
        }
    }
}
