use rocket::serde::{Serialize, json::Json};

use crate::api::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeResp {
  string: String,
}

#[get("/")]
fn home() -> Json<ApiResponse<HomeResp>> {
  api_success(HomeResp { string: String::from("ok") })
}

pub fn routes() -> Vec<rocket::Route> {
  routes![home]
}