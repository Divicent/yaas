use mongodb::{bson::doc, Collection, Database};

use crate::models::User;

pub struct UserRepository(Collection<User>);

impl UserRepository {
    pub async fn new(db: &Database) -> Self {
        let collection = db.collection::<User>("users");
        UserRepository(collection)
    }

    pub async fn find_by_email(&self, email: String) -> Option<User> {
        self.0
            .find_one(doc! { "email": email }, None)
            .await
            .expect("Faile to find user by email")
    }

    pub async fn create_user(&self, user: User) {
        self.0
            .insert_one(user, None)
            .await
            .expect("Unable to create user");
    }
}
