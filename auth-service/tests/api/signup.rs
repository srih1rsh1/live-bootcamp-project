use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_auth_ui() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // Check the response on /signup
    let test_cases = [
        serde_json::json!({
            "password": "123444565",
            "required2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "required2FA": false
        })
    ];

   for test_case in test_cases {

    let response = app.signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}",test_case);
   }
    
}
