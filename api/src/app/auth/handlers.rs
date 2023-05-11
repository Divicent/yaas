use crate::app::auth::models::NetworkResponse;
use crate::domain::auth::Auth;
use rocket::serde::json::Json;
use rocket::{serde::Deserialize, serde::Serialize, State};

use super::models::SignInResponse;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,

    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
}

#[post("/auth/sign-in", format = "json", data = "<login_request>")]
pub async fn sign_in_handler(
    auth: &State<Auth>,
    login_request: Json<LoginRequest>,
) -> Result<Json<SignInResponse>, NetworkResponse> {
    let token = match auth
        .sign_in(login_request.email.clone(), login_request.password.clone())
        .await
    {
        Ok(token) => token,
        Err(err) => return Err(err),
    };

    let response = SignInResponse { auth_token: token };

    Ok(Json(response))
}

#[post("/auth/sign-up", format = "json", data = "<sign_up_request>")]
pub async fn sign_up_handler(
    auth: &State<Auth>,
    sign_up_request: Json<SignUpRequest>,
) -> Result<Json<bool>, NetworkResponse> {
    match auth
        .sign_up(
            &sign_up_request.email.to_string(),
            &sign_up_request.password.to_string(),
            &sign_up_request.first_name.to_string(),
            &sign_up_request.last_name.to_string(),
        )
        .await
    {
        Ok(_) => Ok(Json(true)),
        Err(err) => Err(err),
    }
}
