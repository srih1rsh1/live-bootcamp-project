use crate::app_state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    state: State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let mut banned_token_state = state.banned_token_store.write().await;
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(t) => t,
        None => return (CookieJar::new(), Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token).await {
        Ok(_) => (),
        Err(_error) => return (CookieJar::new(), Err(AuthAPIError::InvalidToken)),
    }

    if let Err(_) = banned_token_state.token_store(token).await {
        return (CookieJar::new(), Err(AuthAPIError::InvalidToken));
    };

    let jar = jar.remove(JWT_COOKIE_NAME);

    (jar, Ok(StatusCode::OK.into_response()))
}
