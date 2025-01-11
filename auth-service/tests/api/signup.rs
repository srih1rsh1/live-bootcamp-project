use crate::helpers::TestApp;

#[tokio::test]
async fn signup_auth_ui() {
    let app = TestApp::new().await;

    // Check the response on /signup
    let response = app.signup().await;
    assert_eq!(response.status().as_u16(), 200);
}
