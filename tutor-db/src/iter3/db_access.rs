use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_course_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let course_rwos_frm_query = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM
		ezy_course_c4 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    course_rwos_frm_query
        .iter()
        .map(|course_row| Course {
            course_name: course_row.course_name.clone(),
            course_id: course_row.course_id as u32,
            tutor_id: course_row.tutor_id as u32,
            posted_time: course_row.posted_time,
        })
        .collect()
}
