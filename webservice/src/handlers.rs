use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use super::db_access::*;
use super::errors::MyError;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    println!("incoming for health check");

    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[actix_rt::test]
    async fn post_course_test() {
        // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");

        // 创建数据库连接池
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(3),
            time: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 12).expect("Unknown date"),
                NaiveTime::from_hms_opt(10, 15, 0).expect("Unknown time"),
            )),
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        // 模拟添加课程的请求
        let response = new_course(course, app_state).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");

        // 创建数据库连接池
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<i32> = web::Path::from(1);
        let response = get_courses_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");

        // 创建数据库连接池
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let response = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}