use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(
    state: State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;
    match validate_token(&token).await {
        Ok(_) => (),
        Err(_error) => return Err(AuthAPIError::InvalidToken),
    }
    let state = state.banned_token_store.read().await;
    match state.check_token(token).await {
        Some(_e) => return Err(AuthAPIError::InvalidToken),
        None => (),
    }
    drop(state);
    Ok(StatusCode::OK.into_response())
}
