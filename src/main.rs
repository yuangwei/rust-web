mod api;
mod remote;
mod service;
mod task;
mod model;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let _web = api::init().launch().await;
  Ok(())
}