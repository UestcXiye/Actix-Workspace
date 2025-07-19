use sqlx::MySqlPool;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn post_new_teacher_db(pool: &MySqlPool, new_teacher: CreateTeacher) -> Result<(), MyError> {
    let _insert_query = sqlx::query!(
        "INSERT INTO teacher (name, picture_url, profile)
            VALUES (?, ?, ?)",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile,
    )
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query!(
        "DELETE FROM teacher
            WHERE id = ?",
        teacher_id,
    )
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()))?;

    Ok(format!("Deleted {:?} record", row))
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher
) -> Result<String, MyError> {
    let current_teacher_row: Teacher = sqlx::query_as(
        "SELECT id, name, picture_url, profile
                FROM teacher
                WHERE id = ?"
    )
        .bind(teacher_id)
        .fetch_one(pool) // 获取单条记录
        .await
        .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))?;

    let teacher = Teacher {
        id: current_teacher_row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            current_teacher_row.name
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            current_teacher_row.picture_url
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            current_teacher_row.profile
        },
    };

    let row = sqlx::query!(
        "UPDATE teacher
            SET name = ?, picture_url = ?, profile = ?
            WHERE id = ?",
        teacher.name,
        teacher.picture_url,
        teacher.profile,
        teacher_id,
    )
        .execute(pool)
        .await?;

    Ok(format!("Update {:?} record", row))
}
pub async fn get_all_teachers_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows: Vec<Teacher> = sqlx::query_as(
        "SELECT id, name, picture_url, profile
                FROM teacher"
    )
        .fetch_all(pool) // 获取所有记录
        .await?;

    match rows.len() {
        0 => Err(MyError::NotFound("Teacher not found".into())),
        _ => Ok(rows),
    }
}

pub async fn get_teacher_details_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query_as(
        "SELECT id, name, picture_url, profile
                FROM teacher
                WHERE id = ?"
    )
        .bind(teacher_id)
        .fetch_one(pool) // 获取单条记录
        .await
        .map(|teacher: Teacher| Teacher {
            id: teacher.id,
            name: teacher.name,
            picture_url: teacher.picture_url,
            profile: teacher.profile,
        })
        .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))?;

    Ok(row)
}