use rocket::{
    serde::json::{self, Json},
    Build, Rocket, response::content,
};
use serde::Serialize;

pub mod home;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
    code: i64,
    message: String,
    data: <&'static str>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> content::RawJson {
        Json({
          "code": 200,
        })
    }
    pub fn error(code: i64, message: &str) -> Json<ApiResponse<T>> {
        Json(ApiResponse {
            code,
            message: String::from(message),
            data,
        })
    }
}

pub fn init() -> Rocket<Build> {
    rocket::build().mount("/api/v1/home", home::routes())
}
