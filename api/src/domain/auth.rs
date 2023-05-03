use crate::{
    app::auth::{create_jwt, models::NetworkResponse},
    models::User,
    repository::UserRepository,
};

pub struct Auth {
    user_repository: UserRepository,
}

impl Auth {
    pub fn new(user_repository: UserRepository) -> Self {
        Auth { user_repository }
    }

    pub async fn sign_in(
        &self,
        email: String,
        password: String,
    ) -> Result<String, NetworkResponse> {
        let user = self.user_repository.find_by_email(email).await;
        if user.is_none() {
            return Err(NetworkResponse::BadRequest(
                "Invalid credentials".to_string(),
            ));
        }

        let user = user.unwrap();


        if user.password == password {
            return Ok(create_jwt(user.id.unwrap().to_hex()).unwrap());
        } else {
            return Err(NetworkResponse::BadRequest(
                "Invalid credentials".to_string(),
            ));
        }
    }

    pub async fn sign_up(
        &self,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str,
    ) -> Result<String, NetworkResponse> {
        let user = self.user_repository.find_by_email(email.to_string()).await;
        if user.is_some() {
            return Err(NetworkResponse::BadRequest(
                "Email already exists".to_string(),
            ));
        }
        let user = User {
            id: None,
            email: email.to_string(),
            password: password.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password_salt: "".to_string(),
        };

        self.user_repository.create_user(user).await;

        return Ok("".to_string());
    }
}
