use crate::{
    app_state::AppState,
    domain::{AuthAPIError, BannedTokenStore},
    utils::auth::validate_token,
};
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
    let state = state.banned_token_store.read().await;
    match validate_token(&token).await {
        Ok(_) => (),
        Err(_error) => return Err(AuthAPIError::InvalidToken),
    }

    match state.check_token(token).await {
        Some(e) => return Err(AuthAPIError::InvalidToken),
        None => (),
    }

    Ok(StatusCode::OK.into_response())
}
