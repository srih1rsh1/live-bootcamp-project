use crate::{domain::AuthAPIError, utils::auth::validate_token};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;

    match validate_token(&token).await {
        Ok(_) => (),
        Err(error) => return Err(AuthAPIError::InvalidToken),
    }

    Ok(StatusCode::OK.into_response())
}
