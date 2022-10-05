use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::fairing::AdHoc;
use serde::Deserialize;

pub mod user;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MongoConfig {
    url: String,
    database: String,
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        let db_config: MongoConfig = rocket.figment().extract_inner("mongodb").expect("mongodb");
        match connect_mongodb(db_config).await {
            Ok(database) => rocket.manage(database),
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}

async fn connect_mongodb(db_config: MongoConfig) -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse(db_config.url).await?;

    client_options.app_name = Some("falcon".to_string());
    let client = Client::with_options(client_options)?;
    client
        .database(&db_config.database)
        .run_command(doc! {"ping": 1}, None)
        .await?;
    Ok(())
}
