use reqwest::Url;

use crate::helpers::{get_random_email, TestApp};

const JTW_COOKIE_NAME: &str = "jwt";

#[tokio::test]
async fn logout_auth_ui() {
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

    // Check the response on /logout
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
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

    // Check the response on /logout
    let response = app.logout().await;
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400, "the JWT Cookie is missing");
}

#[tokio::test]
async fn should_retunr_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JTW_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401, "the JWT Cookie is invalid");
}
