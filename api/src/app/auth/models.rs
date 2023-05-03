use jsonwebtoken::errors::Error;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket::Responder;
use serde::Deserialize;
use serde_json;

use crate::app::auth::jwt::decode_jwt;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 500)]
    InternalServerError(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
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
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(key.to_string())?)
        }

        fn create_response(message: &str) -> Response {
            Response {
                body: ResponseBody::Message(message.to_string()),
            }
        }

        fn json_to_str(response: Response) -> String {
            serde_json::to_string(&response).unwrap()
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = create_response("Error validating JWT token - No token provided");

                Outcome::Failure((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(json_to_str(response)),
                ))
            }
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response =
                            create_response("Error validating JWT token - Expired Token");

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json_to_str(response)),
                        ))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response =
                            create_response("Error validating JWT token - Invalid Token");

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json_to_str(response)),
                        ))
                    }
                    _ => {
                        let response =
                            create_response(&format!("Error validating JWT token - {}", err));

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(json_to_str(response)),
                        ))
                    }
                },
            },
        }
    }
}
