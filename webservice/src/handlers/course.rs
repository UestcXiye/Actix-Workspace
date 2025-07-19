use crate::state::AppState;
use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::models::course::{CreateCourse, UpdateCourse};
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|_| HttpResponse::Ok().json("Post new course successfully."))
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

pub async fn update_course_detail(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use actix_web::ResponseError;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use sqlx::MySqlPool;

    async fn create_db_pool() -> MySqlPool {
        // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL 没有在 .env 文件里设置");

        // 创建数据库连接池
        MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap()
    }

    async fn create_app_state() -> web::Data<AppState> {
        let db_pool = create_db_pool().await;

        web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        })
    }

    #[ignore]
    #[actix_rt::test]
    async fn post_new_course_success() {
        let app_state = create_app_state().await;

        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test course".into(),
            time: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 12).expect("Unknown date"),
                NaiveTime::from_hms_opt(10, 15, 0).expect("Unknown time"),
            )),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });

        // 模拟添加课程的请求
        let response = post_new_course(course, app_state).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state = create_app_state().await;

        let teacher_id: web::Path<i32> = web::Path::from(1);
        let response = get_courses_for_teacher(app_state, teacher_id).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state = create_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let response = get_course_detail(app_state, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        let app_state = create_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let response = get_course_detail(app_state, params).await;

        match response {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        let app_state = create_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((3, 4));
        let update_param = web::Json(UpdateCourse {
            name: Some("Course name changed".into()),
            time: Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2025, 7, 19).expect("Unknown date"),
                NaiveTime::from_hms_opt(10, 15, 0).expect("Unknown time"),
            )),
            description: Some("This is another test course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("Chinese".into()),
            level: Some("Intermediate".into())
        });
        
        let response = update_course_detail(app_state, update_param, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        let app_state = create_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let response = delete_course(app_state, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        let app_state = create_app_state().await;

        let params: web::Path<(i32, i32)> = web::Path::from((1, 101));
        let response = delete_course(app_state, params).await;

        match response {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}