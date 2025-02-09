use crate::helpers::TestApp;

#[tokio::test]
async fn login_auth_ui() {
    let _app = TestApp::new().await;
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

    assert_eq!( response.status().as_u16(), 401 , "Please provide Correct Credentials")
}
