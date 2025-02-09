use axum::{
    extract::State,
    http::{response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::validate_email;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Parse, Password, User},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(PartialEq, Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    state: State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            message: "Login Successfull".to_owned(),
        }),
    ))
}
