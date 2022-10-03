use serde::{Deserialize, Serialize};

use crate::api::*;

#[derive(Serialize, Deserialize)]
struct HomeResp {
    string: i32,
}

#[get("/")]
fn home() -> Json<ApiResponse<HomeResp>> {
    let data = HomeResp { string: 200 };
    ApiResponse::error(200, "heelo")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![home]
}
