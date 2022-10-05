use serde::Serialize;

use crate::api::*;

#[derive(Serialize)]
struct HomeResp {
    string: i32,
}

#[get("/")]
fn home() -> JsonValue {
    let data = HomeResp { string: 200 };
    ApiResponse::success(json!(&data))
}

#[get("/2")]
fn home_2() -> JsonValue {
    ApiResponse::error(API_ERR_SYSTEM_ERROR, "No")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![home, home_2]
}
