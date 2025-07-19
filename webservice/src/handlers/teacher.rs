use actix_web::{web, HttpResponse};
use crate::errors::MyError;
use crate::state::AppState;
use crate::dbaccess::teacher::*;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.into())
    .await
    .map(|_| HttpResponse::Ok().json("Post new teacher successfully."))
}

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_detail(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_detail(
    app_state: web::Data<AppState>,
    update_teacher: web::Json<UpdateTeacher>,
    params: web::Path<i32>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, update_teacher.into())
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
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
    async fn post_new_teacher_success() {
        let app_state = create_app_state().await;

        let teacher = web::Json(CreateTeacher {
            name: "A New Teacher".into(),
            picture_url: "https://i2.hdslb.com/bfs/article/1f8a3ece569b3d61903fe2062cf71d96435d6f8b.jpg@1192w.avif".into(),
            profile: "This is a test profile".into(),
        });

        // 模拟添加教师的请求
        let response = post_new_teacher(teacher, app_state).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_teachers_success() {
        let app_state = create_app_state().await;

        let response = get_all_teachers(app_state).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_detail_success() {
        let app_state = create_app_state().await;

        let params: web::Path<i32> = web::Path::from(1);
        let response = get_teacher_detail(app_state, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_detail_failure() {
        let app_state = create_app_state().await;

        let params: web::Path<i32> = web::Path::from(100);
        let response = get_teacher_detail(app_state, params).await;

        match response {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_teacher_success() {
        let app_state = create_app_state().await;

        let params: web::Path<i32> = web::Path::from(2);
        let update_param = web::Json(UpdateTeacher {
            name: Some("Teacher name changed".into()),
            picture_url: Some("https://i2.hdslb.com/bfs/article/6e82414fa0c96d530caa120caed421240df75cfc.jpg@1192w.avif".into()),
            profile: Some("This is a update profile".into()),
        });
        let response = update_teacher_detail(app_state, update_param, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_teacher_success() {
        let app_state = create_app_state().await;

        let params: web::Path<i32> = web::Path::from(3);
        let response = delete_teacher(app_state, params).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_teacher_failure() {
        let app_state = create_app_state().await;

        let params: web::Path<i32> = web::Path::from(100);
        let response = delete_teacher(app_state, params).await;

        match response {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}