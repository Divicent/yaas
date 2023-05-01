use crate::app::auth::{create_jwt, models::NetworkResponse};

pub fn login_user(email: String, password: String) -> Result<String, NetworkResponse> {
    if email == "rusith@mail.com" && password == "pass" {
        match create_jwt("user_id".to_string()) {
            Ok(token) => Ok(token),
            Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
        }
    } else {
        Err(NetworkResponse::BadRequest(
            "Invalid credentials".to_string(),
        ))
    }
}
