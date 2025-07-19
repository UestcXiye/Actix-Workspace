use serde::{Deserialize, Serialize};

/// 教师信息，用于应用注册
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

/// 教师信息，用于数据库查询
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}