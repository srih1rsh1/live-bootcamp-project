use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;

#[tokio::test]
async fn verify_token_auth_ui() {
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

    let cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let verify_token_request = serde_json::json!({
        "token": cookie.value()
    });

    // Check the response on /verify_token
    let response = app.verify_token(&verify_token_request).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let token = serde_json::json!({
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoYXJzaGFAZXhhbXBsZS5jb20iLCJleHAiOjE3MzkyNTQ5NzF9.5t2Zi16vqX5JhHc2ik05VPO1ZI6mus1steKUguVCGP8"
    });

    let response = app.verify_token(&token).await;
    assert_eq!(response.status().as_u16(), 401);
}
