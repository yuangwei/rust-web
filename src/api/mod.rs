use rocket::{Rocket, Build, response};
use rocket::serde::{Serialize, json::Json};

pub mod home;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiResponse<T> {
  code: i32,
  message: String,
  data: T,
}

pub fn api_success<T>(data: T) -> Json<ApiResponse<T>> {
  Json(ApiResponse {
     code: 200, 
     message: String::from("OK"), 
     data,
  })
}


pub fn init() -> Rocket<Build> {
  rocket::build().mount("/api/v1/home", home::routes())
}
