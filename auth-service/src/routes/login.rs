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

pub async fn login(state: State<AppState>, Json(request): Json<LoginRequest>) -> impl IntoResponse {
    let email = request.email;
    let password = request.password;
    let message = "Provide proper login information".to_owned();

    let loginresponse = Json(LoginResponse {
        message: "Login Successfull...!".to_owned(),
    });

    if validate_email(email) {
        if !password.is_empty() {
            if password.chars().count() < 8 {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(LoginResponse { message }),
                )
            } else {
                (StatusCode::OK, loginresponse)
            }
        } else {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(LoginResponse { message }),
            )
        }
    } else {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(LoginResponse { message }),
        )
    }
}
