/**
 * User manage
 */

#[post("/login")]
fn login() {}

#[get("/userinfo")]
fn user_info() {}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, user_info]
}
