use std::env;
extern crate dotenvy;
use dotenvy::dotenv;

use mongodb::{
  bson::{extjson::de::Error},
  results::{ InsertOneResult},
  Client,
  Collection
};

use crate::models::user_model::User;


pub struct UserRepository {
  collection: Collection<User>
}

impl UserRepository {
   pub async fn init() -> Self {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
      Ok(uri) => uri.to_string(),
      Err(_) => panic!("MONGOURI must be set in .env file")
    };

    let client = match Client::with_uri_str(uri).await {
      Ok(client) => client,
      Err(_) => panic!("Failed to initialize mongodb client")
    };

    let db = client.database("auth");
    let collection = db.collection("users");

    UserRepository { collection }
  }
}