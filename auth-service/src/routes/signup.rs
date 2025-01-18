use crate::{app_state::AppState, domain::{AuthAPIError, User}};
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
    let email = request.email.clone();
    if request.email.is_empty() & request.email.contains("@") {
            return Err(AuthAPIError::InvalidCredentials)
    }
    if request.password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials)
    }


    let user = User::new(request.email, request.password, request.requires_2fa).await;

    let mut user_store = state.user_store.write().await;

    if let Ok(value) = user_store.get_user(email.as_str()).await {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if let Err(error) = user_store.add_user(user).await {
        return  Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}
