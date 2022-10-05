mod api;
mod model;
mod remote;
mod service;
mod task;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = api::init().attach(model::init().await).launch().await;
    Ok(())
}
