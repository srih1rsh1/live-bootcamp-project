use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_auth_ui() {
    let app = TestApp::new().await;

    // Check the response on /verify-2fa
    let response = app.verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
}
