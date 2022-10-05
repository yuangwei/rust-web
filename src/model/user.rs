use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    email: String,
    password: String,
    nickname: String,
}

impl User {
    fn conn(db: &Database) -> Collection<Self> {
        db.collection::<Self>("user")
    }
    pub async fn get_userinfo(
        db: &Database,
        email: &String,
    ) -> mongodb::error::Result<Option<Self>> {
        let user = User::conn(&db)
            .find_one(mongodb::bson::doc! {"email":email }, None)
            .await?;
        if user.is_none() {
            Ok(())
        }
        Ok(user)
    }
}
