#[path = "../mod.rs"]
mod webapp;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use webapp::{errors, handlers, models, routers};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 检测并读取 .env 文件中的内容，若不存在也会跳过异常
    dotenv().ok();

    let host_port = env::var("HOST_PORT")
        .expect("HOST_PORT is not set in .env file");
    println!("Listening on {}", &host_port);

    let app = move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    };

    HttpServer::new(app).bind(&host_port)?.run().await
}