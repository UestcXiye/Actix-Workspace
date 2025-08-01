use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_teachers(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    // 创建 HTTP 客户端
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("http://localhost:3000/teachers/")
        .send().await.unwrap()
        .json::<Vec<TeacherResponse>>().await.unwrap();

    // 创建一个上下文，可以向 HTML 模板里添加数据
    let mut ctx = tera::Context::new();
    // 向上下文中插入数据
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    // s 是渲染的模板，静态部分是 teachers.html，动态数据是 ctx
    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(
    tmpl: web::Data<tera::Tera>
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_picture_url", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;

    if params.name == "Dave" {
        ctx.insert("error", "Dave already exists!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_picture_url", &params.picture_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|err| MyError::TeraError(err.to_string()))?;
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.picture_url,
            "profile": &params.profile,
        });

        let awc_client = awc::Client::default();

        let res = awc_client
            .post("http://localhost:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;

        let teacher_response: String =
            serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = format!("Message from Web Server: {}", teacher_response);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}