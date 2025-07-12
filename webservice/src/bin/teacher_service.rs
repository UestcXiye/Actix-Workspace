use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../db_access.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

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
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}