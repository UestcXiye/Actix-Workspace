use super::models::*;
use sqlx::MySqlPool;
use super::errors::MyError;

pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as(
        "SELECT id, teacher_id, name, time
            FROM course
            WHERE teacher_id = ?"
    )
        .bind(teacher_id)
        .fetch_all(pool) // 获取所有记录
        .await?;

    let courses: Vec<Course> = rows.iter()
        .map(|r| Course {
            id: Some(r.id.expect("Unknown")),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: Some(chrono::NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    match courses.len() {
        0 => Err(MyError::NotFound("Course not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row: Result<Course, sqlx::Error> = sqlx::query_as(
        "SELECT id, teacher_id, name, time
            FROM course
            WHERE teacher_id = ? and id = ?"
    )
        .bind(teacher_id)
        .bind(course_id)
        .fetch_one(pool) // 获取单条记录
        .await;

    if let Ok(row) = row {
        Ok(Course {
            id: Some(row.id.expect("Unknown")),
            teacher_id: row.teacher_id,
            name: row.name.clone(),
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
        })
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}

pub async fn post_new_course_db(pool: &MySqlPool, new_course: Course) -> Result<(), MyError> {
    let _insert_query = sqlx::query!(
        "INSERT INTO course (id, teacher_id, name, time)
            VALUES (?, ?, ?, ?)",
        new_course.id.unwrap(),
        new_course.teacher_id,
        new_course.name,
        new_course.time
    )
        .execute(pool)
        .await?;

    Ok(())
}