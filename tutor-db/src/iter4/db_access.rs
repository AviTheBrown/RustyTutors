use super::errors::TutorError;
use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_course_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, TutorError> {
    let course_row_qry = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
        FROM ezy_course_c4 
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    let course: Vec<Course> = course_row_qry
        .iter()
        .map(|course_row| Course {
            course_name: course_row.course_name.clone(),
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect();
    match course.len() {
        0 => Err(TutorError::NotFound("Course not found for tutor".into())),
        _ => Ok(course),
    }
}
pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    let course_row_qry = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time 
         FROM ezy_course_c4
         WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_name: course_row_qry.course_name.clone(),
        course_id: course_row_qry.course_id,
        tutor_id: course_row_qry.tutor_id,
        posted_time: Some(chrono::NaiveDateTime::from(
            course_row_qry.posted_time.unwrap(),
        )),
    }
}
pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course_row_qry = sqlx::query!(
        "INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name) VALUES ($1, $2, $3)  
        RETURNING tutor_id, course_id, course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_name: course_row_qry.course_name.clone(),
        course_id: course_row_qry.course_id,
        tutor_id: course_row_qry.tutor_id,
        posted_time: Some(chrono::NaiveDateTime::from(
            course_row_qry.posted_time.unwrap(),
        )),
    }
}
