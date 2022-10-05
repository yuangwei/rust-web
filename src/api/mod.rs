use rocket::{Build, Rocket};
use serde::Serialize;
use serde_json::json;
pub use serde_json::Value as JsonValue;

pub mod home;
pub mod user;

pub const API_SUCCESS: i32 = 0;
pub const API_ERR_SYSTEM_ERROR: i32 = 10000; // 未知异常

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse {}

impl ApiResponse {
    pub fn success(data: JsonValue) -> JsonValue {
        json!({
          "code": API_SUCCESS,
          "message": "OK",
          "data": data,
        })
    }
    pub fn error(code: i32, message: &str) -> JsonValue {
        json!({
          "code": code,
          "message": message
        })
    }
}

pub fn init() -> Rocket<Build> {
    rocket::build()
        .mount("/api/v1/home", home::routes())
        .mount("/api/v1/user", user::routes())
}
