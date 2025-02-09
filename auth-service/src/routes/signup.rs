use std::clone;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Parse, Password, User, UserStore},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    //let email = request.email.clone();
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, request.requires_2fa).await;

    let mut user_store = state.user_store.write().await;

    #[warn(unused_variables)]
    if let Ok(value) = user_store.get_user(&user.email).await {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    #[warn(unused_variables)]
    if let Err(error) = user_store.add_user(user).await {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}
