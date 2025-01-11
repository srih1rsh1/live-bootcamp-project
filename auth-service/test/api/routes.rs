use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui(){
    let app = TestApp::new.await;

    // Check the response on root /
    let response = app.get_root().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(), "text/html");
    

    // Check the response on /signup
    let response = app.signup().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(), "text/html");

    // Check the response on /login
    let response = app.login().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(),"text/html");

    // Check the response on /logout
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(),"text/html");

    // Check the response on /verify-2fa
    let response = app.verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(),"text/html");

    // Check the response on /verify_token
    let response = app.verify_token().await;
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwarp(),"text/html");
}