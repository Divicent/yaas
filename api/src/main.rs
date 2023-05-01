mod app;
mod domain;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use app::auth::models::{NetworkResponse, Response, ResponseBody, JWT};
use domain::auth::login_user;
use dotenvy::dotenv;
use mongodb::Client;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde::Deserialize;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[get("/ping")]
fn ping() -> &'static str {
    "Pong"
}

#[get("/nodes")]
fn node_list(key: Result<JWT, NetworkResponse>) -> Result<Json<String>, NetworkResponse> {
    match key {
        Ok(key) => return Ok(Json(key.claims.subject_id)),
        Err(err) => return Err(err),
    }
}

#[post("/auth/sign-in", format = "json", data = "<login_request>")]
fn sign_in_handler(login_request: Json<LoginRequest>) -> Result<Json<Response>, NetworkResponse> {
    let token = match login_user(login_request.email.clone(), login_request.password.clone()) {
        Ok(token) => token,
        Err(err) => return Err(err),
    };

    let response = Response {
        body: ResponseBody::AuthToken(token),
    };

    Ok(Json(response))
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

    let user_repo = repository::UserRepository::new(&db);

    let b = rocket::build();
    b.mount("/v1", routes![ping, node_list, sign_in_handler])
}
