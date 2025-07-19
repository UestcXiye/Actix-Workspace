use chrono::NaiveDateTime;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::errors::MyError;
use sqlx::MySqlPool;

pub async fn post_new_course_db(pool: &MySqlPool, new_course: CreateCourse) -> Result<(), MyError> {
    let _insert_query = sqlx::query!(
        "INSERT INTO course (teacher_id, name, time, description, format, structure, duration, price, language, level)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        new_course.teacher_id,
        new_course.name,
        new_course.time,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level
    )
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_course_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<String, MyError> {
    let row = sqlx::query!(
        "DELETE FROM course
            WHERE teacher_id = ? AND id = ?",
        teacher_id,
        course_id
    )
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:?} record", row))
}

pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<String, MyError> {
    let current_course_row: Course = sqlx::query_as(
        "SELECT * FROM course
            WHERE teacher_id = ? and id = ?"
    )
        .bind(teacher_id)
        .bind(course_id)
        .fetch_one(pool) // 获取单条记录
        .await
        .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let time: NaiveDateTime = if let Some(time) = update_course.time {
        time
    } else {
        current_course_row
            .time
            .unwrap_or_default()
    };
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row
            .description
            .unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row
            .format
            .unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row
            .structure
            .unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row
            .duration
            .unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row
            .level
            .unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row
            .language
            .unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row
            .price
            .unwrap_or_default()
    };

    let row = sqlx::query!(
        "UPDATE course
            SET name = ?, time = ?, description = ?, format = ?, structure = ?, duration = ?, price = ?, language = ?, level = ?
            WHERE teacher_id = ? AND id = ?",
        name,
        time,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        course_id
    )
        .execute(pool)
        .await?;

    Ok(format!("Update {:?} record", row))
}

pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as(
        "SELECT * FROM course
                WHERE teacher_id = ?"
    )
        .bind(teacher_id)
        .fetch_all(pool) // 获取所有记录
        .await?;

    Ok(rows)
}

pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
    let row = sqlx::query_as(
        "SELECT * FROM course
            WHERE teacher_id = ? and id = ?"
    )
        .bind(teacher_id)
        .bind(course_id)
        .fetch_optional(pool) // 获取单条记录
        .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}