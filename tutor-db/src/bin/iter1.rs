// connecting to the database.

use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;

#[derive(Debug)]
// creates Course data type
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // introduces the .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
    // connects to the database url provided above,
    let database_pool = PgPool::connect(&database_url).await.unwrap();
    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time
           from ezy_course_c4 where course_id = $1"#,
        1,
    )
    // fetches all the rows from the SQL query -> Result<Vec<course_rows>, sqlx::Error>
    .fetch_all(&database_pool)
    .await
    .unwrap();
    let mut course_list = vec![];
    for course_row in course_rows {
        course_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            // posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
    }
    println!("Course = {:?}", course_list);
    Ok(())
}
