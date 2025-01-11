use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_auth_ui() {
    let app = TestApp::new().await;

    // Check the response on /verify_token
    let response = app.verify_token().await;
    assert_eq!(response.status().as_u16(), 200);
}
