use crate::helpers::TestApp;

#[tokio::test]
async fn logout_auth_ui() {
    let app = TestApp::new().await;

    // Check the response on /logout
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
}
