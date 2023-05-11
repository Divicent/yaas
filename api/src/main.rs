#[macro_use]
extern crate rocket;
extern crate rocket_cors;

mod models;
mod repository;

use app::auth::handlers::{sign_in_handler, sign_up_handler};
use domain::auth::Auth;
use dotenvy::dotenv;
use mongodb::Client;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
type HttpMethod = rocket::http::Method;
use std::env;

mod domain {
    pub mod auth;
}

mod app {
    pub mod auth {
        pub mod handlers;
        pub mod jwt;
        pub mod models;
    }
}

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[options("/<_..>")]
fn all_options() {}

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

    let db = client.database("yaas");

    let user_repo = repository::UserRepository::new(&db).await;

    let auth = Auth::new(user_repo);

    rocket::build()
        .manage(auth)
        .mount(
            "/v1",
            routes![ping, sign_in_handler, sign_up_handler, all_options],
        )
        .attach(Cors)
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
