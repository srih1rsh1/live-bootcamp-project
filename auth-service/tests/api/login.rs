use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn login_auth_ui() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_creds = serde_json::json!({
        "email": random_email,
        "password": "12345678999",
        "requires2FA": false
    });

    let response = app.signup(&signup_creds).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_creds = serde_json::json!({
        "email": random_email,
        "password": "12345678999"
    });

    let response = app.login(&login_creds).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let login_creds = serde_json::json!({
        "email": "hellowork@gmail.com",
    });
    let response = app.login(&login_creds).await;

    assert_eq!(
        response.status().as_u16(),
        422,
        "Please provide all the required infomation for Login properly"
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let login_creds = serde_json::json!({
        "email": "hellowork@gmail.com",
        "password": "1234567"
    });

    let response = app.login(&login_creds).await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "Please provide valid Creds"
    )
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let login_creds = serde_json::json!({
        "email": "hellowork@gmail.com",
        "password": "12345678"
    });

    let response = app.login(&login_creds).await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "Please provide Correct Credentials"
    )
}
