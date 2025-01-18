use auth_service::routes::SignupResponse;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn signup_auth_ui() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    // Check the response on /signup
    let test_cases = [
        serde_json::json!({
            "password": "123444565",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": false
        }),
    ];

    for test_case in test_cases {
        let response = app.signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_rerurn_201_if_valid_input() {
    let app = TestApp::new().await;
    let random_email = "hey@clickup.com".to_string();
    let test_case = serde_json::json!({
        "email": random_email,
        "password": "67hey67",
        "requires2FA": true
    });
    let response = app.signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}
