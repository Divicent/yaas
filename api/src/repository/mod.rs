use mongodb::{Collection, Database};

use crate::models::User;

pub struct UserRepository(Collection<User>);

impl UserRepository {
    pub async fn new(db: &Database) -> Self {
        let collection = db.collection::<User>("users");
        UserRepository(collection)
    }
}
