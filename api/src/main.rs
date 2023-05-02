#[macro_use]
extern crate rocket;

mod app;
mod domain;
mod models;
mod repository;

use app::auth::handlers::{sign_in_handler, sign_up_handler};
use domain::auth::Auth;
use dotenvy::dotenv;
use mongodb::Client;
use std::env;

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let uri = match env::var("MONGOURI") {
        Ok(uri) => uri.to_string(),
        Err(_) => panic!("MONGOURI environment variable not set!"),
    };

    let client = match Client::with_uri_str(uri).await {
        Ok(client) => client,
        Err(_) => panic!("Failed to initialize mongodb client"),
    };

    let db = client.database("auth");

    let user_repo = repository::UserRepository::new(&db).await;

    let auth = Auth::new(user_repo);

    rocket::build()
        .manage(auth)
        .mount("/v1", routes![ping, sign_in_handler, sign_up_handler])
}
