pub mod models;
use jsonwebtoken::{errors::{Error, ErrorKind}, Header, Algorithm, encode, EncodingKey, decode, DecodingKey, Validation};
use chrono::Utc;
use models::Claims;
use std::env;

 fn get_secret() -> String {
  env::var("JWT_SECRET").expect("JWT_SECRET must be set")
 }

pub fn create_jwt(id: String) -> Result<String, Error> {
  let secret = get_secret();
  let expiration = Utc::now()
    .checked_add_signed(chrono::Duration::seconds(60))
    .expect("valid timestamp")
    .timestamp();

  let claims = Claims {
    subject_id: id,
    exp: expiration as usize,
  };

  let header = Header::new(Algorithm::HS512);
  encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
  let secret = get_secret();
  let token = token.trim_start_matches("Bearer").trim();

  match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS512)) {
    Ok(token) => Ok(token.claims),
    Err(err) => Err(err.kind().to_owned())
  }
}