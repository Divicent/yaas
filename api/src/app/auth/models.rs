use rocket::serde::json::serde_json::de;
use rocket::{Responder, response};
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
  #[response(status = 201)]
  Created(String),
  #[response(status = 400)]
  BadRequest(String),
  #[response(status = 401)]
  Unauthorized(String),
  #[response(status = 404)]
  NotFound(String),
  #[response(status = 409)]
  Conflict(String),
}


#[derive(Serialize)]
pub enum ResponseBody {
  Message(String),
  AuthToken(String)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
  pub subject_id: String,
  pub exp: usize,
}

#[derive(Debug)]
pub struct JWT {
  pub claims: Claims
}

pub struct User {
  pub id: String,
  pub user_name: String,
  pub password: String,
}