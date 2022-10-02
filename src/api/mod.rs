use rocket::{Rocket, Build};

pub mod home;


pub fn init() -> Rocket<Build> {
  rocket::build().mount("/api/v1/home", home::routes())
}
