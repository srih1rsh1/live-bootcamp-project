use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Parse, Password},
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
    
    let user_store = state.user_store.read().await;

    match user_store.validate_user(&email, &password).await {
        Ok(_) => (),
        Err(e) => return  Err(AuthAPIError::IncorrectCredentials)
    }

    match user_store.get_user(&email).await {
        Ok(_) => (),
        Err(e) => return Err(AuthAPIError::IncorrectCredentials) 
    }
   
   drop(user_store);
    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            message: "Login Successfull".to_owned(),
        })
    ))
}
