use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Parse, Password},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

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
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_e) => return (CookieJar::new(), Err(AuthAPIError::InvalidCredentials)),
    };
    let password = match Password::parse(request.password) {
        Ok(passwrod) => passwrod,
        Err(_e) => return (CookieJar::new(), Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.read().await;

    let _validation = match user_store.validate_user(&email, &password).await {
        Ok(ouptput) => ouptput,
        Err(_e) => return (CookieJar::new(), Err(AuthAPIError::IncorrectCredentials)),
    };
    let _get_user = match user_store.get_user(&email).await {
        Ok(info) => info,
        Err(_e) => return (CookieJar::new(), Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = generate_auth_cookie(&email).unwrap();

    let update_jar = jar.add(auth_cookie);
    drop(user_store);

    (
        update_jar,
        Ok((
            StatusCode::OK,
            Json(LoginResponse {
                message: "Login Successfull".to_owned(),
            }),
        )
            .into_response()),
    )
}
