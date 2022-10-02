#[get("/")]
fn home() {
  
}

pub fn routes() -> Vec<rocket::Route> {
  routes![home]
}