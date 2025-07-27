use actix_web::{http, web, App, HttpServer};
use std::sync::Mutex;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
use actix_cors::Cors;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../errors.rs"]
mod errors;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;
use crate::errors::MyError;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 没有在 .env 文件里设置");

    // 创建数据库连接池
    let db_pool = MySqlPoolOptions::new()
        .connect(&db_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8082/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .wrap(cors)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}